use logos::Logos;

use crate::ast::ParseError;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip "//.*")]
pub enum Token {
    #[regex("\"[^\"]*\"")]
    String,

    #[regex("'[^\']*'")]
    Char,

    #[token("let")]
    Let,

    #[token("def")]
    Def,

    #[token("::")]
    DoubleColon,

    #[token(":")]
    Colon,

    #[token(";")]
    SemiColon,

    #[token("=")]
    Equals,

    #[token("->")]
    Arrow,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("&")]
    Ampersand,

    #[token("|")]
    Bar,

    #[token("!")]
    ExclamationPoint,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("{")]
    LeftCurlyBrace,

    #[token("}")]
    RightCurlyBrace,

    #[token("<")]
    LeftAngleBracket,

    #[token(">")]
    RightAngleBracket,

    #[regex("[a-zA-Z_][a-zA-Z\\d_]*")]
    Identifier,

    #[regex("\\d+", priority = 1)]
    Number,
}

#[derive(Clone)]
pub struct Lexer<'lexer> {
    lexer: logos::Lexer<'lexer, Token>,
    current_tok: Option<Token>,
    next_tok: Option<Token>,
    tok_string: &'lexer str,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Result<Self, ParseError> {
        let mut lexer = Token::lexer(source);
        let next_tok = lexer
            .next()
            .transpose()
            .map_err(|()| ParseError::new(lexer.slice().to_owned(), "unrecognized token"))?;
        Ok(Self {
            lexer,
            current_tok: None,
            next_tok,
            tok_string: "",
        })
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.next_tok
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<Token>, ParseError> {
        self.tok_string = self.lexer.slice();

        self.current_tok = self.next_tok;
        self.next_tok = self
            .lexer
            .next()
            .transpose()
            .map_err(|()| ParseError::new(self.tok_string.to_owned(), "unrecognized token"))?;

        Ok(self.current_tok)
    }

    pub fn tok_string(&self) -> String {
        self.tok_string.to_owned()
    }
}
