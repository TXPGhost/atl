use crate::{
    ast::{subtypes::Subtypes, Parse, ParseError},
    lexer::Token,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sum(pub Subtypes);

impl Parse for Sum {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, crate::ast::ParseError> {
        match lexer.peek() {
            Some(Token::LeftAngleBracket) => {
                lexer.next()?;
                Ok(Self(Subtypes::parse(lexer)?))
            }
            _ => ParseError::err(lexer.tok_string(), "Sum: expected left angle bracket"),
        }
    }
}
