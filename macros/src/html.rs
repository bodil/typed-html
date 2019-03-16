use proc_macro2::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};

use crate::config::required_children;
use crate::error::ParseError;
use crate::ident;
use crate::lexer::{to_stream, Lexer, Token};
use crate::map::StringyMap;
use crate::parser::grammar;

use std::iter::FromIterator;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(Literal),
    Block(Group),
}

impl Node {
    pub fn into_token_stream(self, ty: &Option<Vec<Token>>) -> Result<TokenStream, TokenStream> {
        match self {
            Node::Element(el) => el.into_token_stream(ty),
            Node::Text(text) => {
                let text = TokenTree::Literal(text);
                Ok(quote!(Box::new(typed_html::dom::TextNode::new(#text.to_string()))))
            }
            Node::Block(group) => {
                let span = group.span();
                let error =
                    "you cannot use a block as a top level element or a required child element";
                Err(quote_spanned! { span=>
                    compile_error! { #error }
                })
            }
        }
    }

    fn into_child_stream(self, ty: &Option<Vec<Token>>) -> Result<TokenStream, TokenStream> {
        match self {
            Node::Element(el) => {
                let el = el.into_token_stream(ty)?;
                Ok(quote!(
                    element.children.push(#el);
                ))
            }
            tx @ Node::Text(_) => {
                let tx = tx.into_token_stream(ty)?;
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

    pub fn into_dodrio_token_stream(
        self,
        bump: &Ident,
        is_req_child: bool,
    ) -> Result<TokenStream, TokenStream> {
        match self {
            Node::Element(el) => el.into_dodrio_token_stream(bump, is_req_child),
            Node::Text(text) => {
                let text = TokenTree::Literal(text);
                Ok(quote!(dodrio::builder::text(#text)))
            }
            Node::Block(group) => {
                let group: TokenTree = group.into();
                Ok(quote!(
                    #group
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
            data.insert(key_name[prefix.len()..].to_string(), value);
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
            let value = attrs.remove(&key).unwrap();
            events.insert(ident::new_raw(event_name, key.span()), value);
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

fn stringify_ident(ident: &Ident) -> String {
    let s = ident.to_string();
    if s.starts_with("r#") {
        s[2..].to_string()
    } else {
        s
    }
}

impl Element {
    fn into_token_stream(mut self, ty: &Option<Vec<Token>>) -> Result<TokenStream, TokenStream> {
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
            .map(|node| node.into_child_stream(ty))
            .collect::<Result<Vec<TokenStream>, TokenStream>>()?;
        let req_children = self
            .children
            .into_iter()
            .map(|node| node.into_token_stream(ty))
            .collect::<Result<Vec<TokenStream>, TokenStream>>()?;

        let mut body = TokenStream::new();

        for (attr_str, key, value) in attrs {
            match value {
                TokenTree::Literal(lit) if is_string_literal(lit) => {
                    let mut eprintln_msg = "ERROR: ".to_owned();
                    #[cfg(can_show_location_of_runtime_parse_error)]
                    {
                        let span = lit.span();
                        eprintln_msg += &format!(
                            "{}:{}:{}: ",
                            span.unstable()
                                .source_file()
                                .path()
                                .to_str()
                                .unwrap_or("unknown"),
                            span.unstable().start().line,
                            span.unstable().start().column
                        );
                    }
                    eprintln_msg += &format!(
                        "<{} {}={}> failed to parse attribute value: {{}}",
                        name_str, attr_str, lit,
                    );
                    #[cfg(not(can_show_location_of_runtime_parse_error))]
                    {
                        eprintln_msg += "\nERROR: rebuild with nightly to print source location";
                    }

                    body.extend(quote!(
                        element.attrs.#key = Some(#lit.parse().unwrap_or_else(|err| {
                            eprintln!(#eprintln_msg, err);
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
            if ty.is_none() {
                let mut err = quote_spanned! { key.span() =>
                    compile_error! { "when using event handlers, you must declare the output type inside the html! macro" }
                };
                let hint = quote_spanned! { Span::call_site() =>
                    compile_error! { "for example: change html!(<div>...</div>) to html!(<div>...</div> : String)" }
                };
                err.extend(hint);
                return Err(err);
            }
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

        let mut type_annotation = TokenStream::new();
        if let Some(ty) = ty {
            let type_var = to_stream(ty.clone());
            type_annotation.extend(quote!(: typed_html::elements::#typename<#type_var>));
        }

        Ok(quote!(
            {
                let mut element #type_annotation = typed_html::elements::#typename::new(#args);
                #body
                Box::new(element)
            }
        ))
    }

    fn into_dodrio_token_stream(
        mut self,
        bump: &Ident,
        is_req_child: bool,
    ) -> Result<TokenStream, TokenStream> {
        let name = self.name;
        let name_str = stringify_ident(&name);
        let typename: TokenTree = Ident::new(&name_str, name.span()).into();
        let tag_name = TokenTree::from(Literal::string(&name_str));
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
            .map(|node| node.into_dodrio_token_stream(bump, false))
            .collect::<Result<Vec<TokenStream>, TokenStream>>()?;
        let req_children = self
            .children
            .into_iter()
            .map(|node| node.into_dodrio_token_stream(bump, true))
            .collect::<Result<Vec<TokenStream>, TokenStream>>()?;

        let mut set_attrs = TokenStream::new();

        for (attr_str, key, value) in attrs {
            match value {
                TokenTree::Literal(lit) if is_string_literal(lit) => {
                    let mut eprintln_msg = "ERROR: ".to_owned();
                    #[cfg(can_show_location_of_runtime_parse_error)]
                    {
                        let span = lit.span();
                        eprintln_msg += &format!(
                            "{}:{}:{}: ",
                            span.unstable()
                                .source_file()
                                .path()
                                .to_str()
                                .unwrap_or("unknown"),
                            span.unstable().start().line,
                            span.unstable().start().column
                        );
                    }
                    eprintln_msg += &format!(
                        "<{} {}={}> failed to parse attribute value: {{}}",
                        name_str, attr_str, lit,
                    );
                    #[cfg(not(can_show_location_of_runtime_parse_error))]
                    {
                        eprintln_msg += "\nERROR: rebuild with nightly to print source location";
                    }

                    set_attrs.extend(quote!(
                        element.attrs.#key = Some(#lit.parse().unwrap_or_else(|err| {
                            eprintln!(#eprintln_msg, err);
                            panic!("failed to parse string literal");
                        }));
                    ));
                }
                value => {
                    let value = process_value(value);
                    set_attrs.extend(quote!(
                        element.attrs.#key = Some(std::convert::Into::into(#value));
                    ));
                }
            }
        }

        let mut builder = TokenStream::new();
        builder.extend(quote!(
            dodrio::builder::ElementBuilder::new(#bump, #tag_name)
        ));

        for (key, _) in self.attributes.iter() {
            let key_str = TokenTree::from(Literal::string(&stringify_ident(key)));
            builder.extend(quote!(
                .attr(#key_str, dodrio::bumpalo::format!(in &#bump, "{}",
                      element.attrs.#key.unwrap()).into_bump_str())
            ));
        }

        for (key, value) in data_attrs
            .iter()
            .map(|(k, v)| (TokenTree::from(Literal::string(&k)), v.clone()))
        {
            builder.extend(quote!(
                .attr(#key, #value.into())
            ));
        }

        for (key, value) in events.iter() {
            let key = TokenTree::from(Literal::string(&stringify_ident(key)));
            let value = process_value(value);
            builder.extend(quote!(
                .on(#key, #value)
            ));
        }

        let mut make_req_children = TokenStream::new();
        let mut arg_list = Vec::new();
        let mut req_nodes = Vec::new();
        for (index, child) in req_children.into_iter().enumerate() {
            let req_child = TokenTree::from(Ident::new(
                &format!("req_child_{}", index),
                Span::call_site(),
            ));
            let child_node = TokenTree::from(Ident::new(
                &format!("child_node_{}", index),
                Span::call_site(),
            ));
            make_req_children.extend(quote!(
                let (#req_child, #child_node) = #child;
            ));
            builder.extend(quote!(
                .child(#child_node)
            ));
            arg_list.push(req_child);
            req_nodes.push(child_node);
        }

        for child in opt_children {
            builder.extend(quote!(
                .child(#child)
            ));
        }

        builder.extend(quote!(
            .finish()
        ));

        if is_req_child {
            builder = quote!(
                (element, #builder)
            );
        }

        let mut args = TokenStream::new();
        for arg in arg_list {
            args.extend(quote!( #arg, ));
        }

        Ok(quote!(
            {
                #make_req_children
                let mut element: typed_html::elements::#typename<typed_html::output::dodrio::Dodrio> = typed_html::elements::#typename::new(#args);
                #set_attrs
                #builder
            }
        ))
    }
}

// FIXME report a decent error when the macro contains multiple top level elements
pub fn expand_html(input: &[Token]) -> Result<(Node, Option<Vec<Token>>), ParseError> {
    grammar::NodeWithTypeParser::new().parse(Lexer::new(input))
}

pub fn expand_dodrio(input: &[Token]) -> Result<(Ident, Node), ParseError> {
    grammar::NodeWithBumpParser::new().parse(Lexer::new(input))
}
