use proc_macro::{Ident, Span, TokenStream};

use map::StringyMap;

pub fn required_children(element: &str) -> &[&str] {
    match element {
        "html" => &["head", "body"],
        "head" => &["title"],
        _ => &[],
    }
}

pub fn global_attrs(span: Span) -> StringyMap<Ident, TokenStream> {
    let mut attrs = StringyMap::new();
    {
        let mut insert =
            |key, value: &str| attrs.insert(Ident::new(key, span), value.parse().unwrap());
        insert("id", "crate::elements::CssId");
        insert("class", "crate::elements::CssClass");
    }
    attrs
}
