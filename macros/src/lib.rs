#![recursion_limit = "32768"]

extern crate proc_macro;

use pom::combinator::*;
use pom::{Error, Parser};
use proc_macro2::{Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use quote::quote;
use std::collections::HashMap;

fn required_children(element: &str) -> &[&str] {
    match element {
        "html" => &["head", "body"],
        "head" => &["title"],
        _ => &[],
    }
}

fn global_attrs(span: Span) -> HashMap<Ident, TokenStream> {
    let mut attrs = HashMap::new();
    let mut insert = |key, value: &str| attrs.insert(Ident::new(key, span), value.parse().unwrap());
    insert("id", "crate::elements::CssId");
    insert("class", "crate::elements::CssClass");
    attrs
}

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
    attributes: HashMap<Ident, TokenTree>,
    children: Vec<Node>,
}

impl Element {
    fn new(name: Ident) -> Self {
        Element {
            name,
            attributes: HashMap::new(),
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
        for (index, child) in self.children.iter().enumerate() {
            match child {
                Node::Element(_) => (),
                _ => panic!(
                    "child #{} of {} must be a {} element",
                    index + 1,
                    &name,
                    req_names[index]
                ),
            }
        }
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
                    #opt_children
                )*
                element
            }
        )
    }
}

fn unit<'a, I: 'a, A: Clone>(value: A) -> Combinator<impl Parser<'a, I, Output = A>> {
    comb(move |_, start| Ok((value.clone(), start)))
}

fn punct<'a>(punct: char) -> Combinator<impl Parser<'a, TokenTree, Output = Punct>> {
    comb(move |input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Punct(p)) if p.as_char() == punct => Ok((p.clone(), start + 1)),
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
    (punct('<') * ident()).map(Element::new)
}

fn attr_value<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = TokenTree>> {
    literal().map(TokenTree::Literal) | ident().map(TokenTree::Ident)
}

fn attr<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = (Ident, TokenTree)>> {
    ident() + (punct('=') * attr_value())
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

fn expand_html(input: &[TokenTree]) -> pom::Result<TokenStream> {
    comb(node).parse(input).map(|el| el.into_token_stream())
}

struct Declare {
    name: Ident,
    attrs: HashMap<Ident, TokenStream>,
    req_children: Vec<Ident>,
    opt_children: Option<TokenStream>,
    traits: Vec<TokenStream>,
}

impl Declare {
    fn new(name: Ident) -> Self {
        Declare {
            attrs: global_attrs(name.span()),
            req_children: Vec::new(),
            opt_children: None,
            traits: Vec::new(),
            name,
        }
    }

    fn into_token_stream(self) -> TokenStream {
        let elem_name = Ident::new(
            &format!("Element_{}", self.name.to_string()),
            self.name.span(),
        );
        let name = self.name.to_string();
        let attr_name: Vec<Ident> = self
            .attrs
            .keys()
            .map(|k| Ident::new(&format!("attr_{}", k.to_string()), k.span()))
            .collect();
        let attr_name_2 = attr_name.clone();
        let attr_name_3 = attr_name.clone();
        let attr_name_str = self.attrs.keys().map(|k| k.to_string());
        let attr_type = self.attrs.values().cloned();
        let req_child_name: Vec<Ident> = self
            .req_children
            .iter()
            .map(|c| Ident::new(&format!("child_{}", c.to_string()), c.span()))
            .collect();
        let req_child_name_2 = req_child_name.clone();
        let req_child_name_3 = req_child_name.clone();
        let req_child_name_4 = req_child_name.clone();
        let req_child_type: Vec<Ident> = self
            .req_children
            .iter()
            .map(|c| Ident::new(&format!("Element_{}", c.to_string()), c.span()))
            .collect();
        let req_child_type_2 = req_child_type.clone();
        let construct_children = match self.opt_children {
            Some(_) => quote!(children: Vec::new()),
            None => TokenStream::new(),
        };
        let print_opt_children = if self.opt_children.is_some() {
            quote!(for child in &self.children {
                child.fmt(f)?;
            })
        } else {
            TokenStream::new()
        };
        let print_children = if req_child_name_2.is_empty() {
            if self.opt_children.is_some() {
                quote!(if self.children.is_empty() {
                    write!(f, "/>")
                } else {
                    write!(f, ">")?;
                    #print_opt_children
                    write!(f, "</{}>", #name)
                })
            } else {
                quote!(write!(f, "/>"))
            }
        } else {
            quote!(
                write!(f, ">")?;
                #(
                    self.#req_child_name_2.fmt(f)?;
                )*
                #print_opt_children
                write!(f, "</{}>", #name)
            )
        };
        let children = match self.opt_children {
            Some(child_constraint) => quote!(pub children: Vec<Box<#child_constraint>>),
            None => TokenStream::new(),
        };
        let trait_for = std::iter::repeat(elem_name.clone());
        let trait_name = self.traits.into_iter();

        quote!(
            pub struct #elem_name {
                #( pub #attr_name: Option<#attr_type>, )*
                #( pub #req_child_name: #req_child_type, )*
                #children
            }

            impl #elem_name {
                pub fn new(#(#req_child_name_3: #req_child_type_2),*) -> Self {
                    #elem_name {
                        #( #attr_name_2: None, )*
                        #( #req_child_name_4, )*
                        #construct_children
                    }
                }
            }

            impl Node for #elem_name {}
            impl Element for #elem_name {}
            #(
                impl #trait_name for #trait_for {}
            )*

            impl std::fmt::Display for #elem_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, "<{}", #name);
                    #(
                        if let Some(ref value) = self.#attr_name_3 {
                            write!(f, " {}={:?}", #attr_name_str, value.to_string())?;
                        }
                    )*
                    #print_children
                }
            }
        )
    }
}

fn type_spec<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = TokenStream>> {
    let valid = ident().map(TokenTree::Ident)
        | punct(':').map(TokenTree::Punct)
        | punct('<').map(TokenTree::Punct)
        | punct('>').map(TokenTree::Punct)
        | punct('&').map(TokenTree::Punct)
        | punct('\'').map(TokenTree::Punct);
    valid.repeat(1..).map(|tokens| {
        let mut stream = TokenStream::new();
        stream.extend(tokens);
        stream
    })
}

fn declare_attrs<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Vec<(Ident, TokenStream)>>>
{
    group().map(|group: Group| {
        let attr = ident() - punct(':') + type_spec();
        let input: Vec<TokenTree> = group.stream().into_iter().collect();
        let result = attr.repeat(0..).parse(&input);
        result.unwrap()
    })
}

fn declare_children<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Vec<Ident>>> {
    group().map(|group: Group| {
        let input: Vec<TokenTree> = group.stream().into_iter().collect();
        let children = (ident() - punct(',').opt()).repeat(0..);
        let result = children.parse(&input);
        result.unwrap()
    })
}

fn declare_traits<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Vec<TokenStream>>> {
    group().map(|group: Group| {
        let input: Vec<TokenTree> = group.stream().into_iter().collect();
        let traits = (type_spec() - punct(',').opt()).repeat(0..);
        let result = traits.parse(&input);
        result.unwrap()
    })
}

fn declare<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Declare>> {
    (ident() + declare_attrs() + declare_children() + declare_traits().opt() + type_spec().opt())
        .map(|((((name, attrs), children), traits), child_type)| {
            let mut declare = Declare::new(name);
            for (key, value) in attrs {
                declare.attrs.insert(key, value);
            }
            for child in children {
                declare.req_children.push(child);
            }
            declare.opt_children = child_type;
            declare.traits = traits.unwrap_or_default();
            declare
        })
}

fn expand_declare(input: &[TokenTree]) -> pom::Result<TokenStream> {
    declare().parse(input).map(|decl| decl.into_token_stream())
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = expand_html(&input);
    match result {
        Err(error) => panic!("error: {:?}", error),
        Ok(ts) => ts.into(),
    }
}

#[proc_macro]
pub fn declare_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let input: Vec<TokenTree> = input.into_iter().collect();
    let result = expand_declare(&input);
    match result {
        Err(error) => panic!("error: {:?}", error),
        Ok(ts) => ts.into(),
    }
}
