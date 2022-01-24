//! Types for all standard HTML5 elements.

#![allow(non_camel_case_types)]

use crate::dom::{Node, TextNode};
use crate::types::*;
use crate::OutputType;
use typed_html_macros::declare_elements;

// Marker traits for element content groups

macro_rules! marker_trait {
    ($trait:ident) => {
        marker_trait!($trait, Node);
    };

    ($trait:ident, $parent:ident) => {
        pub trait $trait<T: OutputType + Send>: $parent<T> {}

        impl<T> IntoIterator for Box<dyn $trait<T>>
        where
            T: OutputType + Send,
        {
            type Item = Box<dyn $trait<T>>;
            type IntoIter = std::vec::IntoIter<Box<dyn $trait<T>>>;

            fn into_iter(self) -> Self::IntoIter {
                vec![self].into_iter()
            }
        }
    };
}

marker_trait!(MetadataContent);
marker_trait!(FlowContent);
marker_trait!(SectioningContent);
marker_trait!(HeadingContent);
// Phrasing content seems to be entirely a subclass of FlowContent
marker_trait!(PhrasingContent, FlowContent);
marker_trait!(EmbeddedContent);
marker_trait!(InteractiveContent);
marker_trait!(FormContent);

// Traits for elements that are more picky about their children
marker_trait!(DescriptionListContent);
marker_trait!(HGroupContent);
marker_trait!(MapContent);
marker_trait!(MediaContent); // <audio> and <video>
marker_trait!(SelectContent);
marker_trait!(TableContent);
marker_trait!(TableColumnContent);
marker_trait!(SvgContent);

declare_elements! {
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
        http_equiv: HTTPEquiv,
        name: Metadata,
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
        autoplay: Bool,
        controls: Bool,
        crossorigin: CrossOrigin,
        loop: Bool,
        muted: Bool,
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
        autofocus: Bool,
        disabled: Bool,
        form: Id,
        formaction: Uri,
        formenctype: FormEncodingType,
        formmethod: FormMethod,
        formnovalidate: Bool,
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
    datalist in [FlowContent, PhrasingContent] with option;
    del {
        cite: Uri,
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with FlowContent;
    details {
        open: Bool,
    } in [FlowContent, SectioningContent, InteractiveContent] with [summary] FlowContent;
    dfn in [FlowContent, PhrasingContent] with PhrasingContent;
    div in [FlowContent] with FlowContent;
    dl in [FlowContent] with DescriptionListContent;
    em in [FlowContent, PhrasingContent] with PhrasingContent;
    embed {
        height: usize,
        src: Uri,
        type: Mime,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent];
    // FIXME the legend attribute should be optional
    fieldset in [FlowContent, SectioningContent, FormContent] with [legend] FlowContent;
    // FIXME the figcaption attribute should be optional
    figure in [FlowContent, SectioningContent] with [figcaption] FlowContent;
    footer in [FlowContent] with FlowContent;
    form {
        accept-charset: SpacedList<CharacterEncoding>,
        action: Uri,
        autocomplete: OnOff,
        enctype: FormEncodingType,
        method: FormMethod,
        name: Id,
        novalidate: Bool,
        target: Target,
    } in [FlowContent] with FlowContent;
    h1 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h2 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h3 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h4 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h5 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h6 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    header in [FlowContent] with FlowContent;
    hgroup in [FlowContent, HeadingContent] with HGroupContent;
    hr in [FlowContent];
    i in [FlowContent, PhrasingContent] with PhrasingContent;
    iframe {
        allow: FeaturePolicy,
        allowfullscreen: Bool,
        allowpaymentrequest: Bool,
        height: usize,
        name: Id,
        referrerpolicy: ReferrerPolicy,
        sandbox: SpacedSet<Sandbox>,
        src: Uri,
        srcdoc: Uri,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent] with FlowContent;
    img {
        alt: String,
        crossorigin: CrossOrigin,
        decoding: ImageDecoding,
        height: usize,
        ismap: Bool,
        sizes: SpacedList<String>, // FIXME it's not really just a string
        src: Uri,
        srcset: String, // FIXME this is much more complicated
        usemap: String, // FIXME should be a fragment starting with '#'
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent];
    input {
        accept: String,
        alt: String,
        autocomplete: String,
        autofocus: Bool,
        capture: String,
        checked: Bool,
        disabled: Bool,
        form: Id,
        formaction: Uri,
        formenctype: FormEncodingType,
        formmethod: FormDialogMethod,
        formnovalidate: Bool,
        formtarget: Target,
        height: isize,
        list: Id,
        max: String,
        maxlength: usize,
        min: String,
        minlength: usize,
        multiple: Bool,
        name: Id,
        pattern: String,
        placeholder: String,
        readonly: Bool,
        required: Bool,
        size: usize,
        spellcheck: Bool,
        src: Uri,
        step: String,
        tabindex: usize,
        type: InputType,
        value: String,
        width: isize,
    } in [FlowContent, FormContent, PhrasingContent];
    ins {
        cite: Uri,
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with FlowContent;
    kbd in [FlowContent, PhrasingContent] with PhrasingContent;
    label {
        for: Id,
        form: Id,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with PhrasingContent;
    main in [FlowContent] with FlowContent;
    map {
        name: Id,
    } in [FlowContent, PhrasingContent] with MapContent;
    mark in [FlowContent, PhrasingContent] with PhrasingContent;
    // TODO the <math> element
    meter {
        value: isize,
        min: isize,
        max: isize,
        low: isize,
        high: isize,
        optimum: isize,
        form: Id,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    nav in [FlowContent, SectioningContent] with FlowContent;
    noscript in [MetadataContent, FlowContent, PhrasingContent] with Node;
    object {
        data: Uri,
        form: Id,
        height: usize,
        name: Id,
        type: Mime,
        typemustmatch: Bool,
        usemap: String, // TODO should be a fragment starting with '#'
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent, FormContent] with param;
    ol {
        reversed: Bool,
        start: isize,
        type: OrderedListType,
    } in [FlowContent] with li;
    output {
        for: SpacedSet<Id>,
        form: Id,
        name: Id,
    } in [FlowContent, PhrasingContent, FormContent] with PhrasingContent;
    p in [FlowContent] with PhrasingContent;
    path { // TODO: support all SVG presentation attributes
        d: String,
        fill: String, // TODO: spec defines value as "none | <color> | <url> [none | <color>]? | context-fill | context-stroke"
    } in [SvgContent];
    pre in [FlowContent] with PhrasingContent;
    progress {
        max: f64,
        value: f64,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    q {
        cite: Uri,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    ruby in [FlowContent, PhrasingContent] with PhrasingContent;
    s in [FlowContent, PhrasingContent] with PhrasingContent;
    samp in [FlowContent, PhrasingContent] with PhrasingContent;
    script {
        async: Bool,
        crossorigin: CrossOrigin,
        defer: Bool,
        integrity: Integrity,
        nomodule: Bool,
        nonce: Nonce,
        src: Uri,
        text: String,
        type: String, // TODO could be an enum
    } in [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] with TextNode;
    section in [FlowContent, SectioningContent] with FlowContent;
    select {
        autocomplete: String,
        autofocus: Bool,
        disabled: Bool,
        form: Id,
        multiple: Bool,
        name: Id,
        required: Bool,
        size: usize,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with SelectContent;
    small in [FlowContent, PhrasingContent] with PhrasingContent;
    span in [FlowContent, PhrasingContent] with PhrasingContent;
    strong in [FlowContent, PhrasingContent] with PhrasingContent;
    sub in [FlowContent, PhrasingContent] with PhrasingContent;
    sup in [FlowContent, PhrasingContent] with PhrasingContent;
    svg {
        xmlns: Uri,
        width: String, // TODO: use
        height: String,
        viewBox: String, // TODO: in fact it is more like SpacedList<usize>
        // Note: this is a bit too permissive, the spec is "<align> <meetOrSlice>?", instead we have "<align | meetOrSlice>"
        preserveAspectRatio: SpacedList<PreserveAspectRatio>,
        transform: String,
        x: String, // TODO: spec defines value as "<length> | <percentage>"
        y: String, // TODO: spec defines value as "<length> | <percentage>"
        width: String, // TODO: spec defines value as "<length> | <percentage> | auto | inherit"
        height: String, // TODO: spec defines value as "<length> | <percentage> | auto | inherit"
    } in [FlowContent, PhrasingContent, EmbeddedContent] with SvgContent; // TODO: should support other SVG nodes
    table in [FlowContent] with TableContent;
    template in [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] with Node;
    textarea {
        autocomplete: OnOff,
        autofocus: Bool,
        cols: usize,
        disabled: Bool,
        form: Id,
        maxlength: usize,
        minlength: usize,
        name: Id,
        placeholder: String,
        readonly: Bool,
        required: Bool,
        rows: usize,
        spellcheck: BoolOrDefault,
        wrap: Wrap,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with TextNode;
    time {
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    ul in [FlowContent] with li;
    var in [FlowContent, PhrasingContent] with PhrasingContent;
    video {
        autoplay: Bool,
        controls: Bool,
        crossorigin: CrossOrigin,
        height: usize,
        loop: Bool,
        muted: Bool,
        preload: Preload,
        playsinline: Bool,
        poster: Uri,
        src: Uri,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with MediaContent;
    wbr in [FlowContent, PhrasingContent];

    // Non-group elements
    area {
        alt: String,
        coords: String, // TODO could perhaps be validated
        download: Bool,
        href: Uri,
        hreflang: LanguageTag,
        ping: SpacedList<Uri>,
        rel: SpacedSet<LinkType>,
        shape: AreaShape,
        target: Target,
    } in [MapContent];
    caption in [TableContent] with FlowContent;
    col {
        span: usize,
    };
    colgroup {
        span: usize,
    } in [TableContent] with col;
    dd in [DescriptionListContent] with FlowContent;
    dt in [DescriptionListContent] with FlowContent;
    figcaption with FlowContent;
    legend with PhrasingContent;
    li {
        value: isize,
    } with FlowContent;
    option {
        disabled: Bool,
        label: String,
        selected: Bool,
        value: String,
    } in [SelectContent] with TextNode;
    optgroup {
        disabled: Bool,
        label: String,
    } in [SelectContent] with option;
    param {
        name: String,
        value: String,
    };
    source {
        src: Uri,
        type: Mime,
    } in [MediaContent];
    summary with PhrasingContent;
    tbody in [TableContent] with tr;
    td {
        colspan: usize,
        headers: SpacedSet<Id>,
        rowspan: usize,
    } in [TableColumnContent] with FlowContent;
    tfoot in [TableContent] with tr;
    th {
        abbr: String,
        colspan: usize,
        headers: SpacedSet<Id>,
        rowspan: usize,
        scope: TableHeaderScope,
    } in [TableColumnContent] with FlowContent;
    thead in [TableContent] with tr;
    tr in [TableContent] with TableColumnContent;
    track {
        default: Bool,
        kind: VideoKind,
        label: String,
        src: Uri,
        srclang: LanguageTag,
    } in [MediaContent];

    // Don't @ me
    blink in [FlowContent, PhrasingContent] with PhrasingContent;
    marquee {
        behavior: String, // FIXME enum
        bgcolor: String, // FIXME colour
        direction: String, // FIXME direction enum
        height: String, // FIXME size
        hspace: String, // FIXME size
        loop: isize,
        scrollamount: usize,
        scrolldelay: usize,
        truespeed: Bool,
        vspace: String, // FIXME size
        width: String, // FIXME size
    } in [FlowContent, PhrasingContent] with PhrasingContent;
}

#[test]
fn test_data_attributes() {
    use crate as typed_html;
    use crate::{dom::DOMTree, html};

    let frag: DOMTree<String> = html!(<div data-id="1234">"Boo!"</div>);

    assert_eq!(r#"<div data-id="1234">Boo!</div>"#, frag.to_string());
}

#[test]
fn test_svg_element() {
    use crate as typed_html;
    use crate::{dom::DOMTree, html};

    let elem: DOMTree<String> = html! {
        <svg id="fas_fa-rss" data-prefix="fas" data-icon="rss"
            class="svg-inline--fa fa-rss fa-w-14" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
            <path fill="currentColor"
                d="M128.081 415.959c0 35.369-28.672 64.041-64.041 64.041S0 451.328 0 415.959s28.672-64.041 64.041-64.041 64.04 28.673 64.04 64.041zm175.66 47.25c-8.354-154.6-132.185-278.587-286.95-286.95C7.656 175.765 0 183.105 0 192.253v48.069c0 8.415 6.49 15.472 14.887 16.018 111.832 7.284 201.473 96.702 208.772 208.772.547 8.397 7.604 14.887 16.018 14.887h48.069c9.149.001 16.489-7.655 15.995-16.79zm144.249.288C439.596 229.677 251.465 40.445 16.503 32.01 7.473 31.686 0 38.981 0 48.016v48.068c0 8.625 6.835 15.645 15.453 15.999 191.179 7.839 344.627 161.316 352.465 352.465.353 8.618 7.373 15.453 15.999 15.453h48.068c9.034-.001 16.329-7.474 16.005-16.504z">
            </path>
        </svg>
    };

    assert_eq!(
        r#"<svg class="fa-rss fa-w-14 svg-inline--fa" id="fas_fa-rss" role="img" viewBox="0 0 448 512" xmlns="http://www.w3.org/2000/svg" data-icon="rss" data-prefix="fas"><path d="M128.081 415.959c0 35.369-28.672 64.041-64.041 64.041S0 451.328 0 415.959s28.672-64.041 64.041-64.041 64.04 28.673 64.04 64.041zm175.66 47.25c-8.354-154.6-132.185-278.587-286.95-286.95C7.656 175.765 0 183.105 0 192.253v48.069c0 8.415 6.49 15.472 14.887 16.018 111.832 7.284 201.473 96.702 208.772 208.772.547 8.397 7.604 14.887 16.018 14.887h48.069c9.149.001 16.489-7.655 15.995-16.79zm144.249.288C439.596 229.677 251.465 40.445 16.503 32.01 7.473 31.686 0 38.981 0 48.016v48.068c0 8.625 6.835 15.645 15.453 15.999 191.179 7.839 344.627 161.316 352.465 352.465.353 8.618 7.373 15.453 15.999 15.453h48.068c9.034-.001 16.329-7.474 16.005-16.504z" fill="currentColor"></path></svg>"#,
        elem.to_string()
    );
}
