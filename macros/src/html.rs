use pom::combinator::*;
use pom::Parser;
use proc_macro::{quote, Group, Ident, Literal, TokenStream, TokenTree};

use config::required_children;
use map::StringyMap;
use parser::*;

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
struct Element {
    name: Ident,
    attributes: StringyMap<Ident, TokenTree>,
    children: Vec<Node>,
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

impl Element {
    fn new(name: Ident) -> Self {
        Element {
            name,
            attributes: StringyMap::new(),
            children: Vec::new(),
        }
    }

    fn into_token_stream(mut self) -> TokenStream {
        let name = self.name;
        let name_str = name.to_string();
        let typename: TokenTree = Ident::new(&format!("Element_{}", &name_str), name.span()).into();
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
        let attrs = self.attributes.iter().map(|(key, value)| {
            (
                TokenTree::Ident(Ident::new(&format!("attr_{}", key), key.span())),
                value.clone(),
            )
        });
        let opt_children = self
            .children
            .split_off(req_names.len())
            .into_iter()
            .map(Node::into_child_stream);
        let req_children = self.children.into_iter().map(Node::into_token_stream);

        let mut body = TokenStream::new();
        for (key, value) in attrs {
            body.extend(quote!(
                element.$key = Some($value.into());
            ));
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
