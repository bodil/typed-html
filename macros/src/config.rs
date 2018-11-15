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

// This NEEDS to be a sorted list!
pub static ATTR_EVENTS: &[&str] = &[
    "abort",
    // "autocomplete",
    // "autocompleteerror",
    "blur",
    // "cancel",
    // "canplay",
    // "canplaythrough",
    "change",
    "click",
    // "close",
    "contextmenu",
    // "cuechange",
    "dblclick",
    "drag",
    "dragend",
    "dragenter",
    "dragexit",
    "dragleave",
    "dragover",
    "dragstart",
    "drop",
    // "durationchange",
    // "emptied",
    // "ended",
    "error",
    "focus",
    "input",
    // "invalid",
    "keydown",
    "keypress",
    "keyup",
    "load",
    // "loadeddata",
    // "loadedmetadata",
    "loadstart",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "mousewheel",
    // "pause",
    // "play",
    // "playing",
    "progress",
    // "ratechange",
    // "reset",
    "resize",
    "scroll",
    // "seeked",
    // "seeking",
    // "select",
    // "show",
    // "sort",
    // "stalled",
    "submit",
    // "suspend",
    // "timeupdate",
    // "toggle",
    // "volumechange",
    // "waiting",
];
