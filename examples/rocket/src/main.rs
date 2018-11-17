#![feature(proc_macro_hygiene, decl_macro, try_from)]

extern crate rocket;
extern crate typed_html;
extern crate typed_html_macros;

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket::{get, routes, Request, Response};
use std::io::Cursor;
use typed_html::types::LinkType;
use typed_html::{dom::DOMTree, html, text};

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

#[get("/")]
fn index() -> Html {
    Html(html!(
        <html>
            <head>
                <title>"Hello Kitty!"</title>
                <link rel=LinkType::StyleSheet href="lol.css"/>
            </head>
            <body>
                <h1 data-lol="omg">"Hello Kitty!"</h1>
                <p class="official-position-of-sanrio-ltd">
                    "She is not a "<em><a href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>". She is a "<em>"human girl"</em>"."
                </p>
                <p class=["urgent", "question"]>"But how does she eat?"</p>
                {
                    (1..4).map(|i| {
                        html!(<p>{ text!("{}. Ceci n'est pas une chatte.", i) }</p>)
                    })
                }
                <p>"<img src=\"javascript:alert('pwned lol')\">"</p>
                <button onclick="alert('She is not a cat.')">"Click me!"</button>
            </body>
        </html>
    ))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
