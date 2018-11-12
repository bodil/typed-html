use pom::combinator::*;
use pom::Parser;
use proc_macro::{quote, Group, Ident, Literal, TokenStream, TokenTree};

use config::global_attrs;
use map::StringyMap;
use parser::*;

// State

struct Declare {
    name: Ident,
    attrs: StringyMap<Ident, TokenStream>,
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

    fn elem_name(&self) -> TokenTree {
        Ident::new(
            &format!("Element_{}", self.name.to_string()),
            self.name.span(),
        )
        .into()
    }

    fn attr_type_name(&self) -> TokenTree {
        Ident::new(
            &format!("ElementAttrs_{}", self.name.to_string()),
            self.name.span(),
        )
        .into()
    }

    fn attrs(&self) -> impl Iterator<Item = (TokenTree, TokenStream, TokenTree)> + '_ {
        self.attrs.iter().map(|(key, value)| {
            let attr_name: TokenTree =
                Ident::new(&format!("attr_{}", key.to_string()), key.span()).into();
            let attr_type = value.clone();
            let attr_str = Literal::string(&key.to_string()).into();
            (attr_name, attr_type, attr_str)
        })
    }

    fn req_children(&self) -> impl Iterator<Item = (TokenTree, TokenTree, TokenTree)> + '_ {
        self.req_children.iter().map(|child| {
            let child_name: TokenTree =
                Ident::new(&format!("child_{}", child.to_string()), child.span()).into();
            let child_type: TokenTree =
                Ident::new(&format!("Element_{}", child.to_string()), child.span()).into();
            let child_str = Literal::string(&child.to_string()).into();
            (child_name, child_type, child_str)
        })
    }

    fn into_token_stream(self) -> TokenStream {
        let mut stream = TokenStream::new();
        stream.extend(self.attr_struct());
        stream.extend(self.struct_());
        stream.extend(self.impl_());
        stream.extend(self.impl_node());
        stream.extend(self.impl_element());
        stream.extend(self.impl_marker_traits());
        stream.extend(self.impl_display());
        stream
    }

    fn attr_struct(&self) -> TokenStream {
        let mut body = TokenStream::new();
        for (attr_name, attr_type, _) in self.attrs() {
            body.extend(quote!( pub $attr_name: Option<$attr_type>, ));
        }

        let attr_type_name = self.attr_type_name();
        quote!(
            pub struct $attr_type_name {
                $body
            }
        )
    }

    fn struct_(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let attr_type_name = self.attr_type_name();

        let mut body = TokenStream::new();

        for (child_name, child_type, _) in self.req_children() {
            body.extend(quote!( pub $child_name: Box<$child_type>, ));
        }

        if let Some(child_constraint) = &self.opt_children {
            let child_constraint = child_constraint.clone();
            body.extend(quote!(pub children: Vec<Box<$child_constraint>>,));
        }

        quote!(
            pub struct $elem_name {
                pub attrs: $attr_type_name,
                pub data_attributes: std::collections::BTreeMap<String, String>,
                $body
            }
        )
    }

    fn impl_(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let attr_type_name = self.attr_type_name();

        let mut args = TokenStream::new();
        for (child_name, child_type, _) in self.req_children() {
            args.extend(quote!( $child_name: Box<$child_type>, ));
        }

        let mut attrs = TokenStream::new();
        for (attr_name, _, _) in self.attrs() {
            attrs.extend(quote!( $attr_name: None, ));
        }

        let mut body = TokenStream::new();
        body.extend(quote!(
            attrs: $attr_type_name { $attrs },
        ));
        body.extend(quote!(data_attributes: std::collections::BTreeMap::new(),));
        for (child_name, _, _) in self.req_children() {
            body.extend(quote!( $child_name, ));
        }
        if self.opt_children.is_some() {
            body.extend(quote!(children: Vec::new()));
        }

        quote!(
            impl $elem_name {
                pub fn new($args) -> Self {
                    $elem_name {
                        $body
                    }
                }
            }
        )
    }

    fn impl_vnode(&self) -> TokenStream {
        let elem_name = TokenTree::Literal(Literal::string(self.name.to_string().as_str()));
        let mut req_children = TokenStream::new();
        for (child_name, _, _) in self.req_children() {
            req_children.extend(quote!(
                children.push(self.$child_name.vnode());
            ));
        }
        let mut opt_children = TokenStream::new();
        if self.opt_children.is_some() {
            opt_children.extend(quote!(for child in &self.children {
                children.push(child.vnode());
            }));
        }

        let mut push_attrs = TokenStream::new();
        for (attr_name, _, attr_str) in self.attrs() {
            push_attrs.extend(quote!(
                if let Some(ref value) = self.attrs.$attr_name {
                    attributes.push(($attr_str.to_string(), value.to_string()));
                }
            ));
        }

        quote!(
            let mut attributes = Vec::new();
            $push_attrs
            for (key, value) in &self.data_attributes {
                attributes.push((format!("data-{}", key), value.to_string()));
            }

            let mut children = Vec::new();
            $req_children
            $opt_children

            ::elements::VNode::Element(::elements::VElement {
                name: $elem_name,
                attributes,
                children
            })
        )
    }

    fn impl_node(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let vnode = self.impl_vnode();
        quote!(
            impl ::elements::Node for $elem_name {
                fn vnode(&self) -> ::elements::VNode {
                    $vnode
                }
            }
        )
    }

    fn impl_element(&self) -> TokenStream {
        let name: TokenTree = Literal::string(&self.name.to_string()).into();
        let elem_name = self.elem_name();

        let attrs: TokenStream = self.attrs().map(|(_, _, name)| quote!( $name, )).collect();
        let reqs: TokenStream = self
            .req_children()
            .map(|(_, _, name)| quote!( $name, ))
            .collect();

        let mut push_attrs = TokenStream::new();
        for (attr_name, _, attr_str) in self.attrs() {
            push_attrs.extend(quote!(
                if let Some(ref value) = self.attrs.$attr_name {
                    out.push(($attr_str.to_string(), value.to_string()));
                }
            ));
        }

        quote!(
            impl ::elements::Element for $elem_name {
                fn name() -> &'static str {
                    $name
                }

                fn attribute_names() -> &'static [&'static str] {
                    &[ $attrs ]
                }

                fn required_children() -> &'static [&'static str] {
                    &[ $reqs ]
                }

                fn attributes(&self) -> Vec<(String, String)> {
                    let mut out = Vec::new();
                    $push_attrs
                    for (key, value) in &self.data_attributes {
                        out.push((format!("data-{}", key), value.to_string()));
                    }
                    out
                }
            }
        )
    }

    fn impl_marker_traits(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let mut body = TokenStream::new();
        for t in &self.traits {
            let name = t.clone();
            body.extend(quote!(
                impl $name for $elem_name {}
            ));
        }
        body
    }

    fn impl_display(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let name: TokenTree = Literal::string(&self.name.to_string()).into();

        let print_opt_children = if self.opt_children.is_some() {
            quote!(for child in &self.children {
                child.fmt(f)?;
            })
        } else {
            TokenStream::new()
        };

        let mut print_req_children = TokenStream::new();
        for (child_name, _, _) in self.req_children() {
            print_req_children.extend(quote!(
                self.$child_name.fmt(f)?;
            ));
        }

        let print_children = if self.req_children.is_empty() {
            if self.opt_children.is_some() {
                quote!(if self.children.is_empty() {
                    write!(f, "/>")
                } else {
                    write!(f, ">")?;
                    $print_opt_children
                    write!(f, "</{}>", $name)
                })
            } else {
                quote!(write!(f, "/>"))
            }
        } else {
            quote!(
                write!(f, ">")?;
                $print_req_children
                $print_opt_children
                write!(f, "</{}>", $name)
            )
        };

        let mut print_attrs = TokenStream::new();
        for (attr_name, _, attr_str) in self.attrs() {
            print_attrs.extend(quote!(
                if let Some(ref value) = self.attrs.$attr_name {
                    write!(f, " {}={:?}", $attr_str, value.to_string())?;
                }
            ));
        }

        quote!(
            impl std::fmt::Display for $elem_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, "<{}", $name)?;
                    $print_attrs
                    for (key, value) in &self.data_attributes {
                        write!(f, " data-{}={:?}", key, value)?;
                    }
                    $print_children
                }
            }
        )
    }
}

// Parser

fn declare_attrs<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Vec<(Ident, TokenStream)>>>
{
    group().map(|group: Group| -> Vec<(Ident, TokenStream)> {
        let attr = ident() - punct(':') + type_spec() - punct(',').opt();
        let parser = attr.repeat(0..);
        let input: Vec<TokenTree> = group.stream().into_iter().collect();
        // FIXME the borrow checker won't let me use plain &input, it seems like a bug.
        // It works in Rust 2018, so please get rid of this unsafe block when it stabilises.
        parser
            .parse(unsafe { &*(input.as_slice() as *const _) })
            .unwrap()
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

pub fn expand_declare(input: &[TokenTree]) -> pom::Result<TokenStream> {
    declare().parse(input).map(|decl| decl.into_token_stream())
}
