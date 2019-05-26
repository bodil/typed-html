#![recursion_limit = "256"]

use iron::headers::ContentType;
use iron::modifier::Modifier;
use iron::prelude::*;
use iron::status;
use typed_html::elements::FlowContent;
use typed_html::types::LinkType;
use typed_html::{dom::DOMTree, html, text, OutputType};

struct Html(DOMTree<String>);

impl Modifier<Response> for Html {
    fn modify(self, res: &mut Response) {
        res.body = Some(Box::new(self.0.to_string()));
        res.headers.set(ContentType::html());
    }
}

// Function that wraps a DOM node in an HTML document, to demonstrate how you'd
// do this sort of templating.
//
// It's a bit more complicated than you'd hope because you need to take an input
// argument of the type that the element that you're inserting it into expects,
// which in the case of `<body>` is `FlowContent`, not just `Node`, so you can't
// pass it a `DOMTree<T>` or you'll get a type error.
fn doc<T: OutputType + 'static + Send>(tree: Box<dyn FlowContent<T>>) -> DOMTree<T> {
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

fn index() -> Html {
    let a = true;
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
    : String)))
}

fn main() {
    Iron::new(|_: &mut Request| Ok(Response::with((status::Ok, index()))))
        .http("localhost:1337")
        .unwrap();
}
