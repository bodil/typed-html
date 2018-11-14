use proc_macro::{
    quote, Delimiter, Diagnostic, Group, Ident, Level, Literal, TokenStream, TokenTree,
};

use config::required_children;
use lexer::{Lexer, ParseError, Token};
use map::StringyMap;
use parser::grammar;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(Literal),
    Block(Group),
}

impl Node {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            Node::Element(el) => el.into_token_stream(),
            Node::Text(text) => {
                let text = TokenTree::Literal(text);
                quote!(Box::new(typed_html::elements::TextNode::new($text.to_string())))
            }
            Node::Block(_) => panic!("cannot have a block in this position"),
        }
    }

    fn into_child_stream(self) -> TokenStream {
        match self {
            Node::Element(el) => {
                let el = el.into_token_stream();
                quote!(
                    element.children.push($el);
                )
            }
            tx @ Node::Text(_) => {
                let tx = tx.into_token_stream();
                quote!(
                    element.children.push($tx);
                )
            }
            Node::Block(group) => {
                let group: TokenTree = group.into();
                quote!(
                for child in $group.into_iter() {
                    element.children.push(child);
                }
            )
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

fn process_value(value: &TokenTree) -> TokenStream {
    match value {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
            let content = g.stream();
            quote!( [ $content ] )
        }
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
            let content = g.stream();
            quote!( ( $content ) )
        }
        v => v.clone().into(),
    }
}

fn is_string_literal(literal: &Literal) -> bool {
    // This is the worst API
    literal.to_string().starts_with('"')
}

impl Element {
    fn into_token_stream(mut self) -> TokenStream {
        let name = self.name;
        let name_str = name.to_string();
        let typename: TokenTree = Ident::new(&format!("Element_{}", &name_str), name.span()).into();
        let req_names = required_children(&name_str);
        if req_names.len() > self.children.len() {
            Diagnostic::spanned(
                name.span(),
                Level::Error,
                format!(
                    "<{}> requires {} children but there are only {}",
                    name_str,
                    req_names.len(),
                    self.children.len()
                ),
            )
            .emit();
            panic!();
        }
        let data_attrs = extract_data_attrs(&mut self.attributes);
        let attrs = self.attributes.iter().map(|(key, value)| {
            (
                key.to_string(),
                TokenTree::Ident(Ident::new_raw(&key.to_string(), key.span())),
                value,
            )
        });
        let opt_children = self
            .children
            .split_off(req_names.len())
            .into_iter()
            .map(Node::into_child_stream);
        let req_children = self.children.into_iter().map(Node::into_token_stream);

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
                        span.source_file().path().to_str().unwrap_or("unknown"),
                        span.start().line,
                        span.start().column
                    );
                    let pos_str: TokenTree = Literal::string(&pos).into();
                    body.extend(quote!(
                        element.attrs.$key = Some($value.parse().unwrap_or_else(|err| {
                            eprintln!("ERROR: {}: <{} {}={:?}> failed to parse attribute value: {}",
                                      $pos_str, $tag_name, $attr_str, $value, err);
                            panic!("failed to parse string literal");
                        }));
                    ));
                }
                value => {
                    let value = process_value(value);
                    body.extend(quote!(
                        element.attrs.$key = Some(std::convert::TryInto::try_into($value).unwrap());
                    ));
                }
            }
        }
        for (key, value) in data_attrs
            .iter()
            .map(|(k, v)| (TokenTree::from(Literal::string(&k)), v.clone()))
        {
            body.extend(quote!(
                element.data_attributes.insert($key.into(), $value.into());
            ));
        }
        body.extend(opt_children);

        let mut args = TokenStream::new();
        for arg in req_children {
            args.extend(quote!( $arg, ));
        }

        quote!(
            {
                let mut element = typed_html::elements::$typename::new($args);
                $body
                Box::new(element)
            }
        )
    }
}

pub fn expand_html(input: &[Token]) -> Result<Node, ParseError> {
    grammar::NodeParser::new().parse(Lexer::new(input))
}
