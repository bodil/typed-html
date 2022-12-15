#![feature(proc_macro_hygiene)]

extern crate typed_html;

use typed_html::html;
use typed_html::dom::DOMTree;

fn main() {
    let _: DOMTree<String> = html!{
        <html></head>
    };
}
