#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};

mod config;
mod declare;
mod html;
mod parser;

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = html::expand_html(&input);
    match result {
        Err(error) => panic!(parser::parse_error(&input, &error)),
        Ok(ts) => ts.into(),
    }
}

#[proc_macro]
pub fn declare_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = declare::expand_declare(&input);
    match result {
        Err(error) => panic!(parser::parse_error(&input, &error)),
        Ok(ts) => ts.into(),
    }
}
