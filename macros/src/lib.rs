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
/// See the crate documentation for [`typed_html`][typed_html].
///
/// [typed_html]: ../typed_html/index.html
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
