#![allow(non_camel_case_types)]
#![allow(dead_code)]

use http::Uri;
use std::fmt::Display;
use typed_html_macros::declare_element;

pub type CssId = String;
pub type CssClass = String;

pub trait Node: Display {}

pub trait Element: Node {
    fn attributes() -> &'static [&'static str];
    fn required_children() -> &'static [&'static str];
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

pub struct TextNode(String);

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
declare_element!(title {} [] [MetadataContent] TextNode);
declare_element!(body {} [] FlowContent);
declare_element!(p {} [] [FlowContent] PhrasingContent);
declare_element!(h1 {} [] [FlowContent] PhrasingContent);
declare_element!(em {} [] [FlowContent, PhrasingContent] PhrasingContent);
