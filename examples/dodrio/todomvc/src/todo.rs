//! Type definition and `dodrio::Render` implementation for a single todo item.

use crate::keys;
use dodrio::{Node, Render, RenderContext, RootRender, VdomWeak};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use typed_html::dodrio;
use wasm_bindgen::{prelude::*, JsCast};

/// A single todo item.
#[derive(Serialize, Deserialize)]
pub struct Todo<C> {
    id: usize,
    title: String,
    completed: bool,

    #[serde(skip)]
    edits: Option<String>,

    #[serde(skip)]
    _controller: PhantomData<C>,
}

/// Actions on a single todo item that can be triggered from the UI.
pub trait TodoActions {
    /// Toggle the completion state of the todo item with the given id.
    fn toggle_completed(root: &mut dyn RootRender, vdom: VdomWeak, id: usize);

    /// Delete the todo item with the given id.
    fn delete(root: &mut dyn RootRender, vdom: VdomWeak, id: usize);

    /// Begin editing the todo item with the given id.
    fn begin_editing(root: &mut dyn RootRender, vdom: VdomWeak, id: usize);

    /// Update the edits for the todo with the given id.
    fn update_edits(root: &mut dyn RootRender, vdom: VdomWeak, id: usize, edits: String);

    /// Finish editing the todo with the given id.
    fn finish_edits(root: &mut dyn RootRender, vdom: VdomWeak, id: usize);

    /// Cancel editing the todo with the given id.
    fn cancel_edits(root: &mut dyn RootRender, vdom: VdomWeak, id: usize);
}

impl<C> Todo<C> {
    /// Construct a new `Todo` with the given identifier and title.
    pub fn new<S: Into<String>>(id: usize, title: S) -> Self {
        let title = title.into();
        let completed = false;
        let edits = None;
        Todo {
            id,
            title,
            completed,
            edits,
            _controller: PhantomData,
        }
    }

    /// Set this todo item's id.
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    /// Is this `Todo` complete?
    pub fn is_complete(&self) -> bool {
        self.completed
    }

    /// Mark the `Todo` as complete or not.
    pub fn set_complete(&mut self, to: bool) {
        self.completed = to;
    }

    /// Get this todo's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set this todo item's title.
    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.title = title.into();
    }

    /// Set the edits for this todo.
    pub fn set_edits<S: Into<String>>(&mut self, edits: Option<S>) {
        self.edits = edits.map(Into::into);
    }

    /// Take this todo's edits, leaving `None` in their place.
    pub fn take_edits(&mut self) -> Option<String> {
        self.edits.take()
    }
}

impl<'a, C: TodoActions> Render<'a> for Todo<C> {
    fn render(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        use dodrio::{builder::text, bumpalo};
        use typed_html::types::ClassList;

        let id = self.id;
        let title = self.edits.as_ref().unwrap_or(&self.title);
        let title = bumpalo::collections::String::from_str_in(title, cx.bump).into_bump_str();

        let bump = cx.bump;
        dodrio!(bump,
            <li class={
                let mut class = ClassList::new();
                if self.completed {
                    class.add("completed");
                }
                if self.edits.is_some() {
                    class.add("editing");
                }
                class
            }>
                <div class="view">
                    <input class="toggle" type="checkbox" checked={self.completed}
                           onclick={move |root, vdom, _event| {
                               C::toggle_completed(root, vdom, id)
                           }}
                    />
                    <label ondblclick={move |root, vdom, _event| {
                        C::begin_editing(root, vdom, id)
                    }}>
                        { bumpalo::vec![in &bump; text(title)] }
                    </label>
                    <button class="destroy" onclick={move |root, vdom, _event| {
                        C::delete(root, vdom, id)
                    }}/>
                </div>
                <input class="edit" value={title} name="title" id={
                    bumpalo::format!(in bump, "todo-{}", id).into_bump_str()
                } oninput={move |root, vdom, event| {
                    let input = event
                        .target()
                        .unwrap_throw()
                        .unchecked_into::<web_sys::HtmlInputElement>();
                    C::update_edits(root, vdom, id, input.value());
                }} onblur={move |root, vdom, _event| {
                    C::finish_edits(root, vdom, id)
                }} onkeydown={move |root, vdom, event| {
                    let event = event.unchecked_into::<web_sys::KeyboardEvent>();
                    match event.key_code() {
                        keys::ENTER => C::finish_edits(root, vdom, id),
                        keys::ESCAPE => C::cancel_edits(root, vdom, id),
                        _ => {}
                    }
                }}/>
            </li>
        )
    }
}
