


pub fn from_unstable(span: proc_macro::Span) -> proc_macro2::Span {
    let ident = proc_macro::Ident::new("_", span);
    let tt = proc_macro::TokenTree::Ident(ident);
    let tts = proc_macro::TokenStream::from(tt);
    let tts2 = proc_macro2::TokenStream::from(tts);
    tts2.into_iter().next().unwrap().span()
}
