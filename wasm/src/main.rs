#![feature(try_from)]
#![feature(proc_macro_hygiene)]

extern crate stdweb;
extern crate typed_html;
extern crate typed_html_macros;

use stdweb::web::{self, IElement, INode};
use typed_html::dom::{Node, VNode};
use typed_html_macros::html;

fn build(
    document: &web::Document,
    vnode: VNode,
) -> Result<web::Node, web::error::InvalidCharacterError> {
    match vnode {
        VNode::Text(text) => Ok(document.create_text_node(&text).into()),
        VNode::Element(element) => {
            let mut node = document.create_element(element.name)?;
            for (key, value) in element.attributes {
                node.set_attribute(&key, &value)?;
            }
            for child in element.children {
                let child_node = build(document, child)?;
                node.append_child(&child_node);
            }
            Ok(node.into())
        }
    }
}

fn main() {
    let doc = html!(
        <div>
            <h1>"Hello Kitty"</h1>
            <p>
                "She is not a "<em><a href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>
                ". She is a "<em>"human girl"</em>"."
            </p>
        </div>
    );
    let vdom = doc.vnode();
    let document = web::document();
    let body = document.body().expect("no body element in doc");
    let tree = build(&document, vdom).unwrap();
    body.append_child(&tree);
}
