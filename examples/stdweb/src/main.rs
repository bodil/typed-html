#![recursion_limit = "256"]

extern crate stdweb;
extern crate typed_html;

use stdweb::web::{self, INode};
use typed_html::dom::Node;
use typed_html::html;
use typed_html::output::stdweb::Stdweb;

fn main() {
    let mut doc = html!(
        <div role="main">
            <h1>"Hello Kitty"</h1>
            <p>
                "She is not a "<em><a href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>
                ". She is a "<em>"human girl"</em>"."
            </p>
            <p>
                <button onclick={ |_event| web::alert("Hello Joe!") }>
                    "Call Joe"
                </button>
            </p>
        </div>
    : Stdweb);
    let vdom = doc.vnode();
    let document = web::document();
    let body = document.body().expect("no body element in doc");
    let tree = Stdweb::build(&document, vdom).unwrap();
    body.append_child(&tree);
}
