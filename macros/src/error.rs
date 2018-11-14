use ansi_term::Style;
use lalrpop_util::ParseError::*;
use lexer::Token;
use proc_macro::{Diagnostic, Ident, Level};

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
    let tokens: Vec<&str> = tokens.iter().map(|s| pprint_token(&s)).collect();
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

pub fn parse_error(input: &[Token], error: &ParseError) -> Diagnostic {
    match error {
        InvalidToken { location } => {
            let loc = &input[*location];
            Diagnostic::spanned(loc.span(), Level::Error, "invalid token")
        }
        UnrecognizedToken {
            token: None,
            expected,
        } => {
            let msg = format!("missing {}", pprint_tokens(&expected));
            Diagnostic::spanned(
                input[0].span().join(input[input.len() - 1].span()).unwrap(),
                Level::Error,
                "unexpected end of macro",
            )
            .help(msg)
        }
        UnrecognizedToken {
            token: Some((_, token, _)),
            expected,
        } => {
            let msg = format!("expected {}", pprint_tokens(&expected));
            let mut diag = Diagnostic::spanned(token.span(), Level::Error, msg);
            if is_in_node_position(expected) && token.is_ident() {
                // special case: you probably meant to quote that text
                diag = diag.help(format!(
                    "text nodes need to be quoted, eg. {}",
                    Style::new().bold().paint("<p>\"Hello Joe!\"</p>")
                ))
            }
            diag
        }
        ExtraToken {
            token: (_, token, _),
        } => Diagnostic::spanned(token.span(), Level::Error, "superfluous token"),
        User {
            error: HtmlParseError::TagMismatch { open, close },
        } => Diagnostic::spanned(
            close.span(),
            Level::Error,
            format!("expected closing tag '</{}>', found '</{}>'", open, close),
        )
        .span_help(open.span(), "opening tag is here:"),
    }
}
