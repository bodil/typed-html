use lalrpop_util::ParseError::*;
use proc_macro::{
    Delimiter, Diagnostic, Group, Ident, Level, Literal, Punct, Span, TokenStream, TokenTree,
};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
pub type ParseError = lalrpop_util::ParseError<usize, Token, HtmlParseError>;

#[derive(Clone, Debug)]
pub enum Token {
    Ident(Ident),
    Literal(Literal),
    Punct(char, Punct),
    Group(Delimiter, Group),
    GroupOpen(Delimiter, Span),
    GroupClose(Delimiter, Span),
    Keyword(Keyword, Ident),
}

impl Token {
    pub fn span(&self) -> Span {
        match self {
            Token::Ident(ident) => ident.span(),
            Token::Literal(literal) => literal.span(),
            Token::Punct(_, punct) => punct.span(),
            Token::Group(_, group) => group.span(),
            Token::GroupOpen(_, span) => *span,
            Token::GroupClose(_, span) => *span,
            Token::Keyword(_, ident) => ident.span(),
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false,
        }
    }
}

impl From<Token> for TokenTree {
    fn from(token: Token) -> Self {
        match token {
            Token::Ident(ident) => TokenTree::Ident(ident),
            Token::Literal(literal) => TokenTree::Literal(literal),
            Token::Punct(_, punct) => TokenTree::Punct(punct),
            Token::Group(_, group) => TokenTree::Group(group),
            Token::GroupOpen(_, _) => panic!("Can't convert a GroupOpen token to a TokenTree"),
            Token::GroupClose(_, _) => panic!("Can't convert a GroupClose token to a TokenTree"),
            Token::Keyword(_, ident) => TokenTree::Ident(ident),
        }
    }
}

impl From<Token> for TokenStream {
    fn from(token: Token) -> Self {
        TokenTree::from(token).into()
    }
}

impl From<Ident> for Token {
    fn from(ident: Ident) -> Self {
        Token::Ident(ident)
    }
}

impl From<Literal> for Token {
    fn from(literal: Literal) -> Self {
        Token::Literal(literal)
    }
}

impl From<Punct> for Token {
    fn from(punct: Punct) -> Self {
        Token::Punct(punct.as_char(), punct)
    }
}

impl From<Group> for Token {
    fn from(group: Group) -> Self {
        Token::Group(group.delimiter(), group)
    }
}

#[derive(Debug, Clone)]
pub enum Keyword {
    In,
    With,
}

pub fn keywordise(tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .into_iter()
        .map(|token| match token {
            Token::Ident(ident) => {
                let name = ident.to_string();
                if name == "in" {
                    Token::Keyword(Keyword::In, ident)
                } else if name == "with" {
                    Token::Keyword(Keyword::With, ident)
                } else {
                    Token::Ident(ident)
                }
            }
            t => t,
        })
        .collect()
}

#[derive(Debug)]
pub struct HtmlParseError {
    pub token: Token,
    pub message: String,
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
        } => panic!(
            "unexpected end of macro: expecting {}",
            pprint_tokens(&expected)
        ),
        UnrecognizedToken {
            token: Some((_, token, _)),
            expected,
        } => {
            let mut msg = format!("expected {}", pprint_tokens(&expected));
            if is_in_node_position(expected) && token.is_ident() {
                // special case: you probably meant to quote that text
                msg += "; looks like you forgot to put \"quotes\" around your text nodes";
            }
            Diagnostic::spanned(token.span(), Level::Error, msg)
        }
        ExtraToken {
            token: (_, token, _),
        } => Diagnostic::spanned(token.span(), Level::Error, "superfluous token"),
        User { error } => {
            Diagnostic::spanned(error.token.span(), Level::Error, error.message.to_owned())
        }
    }
}

pub fn to_stream<I: IntoIterator<Item = Token>>(tokens: I) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend(tokens.into_iter().map(TokenTree::from));
    stream
}

pub fn unroll_stream(stream: TokenStream, deep: bool) -> Vec<Token> {
    let mut vec = Vec::new();
    for tt in stream {
        match tt {
            TokenTree::Ident(ident) => vec.push(ident.into()),
            TokenTree::Literal(literal) => vec.push(literal.into()),
            TokenTree::Punct(punct) => vec.push(punct.into()),
            TokenTree::Group(ref group) if deep => {
                vec.push(Token::GroupOpen(group.delimiter(), group.span()));
                let sub = unroll_stream(group.stream(), deep);
                vec.extend(sub);
                vec.push(Token::GroupClose(group.delimiter(), group.span()));
            }
            TokenTree::Group(group) => vec.push(group.into()),
        }
    }
    vec
}

pub struct Lexer<'a> {
    stream: &'a [Token],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a [Token]) -> Self {
        Lexer { stream, pos: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token, usize, HtmlParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.get(self.pos) {
            None => None,
            Some(token) => {
                self.pos += 1;
                Some(Ok((self.pos - 1, token.clone(), self.pos)))
            }
        }
    }
}
