use crate::lexer::Token;

use super::{def::Def, ident::Ident, Parse, ParseError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Named(Ident),
    Def(Box<Def>),
}

impl Parse for Ty {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, super::ParseError> {
        match lexer.peek() {
            Some(Token::Identifier) => match Def::parse(lexer) {
                Ok(def) => Ok(Self::Def(Box::new(def))),
                Err(_) => Ok(Self::Named(Ident(lexer.tok_string()))),
            },
            _ => ParseError::err(lexer.tok_string(), "Ty: expected identifier"),
        }
    }
}
