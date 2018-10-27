use pom::combinator::*;
use pom::Parser;
use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::quote;
use std::collections::HashMap;

use crate::config::global_attrs;
use crate::parser::*;

// State

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

    fn elem_name(&self) -> Ident {
        Ident::new(
            &format!("Element_{}", self.name.to_string()),
            self.name.span(),
        )
    }

    fn attr_names(&self) -> impl Iterator<Item = Ident> + '_ {
        self.attrs
            .keys()
            .map(|k| Ident::new(&format!("attr_{}", k.to_string()), k.span()))
    }

    fn attr_names_str(&self) -> impl Iterator<Item = String> + '_ {
        self.attrs.keys().map(|k| k.to_string())
    }

    fn req_child_names(&self) -> impl Iterator<Item = Ident> + '_ {
        self.req_children
            .iter()
            .map(|c| Ident::new(&format!("child_{}", c.to_string()), c.span()))
    }

    fn req_child_names_str(&self) -> impl Iterator<Item = String> + '_ {
        self.req_children.iter().map(|i| i.to_string())
    }

    fn req_child_types(&self) -> impl Iterator<Item = Ident> + '_ {
        self.req_children
            .iter()
            .map(|c| Ident::new(&format!("Element_{}", c.to_string()), c.span()))
    }

    fn into_token_stream(self) -> TokenStream {
        let mut stream = TokenStream::new();
        stream.extend(self.struct_());
        stream.extend(self.impl_());
        stream.extend(self.impl_node());
        stream.extend(self.impl_element());
        stream.extend(self.impl_marker_traits());
        stream.extend(self.impl_display());
        stream
    }

    fn struct_(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let attr_name = self.attr_names();
        let attr_type = self.attrs.values();
        let req_child_name = self.req_child_names();
        let req_child_type = self.req_child_types();

        let children = match &self.opt_children {
            Some(child_constraint) => quote!(pub children: Vec<Box<#child_constraint>>),
            None => TokenStream::new(),
        };

        quote!(
            pub struct #elem_name {
                #( pub #attr_name: Option<#attr_type>, )*
                pub data_attributes: std::collections::BTreeMap<String, String>,
                #( pub #req_child_name: Box<#req_child_type>, )*
                #children
            }
        )
    }

    fn impl_(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let req_child_name = self.req_child_names();
        let req_child_type = self.req_child_types();
        let req_child_name_again = self.req_child_names();
        let attr_name = self.attr_names();

        let construct_children = match self.opt_children {
            Some(_) => quote!(children: Vec::new()),
            None => TokenStream::new(),
        };

        quote!(
            impl #elem_name {
                pub fn new(#(#req_child_name: Box<#req_child_type>),*) -> Self {
                    #elem_name {
                        #( #attr_name: None, )*
                        data_attributes: std::collections::BTreeMap::new(),
                        #( #req_child_name_again, )*
                        #construct_children
                    }
                }
            }
        )
    }

    fn impl_node(&self) -> TokenStream {
        let elem_name = self.elem_name();
        quote!(
            impl Node for #elem_name {}
        )
    }

    fn impl_element(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let attr_name_str = self.attr_names_str();
        let req_child_str_name = self.req_child_names_str();
        quote!(
            impl Element for #elem_name {
                fn attributes() -> &'static [&'static str] {
                    &[ #(#attr_name_str),* ]
                }

                fn required_children() -> &'static [&'static str] {
                    &[ #(#req_child_str_name),* ]
                }
            }
        )
    }

    fn impl_marker_traits(&self) -> TokenStream {
        let trait_for = std::iter::repeat(self.elem_name());
        let trait_name = self.traits.iter();
        quote!(
            #(
                impl #trait_name for #trait_for {}
            )*
        )
    }

    fn impl_display(&self) -> TokenStream {
        let elem_name = self.elem_name();
        let name = self.name.to_string();
        let attr_name = self.attr_names();
        let attr_name_str = self.attr_names_str();
        let req_child_name: Vec<_> = self.req_child_names().collect();

        let print_opt_children = if self.opt_children.is_some() {
            quote!(for child in &self.children {
                child.fmt(f)?;
            })
        } else {
            TokenStream::new()
        };
        let print_children = if req_child_name.is_empty() {
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
                    self.#req_child_name.fmt(f)?;
                )*
                #print_opt_children
                write!(f, "</{}>", #name)
            )
        };

        quote!(
            impl std::fmt::Display for #elem_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, "<{}", #name);
                    #(
                        if let Some(ref value) = self.#attr_name {
                            write!(f, " {}={:?}", #attr_name_str, value.to_string())?;
                        }
                    )*
                    for (key, value) in &self.data_attributes {
                        write!(f, " data-{}={:?}", key, value)?;
                    }
                    #print_children
                }
            }
        )
    }
}

// Parser

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

pub fn expand_declare(input: &[TokenTree]) -> pom::Result<TokenStream> {
    declare().parse(input).map(|decl| decl.into_token_stream())
}
