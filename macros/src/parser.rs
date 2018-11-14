use lalrpop_util::lalrpop_mod;
use pom::combinator::*;
use pom::{Error, Parser};
use proc_macro::{Diagnostic, Group, Ident, Level, Punct, TokenStream, TokenTree};

lalrpop_mod!(pub grammar);

pub fn punct<'a>(punct: char) -> Combinator<impl Parser<'a, TokenTree, Output = Punct>> {
    comb(move |input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Punct(p)) if p.as_char() == punct => Ok((p.clone(), start + 1)),
        _ => Err(Error::Mismatch {
            message: format!("expected {:?}", punct),
            position: start,
        }),
    })
}

pub fn ident<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Ident>> {
    comb(|input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Ident(i)) => Ok((i.clone(), start + 1)),
        _ => Err(Error::Mismatch {
            message: "expected identifier".to_string(),
            position: start,
        }),
    })
}

pub fn group<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = Group>> {
    comb(|input: &[TokenTree], start| match input.get(start) {
        Some(TokenTree::Group(g)) => Ok((g.clone(), start + 1)),
        _ => Err(Error::Mismatch {
            message: "expected group".to_string(),
            position: start,
        }),
    })
}

fn to_stream<'a, I: IntoIterator<Item = &'a TokenTree>>(tokens: I) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend(tokens.into_iter().cloned());
    stream
}

pub fn type_spec<'a>() -> Combinator<impl Parser<'a, TokenTree, Output = TokenStream>> {
    let valid = ident().map(TokenTree::Ident)
        | punct(':').map(TokenTree::Punct)
        | punct('<').map(TokenTree::Punct)
        | punct('>').map(TokenTree::Punct)
        | punct('&').map(TokenTree::Punct)
        | punct('\'').map(TokenTree::Punct);
    valid.repeat(1..).collect().map(to_stream)
}

/// Turn a parser error into a proc_macro diagnostic.
pub fn parse_error(input: &[TokenTree], error: &pom::Error) -> Diagnostic {
    match error {
        pom::Error::Incomplete => Diagnostic::new(Level::Error, "unexpected end of macro!"),
        pom::Error::Mismatch { message, position } => {
            Diagnostic::spanned(input[*position].span(), Level::Error, message.as_str())
        }
        pom::Error::Conversion { message, position } => {
            Diagnostic::spanned(input[*position].span(), Level::Error, message.as_str())
        }
        pom::Error::Expect {
            message,
            position,
            inner,
        } => {
            let mut diag =
                Diagnostic::spanned(input[*position].span(), Level::Error, message.as_str());
            let child = parse_error(input, &inner);
            diag.span_error(child.spans(), child.message())
        }
        pom::Error::Custom {
            message,
            position,
            inner,
        } => {
            let mut diag =
                Diagnostic::spanned(input[*position].span(), Level::Error, message.as_str());
            if let Some(inner) = inner {
                let child = parse_error(input, &inner);
                diag.span_error(child.spans(), child.message())
            } else {
                diag
            }
        }
    }
}
