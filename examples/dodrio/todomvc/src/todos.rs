//! Type definitions and `dodrio::Render` implementation for a collection of
//! todo items.

use crate::controller::Controller;
use crate::todo::{Todo, TodoActions};
use crate::visibility::Visibility;
use crate::{keys, utils};
use dodrio::RenderContext;
use dodrio::{
    builder::text,
    bumpalo::{self},
    Node, Render, RootRender, VdomWeak,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::mem;
use typed_html::dodrio;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// A collection of todos.
#[derive(Default, Serialize, Deserialize)]
#[serde(rename = "todos-dodrio", bound = "")]
pub struct Todos<C = Controller> {
    todos: Vec<Todo<C>>,

    #[serde(skip)]
    draft: String,

    #[serde(skip)]
    visibility: Visibility,

    #[serde(skip)]
    _controller: PhantomData<C>,
}

/// Actions for `Todos` that can be triggered by UI interactions.
pub trait TodosActions: TodoActions {
    /// Toggle the completion state of all todo items.
    fn toggle_all(root: &mut dyn RootRender, vdom: VdomWeak);

    /// Update the draft todo item's text.
    fn update_draft(root: &mut dyn RootRender, vdom: VdomWeak, draft: String);

    /// Finish the current draft todo item and add it to the collection of
    /// todos.
    fn finish_draft(root: &mut dyn RootRender, vdom: VdomWeak);

    /// Change the todo item visibility filtering to the given `Visibility`.
    fn change_visibility(root: &mut dyn RootRender, vdom: VdomWeak, vis: Visibility);

    /// Delete all completed todo items.
    fn delete_completed(root: &mut dyn RootRender, vdom: VdomWeak);
}

impl<C> Todos<C> {
    /// Construct a new todos set.
    ///
    /// If an existing set is available in local storage, then us that,
    /// otherwise create a new set.
    pub fn new() -> Self
    where
        C: Default,
    {
        Self::from_local_storage().unwrap_or_default()
    }

    /// Deserialize a set of todos from local storage.
    pub fn from_local_storage() -> Option<Self> {
        utils::local_storage()
            .get("todomvc-dodrio")
            .ok()
            .and_then(|opt| opt)
            .and_then(|json| serde_json::from_str(&json).ok())
    }

    /// Serialize this set of todos to local storage.
    pub fn save_to_local_storage(&self) {
        let serialized = serde_json::to_string(self).unwrap_throw();
        utils::local_storage()
            .set("todomvc-dodrio", &serialized)
            .unwrap_throw();
    }

    /// Add a new todo item to this collection.
    pub fn add_todo(&mut self, todo: Todo<C>) {
        self.todos.push(todo);
    }

    /// Delete the todo with the given id.
    pub fn delete_todo(&mut self, id: usize) {
        self.todos.remove(id);
        self.fix_ids();
    }

    /// Delete all completed todo items.
    pub fn delete_completed(&mut self) {
        self.todos.retain(|t| !t.is_complete());
        self.fix_ids();
    }

    // Fix all todo identifiers so that they match their index once again.
    fn fix_ids(&mut self) {
        for (id, todo) in self.todos.iter_mut().enumerate() {
            todo.set_id(id);
        }
    }

    /// Get a shared slice of the underlying set of todo items.
    pub fn todos(&self) -> &[Todo<C>] {
        &self.todos
    }

    /// Get an exclusive slice of the underlying set of todo items.
    pub fn todos_mut(&mut self) -> &mut [Todo<C>] {
        &mut self.todos
    }

    /// Set the draft todo item text.
    pub fn set_draft<S: Into<String>>(&mut self, draft: S) {
        self.draft = draft.into();
    }

    /// Take the current draft text and replace it with an empty string.
    pub fn take_draft(&mut self) -> String {
        mem::replace(&mut self.draft, String::new())
    }

    /// Get the current visibility for these todos.
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Set the visibility for these todoS.
    pub fn set_visibility(&mut self, vis: Visibility) {
        self.visibility = vis;
    }
}

/// Rendering helpers.
impl<C: TodosActions> Todos<C> {
    fn header<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let bump = cx.bump;
        dodrio!(bump,
            <header class="header">
                <h1>"todos"</h1>
                <input oninput={|root, vdom, event| {
                    let input = event
                        .target()
                        .unwrap_throw()
                        .unchecked_into::<web_sys::HtmlInputElement>();
                    C::update_draft(root, vdom, input.value());
                }} onkeydown={|root, vdom, event| {
                    let event = event.unchecked_into::<web_sys::KeyboardEvent>();
                    if event.key_code() == keys::ENTER {
                        C::finish_draft(root, vdom);
                    }
                }} class="new-todo" placeholder="What needs to be done?" autofocus=true value={self.draft.as_str()}/>
            </header>
        )
    }

    fn todos_list<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        use dodrio::bumpalo::collections::Vec;

        let mut todos = Vec::with_capacity_in(self.todos.len(), cx.bump);
        todos.extend(
            self.todos
                .iter()
                .filter(|t| match self.visibility {
                    Visibility::All => true,
                    Visibility::Active => !t.is_complete(),
                    Visibility::Completed => t.is_complete(),
                })
                .map(|t| t.render(cx)),
        );

        let bump = cx.bump;
        dodrio!(bump,
            <section class="main" style={
                if self.todos.is_empty() {
                    "visibility: hidden"
                } else {
                    "visibility: visible"
                }
            }>
                <input
                    class="toggle-all" id="toggle-all" type="checkbox" name="toggle"
                    checked={self.todos.iter().all(|t| t.is_complete())}
                    onclick={|root, vdom, _event| C::toggle_all(root, vdom)}
                />
                <label for="toggle-all">"Mark as complete"</label>
                <ul class="todo-list">
                    { todos }
                </ul>
            </section>
        )
    }

    fn footer<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let completed_count = self.todos.iter().filter(|t| t.is_complete()).count();
        let incomplete_count = self.todos.len() - completed_count;
        let items_left = if incomplete_count == 1 {
            " item left"
        } else {
            " items left"
        };
        let incomplete_count = bumpalo::format!(in cx.bump, "{}", incomplete_count);

        let clear_completed_text = bumpalo::format!(
            in cx.bump,
            "Clear completed ({})",
            self.todos.iter().filter(|t| t.is_complete()).count()
        );

        let bump = cx.bump;
        dodrio!(bump,
            <footer class="footer" hidden={self.todos.is_empty()}>
                <span class="todo-count">
                    <strong>{
                        bumpalo::vec![in &bump; text(incomplete_count.into_bump_str())]
                    }</strong>
                    { bumpalo::vec![in &bump; text(items_left)] }
                </span>
                <ul class="filters">
                    { bumpalo::vec![in &bump;
                        self.visibility_swap(cx, "#/", Visibility::All),
                        self.visibility_swap(cx, "#/active", Visibility::Active),
                        self.visibility_swap(cx, "#/completed", Visibility::Completed)
                    ] }
                </ul>
                <button class="clear-completed" hidden={completed_count == 0} onclick={|root, vdom, _event| {
                    C::delete_completed(root, vdom);
                }}>{ bumpalo::vec![in &bump; text(clear_completed_text.into_bump_str())] }</button>
            </footer>
        )
    }

    fn visibility_swap<'a>(
        &self,
        cx: &mut RenderContext<'a>,
        url: &'static str,
        target_vis: Visibility,
    ) -> Node<'a> {
        let bump = cx.bump;
        dodrio!(bump,
            <li onclick={move |root, vdom, _event| {
                C::change_visibility(root, vdom, target_vis);
            }}>
                <a href={url} class={
                    if self.visibility == target_vis {
                        "selected"
                    } else {
                        ""
                    }
                }>{ bumpalo::vec![in &bump; text(target_vis.label())] }</a>
            </li>
        )
    }
}

impl<'a, C: TodosActions> Render<'a> for Todos<C> {
    fn render(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let bump = cx.bump;
        dodrio!(bump,
            <div>{ bumpalo::vec![in &bump;
                self.header(cx), self.todos_list(cx), self.footer(cx)
            ] }</div>
        )
    }
}
