use proc_macro2::{Ident, Span, TokenStream};

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

        insert("id", "crate::types::Id");
        insert("class", "crate::types::ClassList");

        insert("accesskey", "String");
        insert("autocapitalize", "String");
        insert("contenteditable", "bool");
        insert("contextmenu", "crate::types::Id");
        insert("dir", "crate::types::TextDirection");
        insert("draggable", "bool");
        insert("hidden", "bool");
        insert("is", "String");
        insert("lang", "crate::types::LanguageTag");
        insert("style", "String");
        insert("tabindex", "isize");
        insert("title", "String");

        // FIXME ARIA and XML attrs missing
    }
    attrs
}

pub static SELF_CLOSING: &[&str] = &[
    "area",
    "base",
    "br",
    "col",
    "command",
    "embed",
    "hr",
    "img",
    "input",
    "keygen",
    "link",
    "meta",
    "param",
    "source",
    "track",
    "wbr",
];
