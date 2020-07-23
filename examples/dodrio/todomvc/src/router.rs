//! A simple `#`-fragment router.

use crate::todos::Todos;
use crate::utils;
use crate::visibility::Visibility;
use dodrio::VdomWeak;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Start the router.
pub fn start(vdom: VdomWeak) {
    // Callback fired whenever the URL's hash fragment changes. Keeps the root
    // todos collection's visibility in sync with the `#` fragment.
    let on_hash_change = move || {
        let new_vis = utils::hash()
            .and_then(|hash| {
                if hash.starts_with("#/") {
                    Visibility::from_str(&hash[2..]).ok()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                // If we couldn't parse a visibility, make sure we canonicalize
                // it back to our default hash.
                let v = Visibility::default();
                utils::set_hash(&format!("#/{}", v));
                v
            });

        let vdom = vdom.clone();
        wasm_bindgen_futures::spawn_local(async move {
            vdom.with_component({
                let vdom = vdom.clone();
                move |root| {
                    let todos = root.unwrap_mut::<Todos>();
                    // If the todos' visibility already matches the event's
                    // visibility, then there is nothing to do (ha). If they
                    // don't match, then we need to update the todos' visibility
                    // and re-render.
                    if todos.visibility() != new_vis {
                        todos.set_visibility(new_vis);
                        vdom.schedule_render();
                    }
                }
            }).await.ok();
        });
    };

    // Call it once to handle the initial `#` fragment.
    on_hash_change();

    // Now listen for hash changes forever.
    //
    // Note that if we ever intended to unmount our todos app, we would want to
    // provide a method for removing this router's event listener and cleaning
    // up after ourselves.
    let on_hash_change = Closure::wrap(Box::new(on_hash_change) as Box<dyn FnMut()>);
    let window = utils::window();
    window
        .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
        .unwrap_throw();
    on_hash_change.forget();
}
