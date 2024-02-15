use crate::ast::{subtypes::Subtypes, Parse};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prod(pub Subtypes);

impl Parse for Prod {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, crate::ast::ParseError> {
        todo!()
    }
}
