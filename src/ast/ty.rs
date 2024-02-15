use super::{def::Def, Parse};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Def(Box<Def>),
}

impl Parse for Ty {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, super::ParseError> {
        match Def::parse(lexer) {
            Ok(def) => Ok(Self::Def(Box::new(def))),
            Err(e) => Err(e.context("expected type definition")),
        }
    }
}
