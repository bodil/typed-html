#![feature(proc_macro_hygiene)]
#![feature(proc_macro_quote)]
#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_raw_ident)]

extern crate lalrpop_util;
extern crate pom;
extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

mod config;
mod declare;
mod html;
mod lexer;
mod map;
mod parser;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let stream = lexer::unroll_stream(input, false);
    let result = html::expand_html(&stream);
    match result {
        Err(error) => {
            lexer::parse_error(&stream, &error).emit();
            panic!("macro expansion produced errors; see above.")
        }
        Ok(node) => node.into_token_stream(),
    }
}

#[proc_macro]
pub fn declalrpop_element(input: TokenStream) -> TokenStream {
    let stream = lexer::keywordise(lexer::unroll_stream(input, true));
    let result = declare::expand_declare_lalrpop(&stream);
    match result {
        Err(error) => {
            lexer::parse_error(&stream, &error).emit();
            panic!("macro expansion produced errors; see above.")
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

#[proc_macro]
pub fn declare_element(input: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = declare::expand_declare(&input);
    match result {
        Err(error) => {
            parser::parse_error(&input, &error).emit();
            panic!("macro expansion produced errors; see above.")
        }
        Ok(ts) => ts,
    }
}
