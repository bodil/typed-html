#![feature(proc_macro_hygiene)]
#![feature(proc_macro_quote)]
#![feature(proc_macro_span)]

extern crate pom;
extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

mod config;
mod declare;
mod html;
mod map;
mod parser;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = html::expand_html(&input);
    match result {
        Err(error) => panic!(parser::parse_error(&input, &error)),
        Ok(ts) => ts,
    }
}

#[proc_macro]
pub fn declare_element(input: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = declare::expand_declare(&input);
    match result {
        Err(error) => panic!(parser::parse_error(&input, &error)),
        Ok(ts) => ts,
    }
}
