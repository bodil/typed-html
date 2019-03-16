//! The controller handles UI events, translates them into updates on the model,
//! and schedules re-renders.

use crate::todo::{Todo, TodoActions};
use crate::todos::{Todos, TodosActions};
use crate::visibility::Visibility;
use dodrio::{RootRender, VdomWeak};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// The controller for the TodoMVC app.
///
/// This `Controller` struct is never actually instantiated. It is only used for
/// its `*Actions` trait implementations, none of which take a `self` parameter.
///
/// One could imagine alternative controller implementations with `*Actions`
/// trait implementations for (e.g.) testing that will assert various expected
/// action methods are called after rendering todo items and sending DOM events.
#[derive(Default, Deserialize, Serialize)]
pub struct Controller;

impl TodosActions for Controller {
    fn toggle_all(root: &mut dyn RootRender, vdom: VdomWeak) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        let all_complete = todos.todos().iter().all(|t| t.is_complete());
        for t in todos.todos_mut() {
            t.set_complete(!all_complete);
        }
    }

    fn update_draft(root: &mut dyn RootRender, vdom: VdomWeak, draft: String) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        todos.set_draft(draft);
    }

    fn finish_draft(root: &mut dyn RootRender, vdom: VdomWeak) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        let title = todos.take_draft();
        let title = title.trim();
        if !title.is_empty() {
            let id = todos.todos().len();
            let new = Todo::new(id, title);
            todos.add_todo(new);
        }
    }

    fn change_visibility(root: &mut dyn RootRender, vdom: VdomWeak, vis: Visibility) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        todos.set_visibility(vis);
    }

    fn delete_completed(root: &mut dyn RootRender, vdom: VdomWeak) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        todos.delete_completed();
    }
}

impl TodoActions for Controller {
    fn toggle_completed(root: &mut dyn RootRender, vdom: VdomWeak, id: usize) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        let t = &mut todos.todos_mut()[id];
        let completed = t.is_complete();
        t.set_complete(!completed);
    }

    fn delete(root: &mut dyn RootRender, vdom: VdomWeak, id: usize) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        todos.delete_todo(id);
    }

    fn begin_editing(root: &mut dyn RootRender, vdom: VdomWeak, id: usize) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        let t = &mut todos.todos_mut()[id];
        let desc = t.title().to_string();
        t.set_edits(Some(desc));
    }

    fn update_edits(root: &mut dyn RootRender, vdom: VdomWeak, id: usize, edits: String) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        let t = &mut todos.todos_mut()[id];
        t.set_edits(Some(edits));
    }

    fn finish_edits(root: &mut dyn RootRender, vdom: VdomWeak, id: usize) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        let t = &mut todos.todos_mut()[id];
        if let Some(edits) = t.take_edits() {
            let edits = edits.trim();
            if edits.is_empty() {
                todos.delete_todo(id);
            } else {
                t.set_title(edits);
            }
        }
    }

    fn cancel_edits(root: &mut dyn RootRender, vdom: VdomWeak, id: usize) {
        let mut todos = AutoCommitTodos::new(root, vdom);
        let t = &mut todos.todos_mut()[id];
        let _ = t.take_edits();
    }
}

/// An RAII type that dereferences to a `Todos` and once it is dropped, saves
/// the (presumably just modified) todos to local storage, and schedules a new
/// `dodrio` render.
pub struct AutoCommitTodos<'a> {
    todos: &'a mut Todos,
    vdom: VdomWeak,
}

impl AutoCommitTodos<'_> {
    /// Construct a new `AutoCommitTodos` from the root rendering component and
    /// `vdom` handle.
    pub fn new(root: &mut dyn RootRender, vdom: VdomWeak) -> AutoCommitTodos {
        let todos = root.unwrap_mut::<Todos>();
        AutoCommitTodos { todos, vdom }
    }
}

impl Drop for AutoCommitTodos<'_> {
    fn drop(&mut self) {
        self.todos.save_to_local_storage();
        self.vdom.schedule_render();
    }
}

impl Deref for AutoCommitTodos<'_> {
    type Target = Todos;

    fn deref(&self) -> &Todos {
        &self.todos
    }
}

impl DerefMut for AutoCommitTodos<'_> {
    fn deref_mut(&mut self) -> &mut Todos {
        &mut self.todos
    }
}
