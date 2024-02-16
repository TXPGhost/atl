use crate::{
    ast::{Parse, ParseError},
    lexer::Token,
};

use super::Expr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    exprs: Vec<Expr>,
}

impl Parse for Block {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, crate::ast::ParseError> {
        match lexer.peek() {
            Some(Token::LeftCurlyBrace) => {
                lexer.next()?;
                todo!()
            }
            _ => ParseError::err(lexer.tok_string(), "Block: expected left curly brace"),
        }
    }
}
