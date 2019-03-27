//! DOM and virtual DOM types.

use std::fmt::Display;
use std::marker::PhantomData;

use crate::OutputType;
use crate::elements::{FlowContent, PhrasingContent};
use htmlescape::encode_minimal;

/// A boxed DOM tree, as returned from the `html!` macro.
///
/// # Examples
///
/// ```
/// # use typed_html::html;
/// # use typed_html::dom::DOMTree;
/// # fn main() {
/// let tree: DOMTree<String> = html!(
///     <div class="hello">
///         <p>"Hello Joe!"</p>
///     </div>
/// );
/// let rendered_tree: String = tree.to_string();
/// # }
/// ```
pub type DOMTree<T> = Box<Node<T>>;

/// An untyped representation of an HTML node.
///
/// This structure is designed to be easily walked in order to render a DOM tree
/// or diff against an existing tree. It's the stringly typed version of
/// [`Node`][Node].
///
/// It can be constructed from any ['Node'][Node]:
///
/// ```no_compile
/// html!(
///     <p>"But how does she "<em>"eat?"</em></p>
/// ).vnode()
/// ```
///
/// [Node]: trait.Node.html
pub enum VNode<'a, T: OutputType + 'a> {
    Text(&'a str),
    UnsafeText(&'a str),
    Element(VElement<'a, T>),
}

/// An untyped representation of an HTML element.
pub struct VElement<'a, T: OutputType + 'a> {
    pub name: &'static str,
    pub attributes: Vec<(&'static str, String)>,
    pub events: &'a mut T::Events,
    pub children: Vec<VNode<'a, T>>,
}

/// Trait for rendering a typed HTML node.
///
/// All [HTML elements][elements] implement this, in addition to
/// [`TextNode`][TextNode].
///
/// It implements [`Display`][Display] for rendering to strings, and the
/// [`vnode()`][vnode] method can be used to render a virtual DOM structure.
///
/// [Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
/// [TextNode]: struct.TextNode.html
/// [elements]: ../elements/index.html
/// [vnode]: #tymethod.vnode
pub trait Node<T: OutputType>: Display {
    /// Render the node into a [`VNode`][VNode] tree.
    ///
    /// [VNode]: enum.VNode.html
    fn vnode(&mut self) -> VNode<T>;
}

impl<T> IntoIterator for Box<dyn Node<T>>
where
    T: OutputType,
{
    type Item = Box<dyn Node<T>>;
    type IntoIter = std::vec::IntoIter<Box<dyn Node<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

/// Trait for querying a typed HTML element.
///
/// All [HTML elements][elements] implement this.
///
/// [elements]: ../elements/index.html
pub trait Element<T: OutputType>: Node<T> {
    /// Get the name of the element.
    fn name() -> &'static str;
    /// Get a list of the attribute names for this element.
    ///
    /// This includes only the typed attributes, not any `data-` attributes
    /// defined on this particular element instance.
    ///
    /// This is probably not useful unless you're the `html!` macro.
    fn attribute_names() -> &'static [&'static str];
    /// Get a list of the element names of required children for this element.
    ///
    /// This is probably not useful unless you're the `html!` macro.
    fn required_children() -> &'static [&'static str];
    /// Get a list of the defined attribute pairs for this element.
    ///
    /// This will convert attribute values into strings and return a vector of
    /// key/value pairs.
    fn attributes(&self) -> Vec<(&'static str, String)>;
}

/// An HTML text node.
pub struct TextNode<T: OutputType>(String, PhantomData<T>);

/// Macro for creating text nodes.
///
/// Returns a boxed text node of type `Box<TextNode>`.
///
/// These can be created inside the `html!` macro directly by using string
/// literals. This macro is useful for creating text macros inside code blocks.
///
/// # Examples
///
/// ```no_compile
/// html!(
///     <p>{ text!("Hello Joe!") }</p>
/// )
/// ```
///
/// ```no_compile
/// html!(
///     <p>{ text!("Hello {}!", "Robert") }</p>
/// )
/// ```
#[macro_export]
macro_rules! text {
    ($t:expr) => {
        Box::new($crate::dom::TextNode::new($t))
    };
    ($format:tt, $($tail:tt),*) => {
        Box::new($crate::dom::TextNode::new(format!($format, $($tail),*)))
    };
}

/// An unsafe HTML text node.
/// This is like TextNode, but no escaping will be performed when this node is displayed.
pub struct UnsafeTextNode<T: OutputType>(String, PhantomData<T>);

/// Macro for creating unescaped text nodes.
///
/// Returns a boxed text node of type `Box<UnsafeTextNode>`.
///
/// This macro is useful for creating text macros inside code blocks that contain HTML
/// that you do not want to be escaped. For example, if some other process renders Markdown
/// to an HTML string and you want embed that HTML string in a typed-html template,
/// you may want to avoid escaping the tags in that HTML string.
///
/// # Examples
///
/// ```no_compile
/// html!(
///     <p>{ unsafe_text!("Hello Joe!") }</p>
/// )
/// ```
///
/// ```no_compile
/// html!(
///     <p>{ unsafe_text!("Hello {}!", "Robert") }</p>
/// )
/// ```
///
/// ```no_compile
/// html!(
///     <p>{ unsafe_text!("<div>this text renders unescaped html</div>") }</p>
/// )
/// ```
#[macro_export]
macro_rules! unsafe_text {
    ($t:expr) => {
        Box::new($crate::dom::UnsafeTextNode::new($t))
    };
    ($format:tt, $($tail:tt),*) => {
        Box::new($crate::dom::UnsafeTextNode::new(format!($format, $($tail),*)))
    };
}

impl<T: OutputType> TextNode<T> {
    /// Construct a text node.
    ///
    /// The preferred way to construct a text node is with the [`text!()`][text]
    /// macro.
    ///
    /// [text]: ../macro.text.html
    pub fn new<S: Into<String>>(s: S) -> Self {
        TextNode(s.into(), PhantomData)
    }
}

impl<T: OutputType> Display for TextNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&encode_minimal(&self.0))
    }
}

impl<T: OutputType> Node<T> for TextNode<T> {
    fn vnode(&'_ mut self) -> VNode<'_, T> {
        VNode::Text(&self.0)
    }
}

impl<T: OutputType> IntoIterator for TextNode<T> {
    type Item = TextNode<T>;
    type IntoIter = std::vec::IntoIter<TextNode<T>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl<T: OutputType> IntoIterator for Box<TextNode<T>> {
    type Item = Box<TextNode<T>>;
    type IntoIter = std::vec::IntoIter<Box<TextNode<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl<T: OutputType> FlowContent<T> for TextNode<T> {}
impl<T: OutputType> PhrasingContent<T> for TextNode<T> {}

impl<T: OutputType> UnsafeTextNode<T> {
    /// Construct a unsafe text node.
    ///
    /// The preferred way to construct a unsafe text node is with the [`unsafe_text!()`][unsafe_text]
    /// macro.
    ///
    /// [unsafe_text]: ../macro.unsafe_text.html
    pub fn new<S: Into<String>>(s: S) -> Self {
        UnsafeTextNode(s.into(), PhantomData)
    }
}

impl<T: OutputType> Display for UnsafeTextNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

impl<T: OutputType> Node<T> for UnsafeTextNode<T> {
    fn vnode(&'_ mut self) -> VNode<'_, T> {
        VNode::UnsafeText(&self.0)
    }
}

impl<T: OutputType> IntoIterator for UnsafeTextNode<T> {
    type Item = UnsafeTextNode<T>;
    type IntoIter = std::vec::IntoIter<UnsafeTextNode<T>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl<T: OutputType> IntoIterator for Box<UnsafeTextNode<T>> {
    type Item = Box<UnsafeTextNode<T>>;
    type IntoIter = std::vec::IntoIter<Box<UnsafeTextNode<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl<T: OutputType> FlowContent<T> for UnsafeTextNode<T> {}
impl<T: OutputType> PhrasingContent<T> for UnsafeTextNode<T> {}
