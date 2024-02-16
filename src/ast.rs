use std::fmt::Display;

use thiserror::Error;

use crate::lexer::Lexer;

pub mod def;
pub mod expr;
pub mod ident;
pub mod subtypes;
pub mod ty;

#[derive(Debug, Error, PartialEq, Eq)]
pub struct ParseError {
    trace: Vec<&'static str>,
    tok_string: String,
}

impl ParseError {
    pub fn new(tok_string: String, cause: &'static str) -> Self {
        Self {
            trace: vec![cause],
            tok_string,
        }
    }

    pub fn err<T>(tok_string: String, cause: &'static str) -> Result<T, Self> {
        Err(Self::new(tok_string, cause))
    }

    #[must_use]
    pub fn context(mut self, description: &'static str) -> Self {
        self.trace.push(description);
        self
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "error encountered at token \"{}\" while parsing {}",
            self.tok_string,
            self.trace
                .iter()
                .fold(String::new(), |acc, s| if acc.is_empty() {
                    acc + s
                } else {
                    acc + "\n-> while parsing " + s
                })
        )
    }
}

// TODO: switch from strings to tokens
pub trait Parse: Sized {
    fn parse(lexer: &mut Lexer) -> Result<Self, ParseError>;
}

pub trait ParseResultExt: Sized {
    #[must_use]
    fn context(self, description: &'static str) -> Self;
}

impl<T> ParseResultExt for Result<T, ParseError> {
    fn context(self, description: &'static str) -> Self {
        self.map_err(|e| e.context(description))
    }
}
