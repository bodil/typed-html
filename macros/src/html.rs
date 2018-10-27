use pom::combinator::*;
use pom::Parser;
use proc_macro2::{Group, Ident, Literal, TokenStream, TokenTree};
use quote::quote;
use std::collections::BTreeMap;

use crate::config::required_children;
use crate::parser::*;

#[derive(Clone)]
enum Node {
    Element(Element),
    Text(Literal),
    Block(Group),
}

impl Node {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Node::Element(el) => el.into_token_stream(),
            Node::Text(text) => quote!(typed_html::elements::TextNode::new(#text.to_string())),
            Node::Block(_) => panic!("cannot have a block in this position"),
        }
    }

    fn into_child_stream(self) -> TokenStream {
        match self {
            Node::Element(el) => {
                let el = el.into_token_stream();
                quote!(
                    element.children.push(Box::new(#el));
                )
            }
            tx @ Node::Text(_) => {
                let tx = tx.into_token_stream();
                quote!(
                    element.children.push(Box::new(#tx));
                )
            }
            Node::Block(group) => quote!({
                let iter = #group.into_iter();
                for child in iter {
                    element.children.push(Box::new(child));
                }
            }),
        }
    }
}

#[derive(Clone)]
struct Element {
    name: Ident,
    attributes: BTreeMap<Ident, TokenTree>,
    children: Vec<Node>,
}

fn extract_data_attrs(attrs: &mut BTreeMap<Ident, TokenTree>) -> BTreeMap<String, TokenTree> {
    let mut data = BTreeMap::new();
    let keys: Vec<Ident> = attrs.keys().cloned().collect();
    for key in keys {
        let key_name = key.to_string();
        let prefix = "data_";
        if key_name.starts_with(prefix) {
            let value = attrs.remove(&key).unwrap();
            data.insert(key_name[prefix.len()..].to_string(), value);
        }
    }
    data
}

impl Element {
    fn new(name: Ident) -> Self {
        Element {
            name,
            attributes: BTreeMap::new(),
            children: Vec::new(),
        }
    }

    fn into_token_stream(mut self) -> TokenStream {
        let name = self.name;
        let name_str = name.to_string();
        let typename = Ident::new(&format!("Element_{}", &name_str), name.span());
        let req_names = required_children(&name_str);
        if req_names.len() > self.children.len() {
            panic!(
                "<{}> requires {} children but found only {}",
                name_str,
                req_names.len(),
                self.children.len()
            );
        }
        let data_attrs = extract_data_attrs(&mut self.attributes);
        let data_keys = data_attrs.keys().cloned();
        let data_values = data_attrs.values().cloned();
        let keys: Vec<_> = self
            .attributes
            .keys()
            .map(|key| Ident::new(&format!("attr_{}", key), key.span()))
            .collect();
        let values: Vec<TokenTree> = self.attributes.values().cloned().collect();
        let opt_children = self
            .children
            .split_off(req_names.len())
            .into_iter()
            .map(Node::into_child_stream);
        let req_children = self.children.into_iter().map(Node::into_token_stream);
        quote!(
            {
                let mut element = typed_html::elements::#typename::new(
                    #({ #req_children }),*
                );
                #(
                    element.#keys = Some(#values.into());
                )*
                #(
                    element.data_attributes.insert(#data_keys.into(), #data_values.into());
                )*
                #(
                    #opt_children
                )*
                element
            }
        )
    }
}

fn element_start<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Element>> {
    (punct('<') * html_ident()).map(Element::new)
}

fn attr_value<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = TokenTree>> {
    literal().map(TokenTree::Literal) | ident().map(TokenTree::Ident)
}

fn attr<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = (Ident, TokenTree)>> {
    html_ident() + (punct('=') * attr_value())
}

fn element_with_attrs<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Element>> {
    (element_start() + attr().repeat(0..)).map(|(mut el, attrs)| {
        for (name, value) in attrs {
            el.attributes.insert(name, value);
        }
        el
    })
}

fn element_single<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Element>> {
    element_with_attrs() - punct('/') - punct('>')
}

fn element_open<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Element>> {
    element_with_attrs() - punct('>')
}

fn element_close<'a>(name: &str) -> Combinator<impl Parser<'a, TokenTree, Output = ()>> {
    let name = name.to_lowercase();
    // TODO make this return an error message containing the tag name
    punct('<') * punct('/') * ident_match(name) * punct('>').discard()
}

fn element_with_children<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Element>> {
    (element_open() + comb(node).repeat(0..)).map(|(mut el, children)| {
        el.children.extend(children.into_iter());
        el
    }) >> |el: Element| element_close(&el.name.to_string()).expect("closing tag") * unit(el)
}

fn node(input: &[TokenTree], start: usize) -> pom::Result<(Node, usize)> {
    (element_single().map(Node::Element)
        | element_with_children().map(Node::Element)
        | literal().map(Node::Text)
        | group().map(Node::Block))
    .0
    .parse(input, start)
}

pub fn expand_html(input: &[TokenTree]) -> pom::Result<TokenStream> {
    comb(node).parse(input).map(|el| el.into_token_stream())
}
