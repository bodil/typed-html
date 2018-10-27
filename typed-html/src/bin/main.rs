#![feature(proc_macro_hygiene)]

use typed_html::elements::TextNode;
use typed_html_macros::html;

fn main() {
    let the_big_question = TextNode::new("How does she eat?");
    let splain_class = "well-actually";
    let doc = html!(
        <html>
            <head>
                <title>"Hello Kitty!"</title>
            </head>
            <body>
                <h1>"Hello Kitty!"</h1>
                <p class=splain_class>"She is not a cat. She is a human girl."</p>
                <p class="mind-blown">{the_big_question}</p>
                {
                    (1..4).map(|i| {
                        html!(<p>{ TextNode::new(format!("Generated paragraph {}", i)) }</p>)
                    })
                }
            </body>
        </html>
    );
    println!("{}", doc.to_string());
}
