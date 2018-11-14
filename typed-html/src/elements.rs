#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt::Display;
use typed_html_macros::{declalrpop_element, declare_element};

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
pub trait MapContent: Node {}
pub trait MediaContent: Node {} // <audio> and <video>
pub trait SelectContent: Node {}
pub trait TableContent: Node {}
pub trait TableColumnContent: Node {}

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

declalrpop_element!{
    html {
        xmlns: Uri,
    } with [head, body];
    head with [title] MetadataContent;
    body with FlowContent;

    // Metadata
    base {
        href: Uri,
        target: Target,
    } in [MetadataContent];
    link {
        as: Mime,
        crossorigin: CrossOrigin,
        href: Uri,
        hreflang: LanguageTag,
        media: String, // FIXME media query
        rel: LinkType,
        sizes: String, // FIXME
        title: String, // FIXME
        type: Mime,
    } in [MetadataContent];
    meta {
        charset: String, // FIXME IANA standard names
        content: String,
        http_equiv: String, // FIXME string enum
        name: String, // FIXME string enum
    } in [MetadataContent];
    style {
        type: Mime,
        media: String, // FIXME media query
        nonce: Nonce,
        title: String, // FIXME
    } in [MetadataContent] with TextNode;
    title in [MetadataContent] with TextNode;

    // Flow
    a {
        download: String,
        href: Uri,
        hreflang: LanguageTag,
        ping: SpacedList<Uri>,
        rel: SpacedList<LinkType>,
        target: Target,
        type: Mime,
    } in [FlowContent, PhrasingContent, InteractiveContent] with FlowContent;
    abbr in [FlowContent, PhrasingContent] with PhrasingContent;
    address in [FlowContent] with FlowContent;
    article in [FlowContent, SectioningContent] with FlowContent;
    aside in [FlowContent, SectioningContent] with FlowContent;
    audio {
        autoplay: bool,
        controls: bool,
        crossorigin: CrossOrigin,
        loop: bool,
        muted: bool,
        preload: Preload,
        src: Uri,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with MediaContent;
    b in [FlowContent, PhrasingContent] with PhrasingContent;
    bdo in [FlowContent, PhrasingContent] with PhrasingContent;
    bdi in [FlowContent, PhrasingContent] with PhrasingContent;
    blockquote {
        cite: Uri,
    } in [FlowContent] with FlowContent;
    br in [FlowContent, PhrasingContent];
    button {
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
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with PhrasingContent;
    canvas {
        height: usize,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with FlowContent;
    cite in [FlowContent, PhrasingContent] with PhrasingContent;
    code in [FlowContent, PhrasingContent] with PhrasingContent;
    data {
        value: String,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    datalist in [FlowContent, PhrasingContent] with Element_option;
    del {
        cite: Uri,
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with FlowContent;
    details {
        open: bool,
    } in [FlowContent, SectioningContent, InteractiveContent] with [summary] FlowContent;
}

// Flow content
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
declare_element!(ins {
    cite: Uri,
    datetime: Datetime,
} [] [FlowContent, PhrasingContent] FlowContent);
declare_element!(kbd {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(label {
    for: Id,
    form: Id,
} [] [FlowContent, PhrasingContent, InteractiveContent, FormContent] PhrasingContent);
declare_element!(main {} [] [FlowContent] FlowContent);
declare_element!(map {
    name: Id,
} [] [FlowContent, PhrasingContent] MapContent);
declare_element!(mark {} [] [FlowContent, PhrasingContent] PhrasingContent);
// TODO the <math> element
declare_element!(meter {
    value: isize,
    min: isize,
    max: isize,
    low: isize,
    high: isize,
    optimum: isize,
    form: Id,
} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(nav {} [] [FlowContent, SectioningContent] PhrasingContent);
declare_element!(noscript {} [] [MetadataContent, FlowContent, PhrasingContent] Node);
declare_element!(object {
    data: Uri,
    form: Id,
    height: usize,
    name: Id,
    type: Mime,
    typemustmatch: bool,
    usemap: String, // TODO should be a fragment starting with '#'
    width: usize,
} [] [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent, FormContent] Element_param);
declare_element!(ol {
    reversed: bool,
    start: isize,
    type: OrderedListType,
} [] [FlowContent] Element_li);
declare_element!(output {
    for: SpacedSet<Id>,
    form: Id,
    name: Id,
} [] [FlowContent, PhrasingContent, FormContent] PhrasingContent);
declare_element!(p {} [] [FlowContent] PhrasingContent);
declare_element!(pre {} [] [FlowContent] PhrasingContent);
declare_element!(progress {
    max: f64,
    value: f64,
} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(q {
    cite: Uri,
} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(ruby {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(s {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(samp {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(script {
    async: bool,
    crossorigin: CrossOrigin,
    defer: bool,
    integrity: Integrity,
    nomodule: bool,
    nonce: Nonce,
    src: Uri,
    text: String,
    type: String, // TODO could be an enum
} [] [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] TextNode);
declare_element!(section {} [] [FlowContent, SectioningContent] FlowContent);
declare_element!(select {
    autocomplete: String,
    autofocus: bool,
    disabled: bool,
    form: Id,
    multiple: bool,
    name: Id,
    required: bool,
    size: usize,
} [] [FlowContent, PhrasingContent, InteractiveContent, FormContent] SelectContent);
declare_element!(small {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(span {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(strong {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(sub {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(sup {} [] [FlowContent, PhrasingContent] PhrasingContent);
// TODO the <svg> element
declare_element!(table {} [] [FlowContent] TableContent);
declare_element!(template {} [] [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] Node);
declare_element!(textarea {
    autocomplete: OnOff,
    autofocus: bool,
    cols: usize,
    disabled: bool,
    form: Id,
    maxlength: usize,
    minlength: usize,
    name: Id,
    placeholder: String,
    readonly: bool,
    required: bool,
    rows: usize,
    spellcheck: BoolOrDefault,
    wrap: Wrap,
} [] [FlowContent, PhrasingContent, InteractiveContent, FormContent] TextNode);
declare_element!(time {
    datetime: Datetime,
} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(ul {} [] [FlowContent] Element_li);
declare_element!(var {} [] [FlowContent, PhrasingContent] PhrasingContent);
declare_element!(video {} [] [FlowContent, PhrasingContent, EmbeddedContent] MediaContent);
declare_element!(wbr {} [] [FlowContent, PhrasingContent]);

// Non-group elements
declare_element!(area {
    alt: String,
    coords: String, // TODO could perhaps be validated
    download: bool,
    href: Uri,
    hreflang: LanguageTag,
    ping: SpacedList<Uri>,
    rel: SpacedSet<LinkType>,
    shape: AreaShape,
    target: Target,
} [] [MapContent]);
declare_element!(caption {} [] [TableContent] FlowContent);
declare_element!(col {
    span: usize,
} [] []);
declare_element!(colgroup {
    span: usize,
} [] [TableContent] Element_col);
declare_element!(dd {} [] [DescriptionListContent] FlowContent);
declare_element!(dt {} [] [DescriptionListContent] FlowContent);
declare_element!(figcaption {} [] [] FlowContent);
declare_element!(legend {} [] [] PhrasingContent);
declare_element!(li {
    value: isize,
} [] [] FlowContent);
declare_element!(option {
    disabled: bool,
    label: String,
    selected: bool,
    value: String,
} [] [SelectContent] TextNode);
declare_element!(optgroup {
    disabled: bool,
    label: String,
} [] [SelectContent] Element_option);
declare_element!(param {
    name: String,
    value: String,
} [] []);
declare_element!(source {
    src: Uri,
    type: Mime,
} [] [MediaContent]);
declare_element!(summary {} [] [] PhrasingContent);
declare_element!(tbody {} [] [TableContent] Element_tr);
declare_element!(td {
    colspan: usize,
    headers: SpacedSet<Id>,
    rowspan: usize,
} [] [TableColumnContent] FlowContent);
declare_element!(tfoot {} [] [TableContent] Element_tr);
declare_element!(th {
    abbr: String,
    colspan: usize,
    headers: SpacedSet<Id>,
    rowspan: usize,
    scope: TableHeaderScope,
} [] [TableColumnContent] FlowContent);
declare_element!(thead {} [] [TableContent] Element_tr);
declare_element!(tr {} [] [TableContent] TableColumnContent);
declare_element!(track {
    default: bool,
    kind: VideoKind,
    label: String,
    src: Uri,
    srclang: LanguageTag,
} [] [MediaContent]);

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
