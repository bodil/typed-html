#![recursion_limit = "128"]
#![cfg_attr(can_show_location_of_runtime_parse_error, feature(proc_macro_span))]

extern crate proc_macro;

use proc_macro::TokenStream;

mod config;
mod declare;
mod error;
mod html;
mod ident;
mod lexer;
mod map;
mod parser;
mod span;

/// Construct a DOM tree.
///
/// See the crate documentation for [`typed_html`][typed_html].
///
/// [typed_html]: ../typed_html/index.html
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let stream = lexer::unroll_stream(input.into(), false);
    let result = html::expand_html(&stream);
    TokenStream::from(match result {
        Err(err) => error::parse_error(&stream, &err),
        Ok((node, ty)) => match node.into_token_stream(&ty) {
            Err(err) => err,
            Ok(success) => success,
        },
    })
}

/// Construct a Dodrio node.
///
/// See the crate documentation for [`typed_html`][typed_html].
///
/// [typed_html]: ../typed_html/index.html
#[cfg(feature = "dodrio")]
#[proc_macro]
pub fn dodrio(input: TokenStream) -> TokenStream {
    let stream = lexer::unroll_stream(input.into(), false);
    let result = html::expand_dodrio(&stream);
    TokenStream::from(match result {
        Err(err) => error::parse_error(&stream, &err),
        Ok((bump, node)) => match node.into_dodrio_token_stream(&bump, false) {
            Err(err) => err,
            // Ok(success) => {println!("{}", success); panic!()},
            Ok(success) => success,
        },
    })
}

/// This macro is used by `typed_html` internally to generate types and
/// implementations for HTML elements.
#[proc_macro]
pub fn declare_elements(input: TokenStream) -> TokenStream {
    let stream = lexer::keywordise(lexer::unroll_stream(input.into(), true));
    let result = declare::expand_declare(&stream);
    TokenStream::from(match result {
        Err(err) => error::parse_error(&stream, &err),
        Ok(decls) => {
            let mut out = proc_macro2::TokenStream::new();
            for decl in decls {
                out.extend(decl.into_token_stream());
            }
            out
        }
    })
}
