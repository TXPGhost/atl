use crate::{ast::ParseError, lexer::Token};

use self::{prod::Prod, sum::Sum};

use super::{ident::Ident, Parse};

pub mod func;
pub mod prod;
pub mod sum;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Def {
    Sum(Ident, sum::Sum),
    Prod(Ident, prod::Prod),
    Func(Ident, func::Func),
}

impl Parse for Def {
    fn parse(lexer: &mut crate::lexer::Lexer) -> Result<Self, super::ParseError> {
        match Ident::parse(lexer) {
            Ok(ident) => match lexer.peek() {
                Some(Token::Bar) => Ok(Self::Sum(ident, Sum::parse(lexer)?)),
                Some(Token::LeftBracket) => Ok(Self::Prod(ident, Prod::parse(lexer)?)),
                _ => ParseError::err("expected left bracket or bar for type definition"),
            },
            Err(e) => Err(e.context("expected type identifier")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{subtypes::Subtypes, ty::Ty},
        lexer::Lexer,
    };

    use super::*;

    fn test_parse_def_sum(s: &str, subtypes: Subtypes) {
        let lexer = &mut Lexer::new(s).unwrap();
        let def = Def::parse(lexer);

        match def {
            Ok(Def::Sum(ident, sum)) => {
                assert_eq!(s, ident.0.as_str());
                assert_eq!(subtypes, sum.0);
            }
            Ok(d) => panic!("{:?}", d),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(lexer.peek(), None);
    }

    fn test_parse_def_prod(s: &str, subtypes: Subtypes) {
        let lexer = &mut Lexer::new(s).unwrap();
        let def = Def::parse(lexer);

        match def {
            Ok(Def::Prod(ident, sum)) => {
                assert_eq!(s, ident.0.as_str());
                assert_eq!(subtypes, sum.0);
            }
            Ok(d) => panic!("{:?}", d),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(lexer.peek(), None);
    }

    fn test_parse_def_failure(s: &str) {
        let lexer = &mut Lexer::new(s).unwrap();
        let def = Def::parse(lexer);

        assert!(def.is_err());
    }

    #[test]
    fn test_bool() {
        test_parse_def_sum(
            "bool|true[], false[]|",
            Subtypes(vec![Ty::Def(Box::new(Def::Prod(
                Ident("true".to_owned()),
                Prod(Subtypes(vec![])),
            )))]),
        );
    }
}
