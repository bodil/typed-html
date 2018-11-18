use proc_macro2::{Ident, Span, TokenStream, TokenTree};

use std::str::FromStr;

pub fn new_raw(string: &str, span: Span) -> Ident {
    // Validate that it is an ident.
    let _ = Ident::new(string, span);

    let s = format!("r#{}", string);
    let tts = TokenStream::from_str(&s).unwrap();
    let mut ident = match tts.into_iter().next().unwrap() {
        TokenTree::Ident(ident) => ident,
        _ => unreachable!(),
    };
    ident.set_span(span);
    ident
}
