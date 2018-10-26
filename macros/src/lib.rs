#![recursion_limit = "32768"]

extern crate proc_macro;

use pom::combinator::*;
use pom::{Error, Parser};
use proc_macro2::{Group, Ident, Literal, TokenStream, TokenTree};
use quote::quote;
use std::collections::HashMap;

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
            Node::Text(_) => panic!("top level must be an element"),
            Node::Block(_) => panic!("top level must be an element"),
        }
    }

    fn into_child_stream(self) -> TokenStream {
        match self {
            Node::Element(el) => {
                let el = el.into_token_stream();
                quote!(
                    element.append_child(#el);
                )
            }
            Node::Text(tx) => quote!(
                element.append_child(typed_html::Node::Text(#tx.to_string()));
            ),
            Node::Block(group) => quote!({
                let iter = #group.into_iter();
                for child in iter {
                    element.append_child(child);
                }
            }),
        }
    }
}

#[derive(Clone)]
struct Element {
    name: String,
    attributes: HashMap<String, TokenTree>,
    children: Vec<Node>,
}

impl Element {
    fn new(name: String) -> Self {
        Element {
            name,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let keys: Vec<_> = self.attributes.keys().cloned().collect();
        let values: Vec<TokenTree> = self.attributes.values().cloned().collect();
        let children = self.children.into_iter().map(Node::into_child_stream);
        quote!(
            {
                let mut element = typed_html::Element::new(#name);
                #(
                    element.set_attr(#keys, #values.to_string());
                )*
                #(#children)*
                typed_html::Node::Element(element)
            }
        )
    }
}

fn unit<'a, I: 'a, A: Clone>(value: A) -> Combinator<impl Parser<'a, I, Output = A>> {
    comb(move |_, start| Ok((value.clone(), start)))
}

fn punct<'a>(punct: char) -> Combinator<impl Parser<'a, TokenTree, Output = ()>> {
    comb(move |input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Punct(p)) if p.as_char() == punct => Ok(((), start + 1)),
        _ => Err(Error::Mismatch {
            message: format!("expected {:?}", punct),
            position: start,
        }),
    })
}

fn ident<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Ident>> {
    comb(|input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Ident(i)) => Ok((i.clone(), start + 1)),
        _ => Err(Error::Mismatch {
            message: "expected identifier".to_string(),
            position: start,
        }),
    })
}

fn ident_match<'a>(name: String) -> Combinator<impl Parser<'a, TokenTree, Output = ()>> {
    comb(move |input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Ident(i)) => {
            if *i == name {
                Ok(((), start + 1))
            } else {
                Err(Error::Mismatch {
                    message: format!("expected '</{}>', found '</{}>'", name, i.to_string()),
                    position: start,
                })
            }
        }
        _ => Err(Error::Mismatch {
            message: "expected identifier".to_string(),
            position: start,
        }),
    })
}

fn literal<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Literal>> {
    comb(|input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Literal(l)) => Ok((l.clone(), start + 1)),
        _ => Err(Error::Mismatch {
            message: "expected literal".to_string(),
            position: start,
        }),
    })
}

fn group<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Group>> {
    comb(|input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Group(g)) => Ok((g.clone(), start + 1)),
        _ => Err(Error::Mismatch {
            message: "expected group".to_string(),
            position: start,
        }),
    })
}

fn element_start<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Element>> {
    (punct('<') * ident()).map(|i| Element::new(i.to_string()))
}

fn attr_value<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = TokenTree>> {
    literal().map(TokenTree::Literal) | ident().map(TokenTree::Ident)
}

fn attr<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = (String, TokenTree)>> {
    ident().map(|i| i.to_string()) + (punct('=') * attr_value())
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

fn element_close<'a>(name: String) -> Combinator<impl Parser<'a, TokenTree, Output = ()>> {
    // TODO make this return an error message containing the tag name
    punct('<') * punct('/') * ident_match(name) * punct('>')
}

fn element_with_children<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Element>> {
    (element_open() + comb(node).repeat(0..)).map(|(mut el, children)| {
        el.children.extend(children.into_iter());
        el
    }) >> |el: Element| element_close(el.name.clone()).expect("closing tag") * unit(el)
}

fn node(input: &[TokenTree], start: usize) -> pom::Result<(Node, usize)> {
    (element_single().map(Node::Element)
        | element_with_children().map(Node::Element)
        | literal().map(Node::Text)
        | group().map(Node::Block))
    .0
    .parse(input, start)
}

fn macro_expand(input: &[TokenTree]) -> pom::Result<TokenStream> {
    comb(node).parse(input).map(|el| el.into_token_stream())
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = macro_expand(&input);
    match result {
        Err(error) => panic!("error: {:?}", error),
        Ok(ts) => ts.into(),
    }
}
