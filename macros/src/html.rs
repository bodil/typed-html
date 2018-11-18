use proc_macro2::{Delimiter, Group, Ident, Literal, TokenStream, TokenTree};
use quote::{quote, quote_spanned};

use config::{required_children, ATTR_EVENTS};
use error::ParseError;
use ident;
use lexer::{Lexer, Token};
use map::StringyMap;
use parser::grammar;

use std::iter::FromIterator;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(Literal),
    Block(Group),
}

impl Node {
    pub fn into_token_stream(self) -> Result<TokenStream, TokenStream> {
        match self {
            Node::Element(el) => el.into_token_stream(),
            Node::Text(text) => {
                let text = TokenTree::Literal(text);
                Ok(quote!(Box::new(typed_html::dom::TextNode::new(#text.to_string()))))
            }
            Node::Block(_) => panic!("cannot have a block in this position"),
        }
    }

    fn into_child_stream(self) -> Result<TokenStream, TokenStream> {
        match self {
            Node::Element(el) => {
                let el = el.into_token_stream()?;
                Ok(quote!(
                    element.children.push(#el);
                ))
            }
            tx @ Node::Text(_) => {
                let tx = tx.into_token_stream()?;
                Ok(quote!(
                    element.children.push(#tx);
                ))
            }
            Node::Block(group) => {
                let group: TokenTree = group.into();
                Ok(quote!(
                    for child in #group.into_iter() {
                        element.children.push(child);
                    }
                ))
            }
        }
    }
}

#[derive(Clone)]
pub struct Element {
    pub name: Ident,
    pub attributes: StringyMap<Ident, TokenTree>,
    pub children: Vec<Node>,
}

fn extract_data_attrs(attrs: &mut StringyMap<Ident, TokenTree>) -> StringyMap<String, TokenTree> {
    let mut data = StringyMap::new();
    let keys: Vec<Ident> = attrs.keys().cloned().collect();
    for key in keys {
        let key_name = key.to_string();
        let prefix = "data_";
        if key_name.starts_with(prefix) {
            let value = attrs.remove(&key).unwrap();
            data.insert(format!("data-{}", &key_name[prefix.len()..]), value);
        }
    }
    data
}

fn extract_event_handlers(
    attrs: &mut StringyMap<Ident, TokenTree>,
) -> StringyMap<Ident, TokenTree> {
    let mut events = StringyMap::new();
    let keys: Vec<Ident> = attrs.keys().cloned().collect();
    for key in keys {
        let key_name = key.to_string();
        let prefix = "on";
        if key_name.starts_with(prefix) {
            let event_name = &key_name[prefix.len()..];
            if ATTR_EVENTS.binary_search(&event_name).is_ok() {
                let value = attrs.remove(&key).unwrap();
                events.insert(ident::new_raw(event_name, key.span()), value);
            }
        }
    }
    events
}

fn process_value(value: &TokenTree) -> TokenStream {
    match value {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
            let content = g.stream();
            quote!( [ #content ] )
        }
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
            let content = g.stream();
            quote!( ( #content ) )
        }
        v => TokenStream::from_iter(vec![v.clone()]),
    }
}

fn is_string_literal(literal: &Literal) -> bool {
    // This is the worst API
    literal.to_string().starts_with('"')
}

impl Element {
    fn into_token_stream(mut self) -> Result<TokenStream, TokenStream> {
        let name = self.name;
        let name_str = name.to_string();
        let typename: TokenTree = Ident::new(&name_str, name.span()).into();
        let req_names = required_children(&name_str);
        if req_names.len() > self.children.len() {
            let span = name.span();
            let error = format!(
                "<{}> requires {} children but there are only {}",
                name_str,
                req_names.len(),
                self.children.len()
            );
            return Err(quote_spanned! {span=>
                compile_error! { #error }
            });
        }
        let events = extract_event_handlers(&mut self.attributes);
        let data_attrs = extract_data_attrs(&mut self.attributes);
        let attrs = self.attributes.iter().map(|(key, value)| {
            (
                key.to_string(),
                TokenTree::Ident(ident::new_raw(&key.to_string(), key.span())),
                value,
            )
        });
        let opt_children = self
            .children
            .split_off(req_names.len())
            .into_iter()
            .map(Node::into_child_stream)
            .collect::<Result<Vec<TokenStream>, TokenStream>>()?;
        let req_children = self
            .children
            .into_iter()
            .map(Node::into_token_stream)
            .collect::<Result<Vec<TokenStream>, TokenStream>>()?;

        let mut body = TokenStream::new();
        for (attr_str, key, value) in attrs {
            match value {
                TokenTree::Literal(l) if is_string_literal(l) => {
                    let value = value.clone();
                    let tag_name: TokenTree = Literal::string(&name_str).into();
                    let attr_str: TokenTree = Literal::string(&attr_str).into();
                    let span = value.span();
                    let pos = format!(
                        "{}:{}:{}",
                        span.unstable().source_file().path().to_str().unwrap_or("unknown"),
                        span.unstable().start().line,
                        span.unstable().start().column
                    );
                    let pos_str: TokenTree = Literal::string(&pos).into();
                    body.extend(quote!(
                        element.attrs.#key = Some(#value.parse().unwrap_or_else(|err| {
                            eprintln!("ERROR: {}: <{} {}={:?}> failed to parse attribute value: {}",
                                      #pos_str, #tag_name, #attr_str, #value, err);
                            panic!("failed to parse string literal");
                        }));
                    ));
                }
                value => {
                    let value = process_value(value);
                    body.extend(quote!(
                        element.attrs.#key = Some(std::convert::Into::into(#value));
                    ));
                }
            }
        }
        for (key, value) in data_attrs
            .iter()
            .map(|(k, v)| (TokenTree::from(Literal::string(&k)), v.clone()))
        {
            body.extend(quote!(
                element.data_attributes.push((#key, #value.into()));
            ));
        }
        body.extend(opt_children);

        for (key, value) in events.iter() {
            let key = TokenTree::Ident(key.clone());
            let value = process_value(value);
            body.extend(quote!(
                element.events.#key = Some(typed_html::events::IntoEventHandler::into_event_handler(#value));
            ));
        }

        let mut args = TokenStream::new();
        for arg in req_children {
            args.extend(quote!( #arg, ));
        }

        Ok(quote!(
            {
                let mut element = typed_html::elements::#typename::new(#args);
                #body
                Box::new(element)
            }
        ))
    }
}

// FIXME report a decent error when the macro contains multiple top level elements
pub fn expand_html(input: &[Token]) -> Result<Node, ParseError> {
    grammar::NodeParser::new().parse(Lexer::new(input))
}
