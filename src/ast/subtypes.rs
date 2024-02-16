use crate::lexer::Token;

use super::{ty::Ty, Parse};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subtypes(pub Vec<Ty>);

impl Parse for Subtypes {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, super::ParseError> {
        let mut tys = Vec::new();
        loop {
            if let Some(Token::RightAngleBracket | Token::RightBracket) = lexer.peek() {
                lexer.next()?;
                return Ok(Self(tys));
            } else if let Some(Token::Comma) = lexer.peek() {
                lexer.next()?;
            }

            match Ty::parse(lexer) {
                Ok(ty) => tys.push(ty),
                Err(e) => return Err(e.context("Subtypes: expected type")),
            }
        }
    }
}
