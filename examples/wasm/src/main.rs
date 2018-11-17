#![feature(try_from)]
#![feature(proc_macro_hygiene)]

extern crate stdweb;
extern crate typed_html;
extern crate typed_html_macros;

use stdweb::web::{self, Element, IElement, INode};
use typed_html::dom::{Node, VNode};
use typed_html::events::Events;
use typed_html::{for_events, html, DOM};

fn install_handlers(target: &Element, handlers: &mut Events<DOM>) {
    for_events!(handler in handlers => {
        handler.attach(target);
    });
}

fn build(
    document: &web::Document,
    vnode: VNode<'_, DOM>,
) -> Result<web::Node, web::error::InvalidCharacterError> {
    match vnode {
        VNode::Text(text) => Ok(document.create_text_node(&text).into()),
        VNode::Element(element) => {
            let mut node = document.create_element(element.name)?;
            for (key, value) in element.attributes {
                node.set_attribute(&key, &value)?;
            }
            install_handlers(&node, element.events);
            for child in element.children {
                let child_node = build(document, child)?;
                node.append_child(&child_node);
            }
            Ok(node.into())
        }
    }
}

fn main() {
    let mut doc = html!(
        <div>
            <h1>"Hello Kitty"</h1>
            <p>
                "She is not a "<em><a href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>
                ". She is a "<em>"human girl"</em>"."
            </p>
            <p>
                <button onclick=(|_event| web::alert("Hello Joe!"))>
                    "Call Joe"
                </button>
            </p>
        </div>
    );
    let vdom = doc.vnode();
    let document = web::document();
    let body = document.body().expect("no body element in doc");
    let tree = build(&document, vdom).unwrap();
    body.append_child(&tree);
}
