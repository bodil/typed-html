#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt::Display;
use typed_html_macros::declare_element;

use super::types::*;

pub trait Node: Display {}

pub trait Element: Node {
    fn name() -> &'static str;
    fn attribute_names() -> &'static [&'static str];
    fn required_children() -> &'static [&'static str];
    fn attributes(&self) -> Vec<(String, String)>;
}

pub trait MetadataContent: Node {}
pub trait FlowContent: Node {}
pub trait PhrasingContent: Node {}

impl IntoIterator for TextNode {
    type Item = TextNode;
    type IntoIter = std::vec::IntoIter<TextNode>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl IntoIterator for Box<TextNode> {
    type Item = Box<TextNode>;
    type IntoIter = std::vec::IntoIter<Box<TextNode>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

pub struct TextNode(String);

#[macro_export]
macro_rules! text {
    ($t:expr) => {
        Box::new($crate::elements::TextNode::new($t))
    };
    ($format:tt, $($tail:tt),*) => {
        Box::new($crate::elements::TextNode::new(format!($format, $($tail),*)))
    };
}

impl TextNode {
    pub fn new<S: Into<String>>(s: S) -> Self {
        TextNode(s.into())
    }
}

impl Display for TextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl Node for TextNode {}
impl FlowContent for TextNode {}
impl PhrasingContent for TextNode {}

declare_element!(html {
    xmlns: Uri,
} [head, body]);
declare_element!(head {} [title] MetadataContent);
declare_element!(body {} [] FlowContent);

// Metadata content
declare_element!(base {
    href: Uri,
    target: String,
} [] [MetadataContent]);
declare_element!(link {
    as: Mime,
    crossorigin: CrossOrigin,
    href: Uri,
    hreflang: LanguageTag,
    media: String, // FIXME media query
    rel: LinkType,
    sizes: String, // FIXME
    title: String, // FIXME
    type: Mime,
} [] [MetadataContent]);
declare_element!(meta {
    charset: String, // FIXME IANA standard names
    content: String,
    http_equiv: String, // FIXME string enum
    name: String, // FIXME string enum
} [] [MetadataContent]);
declare_element!(style {
    type: Mime,
    media: String, // FIXME media query
    nonce: String, // bigint?
    title: String, // FIXME
} [] [MetadataContent] TextNode);
declare_element!(title {} [] [MetadataContent] TextNode);

// Flow content
declare_element!(div {} [] [FlowContent] FlowContent);
declare_element!(p {} [] [FlowContent] PhrasingContent);
declare_element!(h1 {} [] [FlowContent] PhrasingContent);
declare_element!(h2 {} [] [FlowContent] PhrasingContent);
declare_element!(h3 {} [] [FlowContent] PhrasingContent);
declare_element!(h4 {} [] [FlowContent] PhrasingContent);
declare_element!(h5 {} [] [FlowContent] PhrasingContent);
declare_element!(h6 {} [] [FlowContent] PhrasingContent);
declare_element!(em {} [] [FlowContent, PhrasingContent] PhrasingContent);

// Don't @ me
declare_element!(blink {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(marquee {
    behavior: String, // FIXME enum
    bgcolor: String, // FIXME colour
    direction: String, // FIXME direction enum
    height: String, // FIXME size
    hspace: String, // FIXME size
    loop: isize,
    scrollamount: usize,
    scrolldelay: usize,
    truespeed: bool,
    vspace: String, // FIXME size
    width: String, // FIXME size
} [] [FlowContent, PhrasingContent] PhrasingContent);
