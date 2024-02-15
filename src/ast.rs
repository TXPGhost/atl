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
}

impl ParseError {
    pub fn new(cause: &'static str) -> Self {
        Self { trace: vec![cause] }
    }

    pub fn err<T>(cause: &'static str) -> Result<T, Self> {
        Err(Self::new(cause))
    }

    #[must_use]
    pub fn context(mut self, description: &'static str) -> Self {
        self.trace.push(description);
        self
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.trace {
            writeln!(f, "{}", message)?;
        }
        Ok(())
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
