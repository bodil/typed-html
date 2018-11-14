#![feature(try_from)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate typed_html;
extern crate typed_html_macros;

use typed_html::dom::Node;
use typed_html::types::*;
use typed_html_macros::html;

struct Foo {
    foo: &'static str,
}

fn main() {
    let the_big_question = text!("How does she eat?");
    let splain_class = "well-actually";
    let wibble = Foo { foo: "welp" };
    let doc = html!(
        <html>
            <head>
                <title>"Hello Kitty!"</title>
                <link rel=LinkType::StyleSheet href="lol.css"/>
            </head>
            <body>
                <h1 data-lol="omg" data-foo=wibble.foo>"Hello Kitty!"</h1>
                <p class=splain_class>
                    "She is not a "<em><a href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>". She is a "<em>"human girl"</em>"."
                </p>
                <p class=["foo", "bar"]>{the_big_question}</p>
                {
                    (1..4).map(|i| {
                        html!(<p>{ text!("{}. Ceci n'est pas une chatte.", i) }</p>)
                    })
                }
                <p>"<img src=\"javascript:alert('pwned lol')\">"</p>
            </body>
        </html>
    );
    println!("{}", doc.to_string());
    println!("{:?}", doc.vnode());
}
