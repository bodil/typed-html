#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt::Display;
use typed_html_macros::declare_element;

use super::types::*;

#[derive(Clone, Debug)]
pub enum VNode {
    Text(String),
    Element(VElement),
}

#[derive(Clone, Debug)]
pub struct VElement {
    pub name: &'static str,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<VNode>,
}

pub trait Node: Display {
    fn vnode(&self) -> VNode;
}

pub trait Element: Node {
    fn name() -> &'static str;
    fn attribute_names() -> &'static [&'static str];
    fn required_children() -> &'static [&'static str];
    fn attributes(&self) -> Vec<(String, String)>;
}

pub trait MetadataContent: Node {}
pub trait FlowContent: Node {}
pub trait SectioningContent: Node {}
pub trait HeadingContent: Node {}
// Phrasing content seems to be entirely a subclass of FlowContent
pub trait PhrasingContent: FlowContent {}
pub trait EmbeddedContent: Node {}
pub trait InteractiveContent: Node {}
pub trait FormContent: Node {}

// Traits for elements that are more picky about their children
pub trait DescriptionListContent: Node {}
pub trait HGroupContent: Node {}

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

impl Node for TextNode {
    fn vnode(&self) -> VNode {
        VNode::Text(self.0.clone())
    }
}
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
    target: Target,
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
declare_element!(a {
    download: String,
    href: Uri,
    hreflang: LanguageTag,
    ping: SpacedList<Uri>,
    rel: SpacedList<LinkType>,
    target: Target,
    type: Mime,
} [] [FlowContent, PhrasingContent, InteractiveContent] FlowContent);
declare_element!(abbr {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(address {} [] [FlowContent] FlowContent); // FIXME it has additional constraints on FlowContent
declare_element!(article {} [] [FlowContent, SectioningContent] FlowContent);
declare_element!(aside {} [] [FlowContent, SectioningContent] FlowContent);
declare_element!(audio {
    autoplay: bool,
    controls: bool,
    crossorigin: CrossOrigin,
    loop: bool,
    muted: bool,
    preload: Preload,
    src: Uri,
} [] [FlowContent, PhrasingContent]);
declare_element!(b {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(bdo {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(bdi {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(blockquote {
    cite: Uri,
} [] [FlowContent] FlowContent);
declare_element!(br {} [] [FlowContent, PhrasingContent]);
declare_element!(button {
    autofocus: bool,
    disabled: bool,
    form: Id,
    formaction: Uri,
    formenctype: FormEncodingType,
    formmethod: FormMethod,
    formnovalidate: bool,
    formtarget: Target,
    name: Id,
    type: ButtonType,
    value: String,
} [] [FlowContent, PhrasingContent, InteractiveContent, FormContent] PhrasingContent);
declare_element!(canvas {
    height: usize,
    width: usize,
} [] [FlowContent] FlowContent); // FIXME has additional child constraints
declare_element!(cite {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(code {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(data {
    value: String,
} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(datalist {} [] [FlowContent, PhrasingContent] Element_option);
declare_element!(del {
    cite: Uri,
    datetime: String, // FIXME should be "a valid date string with an optional time",
                      //       but I have other hells to live in right now.
} [] [FlowContent, PhrasingContent] FlowContent);
declare_element!(details {
    open: bool,
} [summary] [FlowContent, SectioningContent, InteractiveContent] FlowContent);
declare_element!(dfn {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(div {} [] [FlowContent] FlowContent);
declare_element!(dl {} [] [FlowContent] DescriptionListContent);
declare_element!(em {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(embed {
    height: usize,
    src: Uri,
    type: Mime,
    width: usize,
} [] [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent]);
// FIXME the legend attribute should be optional
declare_element!(fieldset {} [legend] [FlowContent, SectioningContent, FormContent] FlowContent);
// FIXME the figcaption attribute should be optional
declare_element!(figure {} [figcaption] [FlowContent, SectioningContent] FlowContent);
declare_element!(footer {} [] [FlowContent] FlowContent);
declare_element!(form {
    accept-charset: SpacedList<CharacterEncoding>,
    action: Uri,
    autocomplete: OnOff,
    enctype: FormEncodingType,
    method: FormMethod,
    name: Id,
    novalidate: bool,
    target: Target,
} [] [FlowContent] FlowContent);
declare_element!(h1 {} [] [FlowContent, HeadingContent, HGroupContent] PhrasingContent);
declare_element!(h2 {} [] [FlowContent, HeadingContent, HGroupContent] PhrasingContent);
declare_element!(h3 {} [] [FlowContent, HeadingContent, HGroupContent] PhrasingContent);
declare_element!(h4 {} [] [FlowContent, HeadingContent, HGroupContent] PhrasingContent);
declare_element!(h5 {} [] [FlowContent, HeadingContent, HGroupContent] PhrasingContent);
declare_element!(h6 {} [] [FlowContent, HeadingContent, HGroupContent] PhrasingContent);
declare_element!(header {} [] [FlowContent] FlowContent);
declare_element!(hgroup {} [] [FlowContent, HeadingContent] HGroupContent);
declare_element!(hr {} [] [FlowContent]);
declare_element!(i {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(iframe {
    allow: FeaturePolicy,
    allowfullscreen: bool,
    allowpaymentrequest: bool,
    height: usize,
    name: Id,
    referrerpolicy: ReferrerPolicy,
    sandbox: SpacedSet<Sandbox>,
    src: Uri,
    srcdoc: Uri,
    width: usize,
} [] [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent] FlowContent);
declare_element!(img {
    alt: String,
    crossorigin: CrossOrigin,
    decoding: ImageDecoding,
    height: usize,
    ismap: bool,
    sizes: SpacedList<String>, // FIXME it's not really just a string
    src: Uri,
    srcset: String, // FIXME this is much more complicated
    usemap: String, // FIXME should be a fragment starting with '#'
    width: usize,
} [] [FlowContent, PhrasingContent, EmbeddedContent]);
declare_element!(input {
    autocomplete: String,
    autofocus: bool,
    disabled: bool,
    form: Id,
    list: Id,
    name: Id,
    required: bool,
    tabindex: usize,
    type: InputType,
    value: String,
} [] [FlowContent, FormContent, PhrasingContent]);
declare_element!(p {} [] [FlowContent] PhrasingContent);

// Non-content elements
declare_element!(dd {} [] [DescriptionListContent] FlowContent);
declare_element!(dt {} [] [DescriptionListContent] FlowContent);
declare_element!(figcaption {} [] [] FlowContent);
declare_element!(legend {} [] [] PhrasingContent);
declare_element!(option {
    disabled: bool,
    label: String,
    selected: bool,
    value: String,
} [] [] TextNode);
declare_element!(summary {} [] [] PhrasingContent);

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
