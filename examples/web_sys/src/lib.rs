#![recursion_limit = "256"]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
extern crate web_sys;
extern crate typed_html;

use typed_html::dom::Node;
use typed_html::html;
use typed_html::output::web_sys::WebSys;

#[wasm_bindgen]
pub fn main() {
    let window = web_sys::window().expect("no global `window` exists");

    let mut doc = html!(
        <div>
            <h1>"Hello Kitty"</h1>
            <p>
                "She is not a "<em><a href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>
                ". She is a "<em>"human girl"</em>"."
            </p>
            <p>
                <button onclick={ |_event| web_sys::window().unwrap().alert_with_message("Hello Joe!").unwrap() }>
                    "Call Joe"
                </button>
            </p>
        </div>
    : WebSys);
    let vdom = doc.vnode();

    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let tree = WebSys::build(&document, vdom).unwrap();
    body.append_child(&tree).unwrap();
}
