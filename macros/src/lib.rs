#![recursion_limit = "128"]
#![feature(proc_macro_hygiene)]
#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

extern crate ansi_term;
extern crate lalrpop_util;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;

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
        Err(err) => {
            error::parse_error(&stream, &err).emit();
            quote!(panic!())
        }
        Ok(node) => node.into_token_stream(),
    })
}

/// This macro is used by `typed_html` internally to generate types and
/// implementations for HTML elements.
#[proc_macro]
pub fn declare_elements(input: TokenStream) -> TokenStream {
    let stream = lexer::keywordise(lexer::unroll_stream(input.into(), true));
    let result = declare::expand_declare(&stream);
    TokenStream::from(match result {
        Err(err) => {
            error::parse_error(&stream, &err).emit();
            quote!(panic!())
        }
        Ok(decls) => {
            let mut out = proc_macro2::TokenStream::new();
            for decl in decls {
                out.extend(decl.into_token_stream());
            }
            out
        }
    })
}
