use error::HtmlParseError;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

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
