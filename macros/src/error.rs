use crate::lexer::Token;
use ansi_term::Style;
use lalrpop_util::ParseError::*;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};

pub type ParseError = lalrpop_util::ParseError<usize, Token, HtmlParseError>;

#[derive(Debug)]
pub enum HtmlParseError {
    TagMismatch { open: Ident, close: Ident },
}

fn pprint_token(token: &str) -> &str {
    match token {
        "BraceGroupToken" => "code block",
        "LiteralToken" => "literal",
        "IdentToken" => "identifier",
        a => a,
    }
}

fn pprint_tokens(tokens: &[String]) -> String {
    let tokens: Vec<&str> = tokens.iter().map(|s| pprint_token(s)).collect();
    if tokens.len() > 1 {
        let start = tokens[..tokens.len() - 1].join(", ");
        let end = &tokens[tokens.len() - 1];
        format!("{} or {}", start, end)
    } else {
        tokens[0].to_string()
    }
}

fn is_in_node_position(tokens: &[String]) -> bool {
    use std::collections::HashSet;
    let input: HashSet<&str> = tokens.iter().map(String::as_str).collect();
    let output: HashSet<&str> = ["\"<\"", "BraceGroupToken", "LiteralToken"]
        .iter()
        .cloned()
        .collect();
    input == output
}

pub fn parse_error(input: &[Token], error: &ParseError) -> TokenStream {
    match error {
        InvalidToken { location } => {
            let span = input[*location].span();
            quote_spanned! {span=>
                compile_error! { "invalid token" }
            }
        }
        UnrecognizedEOF { expected, .. } => {
            let msg = format!(
                "unexpected end of macro; missing {}",
                pprint_tokens(expected)
            );
            quote! {
                compile_error! { #msg }
            }
        }
        UnrecognizedToken {
            token: (_, token, _),
            expected,
        } => {
            let span = token.span();
            let error_msg = format!("expected {}", pprint_tokens(expected));
            let error = quote_spanned! {span=>
                compile_error! { #error_msg }
            };
            let help = if is_in_node_position(expected) && token.is_ident() {
                // special case: you probably meant to quote that text
                let help_msg = format!(
                    "text nodes need to be quoted, eg. {}",
                    Style::new().bold().paint("<p>\"Hello Joe!\"</p>")
                );
                Some(quote_spanned! {span=>
                    compile_error! { #help_msg }
                })
            } else {
                None
            };
            quote! {{
                #error
                #help
            }}
        }
        ExtraToken {
            token: (_, token, _),
        } => {
            let span = token.span();
            quote_spanned! {span=>
                compile_error! { "superfluous token" }
            }
        }
        User {
            error: HtmlParseError::TagMismatch { open, close },
        } => {
            let close_span = close.span();
            let close_msg = format!("expected closing tag '</{}>', found '</{}>'", open, close);
            let close_error = quote_spanned! {close_span=>
                compile_error! { #close_msg }
            };
            let open_span = open.span();
            let open_error = quote_spanned! {open_span=>
                compile_error! { "unclosed tag" }
            };
            quote! {{
                #close_error
                #open_error
            }}
        }
    }
}
