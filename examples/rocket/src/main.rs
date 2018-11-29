#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate typed_html;
extern crate typed_html_macros;

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket::{get, routes, Request, Response};
use std::io::Cursor;
use typed_html::types::LinkType;
use typed_html::elements::FlowContent;
use typed_html::{dom::DOMTree, html, text, OutputType};

struct Html(DOMTree<String>);

impl<'r> Responder<'r> for Html {
    fn respond_to(self, _request: &Request) -> Result<'r> {
        Ok(Response::build()
            .status(Status::Ok)
            .header(ContentType::HTML)
            .sized_body(Cursor::new(self.0.to_string()))
            .finalize())
    }
}

// Function that wraps a DOM node in an HTML document, to demonstrate how you'd
// do this sort of templating.
//
// It's a bit more complicated than you'd hope because you need to take an input
// argument of the type that the element that you're inserting it into expects,
// which in the case of `<body>` is `FlowContent`, not just `Node`, so you can't
// pass it a `DOMTree<T>` or you'll get a type error.
fn doc<T: OutputType + 'static>(tree: Box<dyn FlowContent<T>>) -> DOMTree<T> {
    html!(
        <html>
            <head>
                <title>"Hello Kitty!"</title>
                <link rel=LinkType::StyleSheet href="lol.css"/>
            </head>
            <body>
                { tree }
            </body>
        </html>
    )
}

#[get("/")]
fn index() -> Html {
    let a = false;
    Html(doc(html!(
        <div>
            <h1 data-lol="omg">"Hello Kitty!"</h1>
            <p class="official-position-of-sanrio-ltd emphasis">
                "She is not a "<em><a href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>". She is a "<em>"human girl"</em>"."
            </p>
            <p class=["urgent", "question"]>"But how does she eat?"</p>
            {
                (1..4).map(|i| {
                    html!(<p>{ text!("{}. Ceci n'est pas une chatte.", i) }</p>)
                })
            }
            <p>"<img src=\"javascript:alert('pwned lol')\">"</p>
            <button disabled=a onclick="alert('She is not a cat.')">"Click me!"</button>
        </div>
    )))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
