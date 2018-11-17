#![feature(proc_macro_hygiene)]
#![feature(proc_macro_quote)]
#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_raw_ident)]

extern crate ansi_term;
extern crate lalrpop_util;
extern crate proc_macro;

use proc_macro::{quote, TokenStream};

mod config;
mod declare;
mod error;
mod html;
mod lexer;
mod map;
mod parser;

/// Construct a DOM tree.
///
/// # Syntax
///
/// This macro largely follows [JSX] syntax, but with some differences:
///
/// * Text nodes must be quoted, because there's only so much Rust's tokeniser
///   can handle outside string literals. So, instead of `<p>Hello</p>`, you
///   need to write `<p>"Hello"</p>`. (The parser will throw an error asking you
///   to do this if you forget.)
/// * Element attributes will accept simple Rust expressions, but the parser has
///   its limits, as it's not a full Rust parser. You can use literals,
///   variables, dotted properties and single function or method calls. If you
///   use something the parser isn't currently capable of handling, it will
///   complain. You can put braces or parentheses around the expression if the
///   parser doesn't understand it. You can use any Rust code inside a brace or
///   parenthesis block.
///
/// # Valid HTML5
///
/// The macro will only accept valid HTML5 tags, with no tags or attributes
/// marked experimental or obsolete. If it won't accept something you want it to
/// accept, we can discuss it over a pull request (experimental tags and
/// attributes, in particular, are mostly omitted just for brevity, and you're
/// welcome to implement them).
///
/// The structure validation is simplistic by necessity, as it defers to the
/// type system: a few elements will have one or more required children, and any
/// element which accepts children will have a restriction on the type of the
/// children, usually a broad group as defined by the HTML spec. Many elements
/// have restrictions on children of children, or require a particular ordering
/// of optional elements, which isn't currently validated.
///
/// # Attribute Values
///
/// Brace blocks in the attribute value position should return the expected type
/// for the attribute. The type checker will complain if you return an
/// unsupported type. You can also use literals or a few simple Rust expressions
/// as attribute values (see the Syntax section above).
///
/// The `html!` macro will add an `.into()` call to the value expression, so
/// that you can use any type that has an `Into<A>` trait defined for the actual
/// attribute type `A`.
///
/// As a special case, if you use a string literal, the macro will instead use
/// the `FromStr<A>` trait to try and parse the string literal into the expected
/// type. This is extremely useful for eg. CSS classes, letting you type
/// `class="css-class-1 css-class-2"` instead of going to the trouble of
/// constructing a `SpacedSet<Class>`. The big caveat for this: currently, the
/// macro is not able to validate the string at compile time, and the conversion
/// will panic at runtime if the string is invalid.
///
/// ## Example
///
/// ```no_compile
/// let classList: SpacedSet<Class> = ["foo", "bar", "baz"].into();
/// html!(
///     <div class="foo bar baz"></div>         // parses a string literal
///     <div class=["foo", "bar", "baz"]></div> // uses From<[&str, &str, &str]>
///     <div class=classList></div>             // uses a variable in scope
///     <div class={                            // evaluates a code block
///         SpacedSet::from(["foo", "bar", "baz"])
///     }></div>
/// )
/// ```
///
/// # Generated Nodes
///
/// Brace blocks in the child node position are expected to return an
/// `IntoIterator` of `DOMTree`s. You can return single elements or text nodes,
/// as they both implement `IntoIterator` for themselves. The macro will consume
/// this iterator at runtime and insert the generated nodes as children in the
/// expected position.
///
/// ## Example
///
/// ```no_compile
/// html!(
///     <ul>
///         { (1..=5).map(|i| html!(
///             <li>{ text!("{}", i) }</li>
///         )) }
///     </ul>
/// )
/// ```
///
/// [JSX]: https://reactjs.org/docs/introducing-jsx.html
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let stream = lexer::unroll_stream(input, false);
    let result = html::expand_html(&stream);
    match result {
        Err(err) => {
            error::parse_error(&stream, &err).emit();
            quote!(panic!())
        }
        Ok(node) => node.into_token_stream(),
    }
}

/// This macro is used by `typed_html` internally to generate types and
/// implementations for HTML elements.
#[proc_macro]
pub fn declare_elements(input: TokenStream) -> TokenStream {
    let stream = lexer::keywordise(lexer::unroll_stream(input, true));
    let result = declare::expand_declare(&stream);
    match result {
        Err(err) => {
            error::parse_error(&stream, &err).emit();
            quote!(panic!())
        }
        Ok(decls) => {
            let mut out = TokenStream::new();
            for decl in decls {
                out.extend(decl.into_token_stream());
            }
            out
        }
    }
}
