use crate::{
    ast::{subtypes::Subtypes, Parse, ParseError},
    lexer::Token,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prod(pub Subtypes);

impl Parse for Prod {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, crate::ast::ParseError> {
        match lexer.peek() {
            Some(Token::LeftBracket) => {
                lexer.next()?;
                Ok(Self(Subtypes::parse(lexer)?))
            }
            _ => ParseError::err(lexer.tok_string(), "Prod: expected right bracket"),
        }
    }
}
