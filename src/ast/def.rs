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
                Some(Token::LeftAngleBracket) => Ok(Self::Sum(ident, Sum::parse(lexer)?)),
                Some(Token::LeftBracket) => Ok(Self::Prod(ident, Prod::parse(lexer)?)),
                _ => ParseError::err(
                    lexer.tok_string(),
                    "Def: expected left bracket or bar for type definition",
                ),
            },
            Err(e) => Err(e.context("Def: expected type identifier")),
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

    fn test_parse_def_sum(s: &str, subtypes: &Subtypes) {
        let lexer = &mut Lexer::new(s).unwrap();
        let def = Def::parse(lexer);

        match def {
            Ok(Def::Sum(ident, sum)) => {
                assert_eq!(s.split('<').next().unwrap(), ident.0.as_str());
                assert_eq!(*subtypes, sum.0);
            }
            Ok(d) => panic!("{:?}", d),
            Err(e) => panic!("{}", e),
        }

        assert_eq!(lexer.peek(), None);
    }

    fn test_parse_def_prod(s: &str, subtypes: &Subtypes) {
        let lexer = &mut Lexer::new(s).unwrap();
        let def = Def::parse(lexer);

        match def {
            Ok(Def::Prod(ident, sum)) => {
                assert_eq!(s.split('[').next().unwrap(), ident.0.as_str());
                assert_eq!(*subtypes, sum.0);
            }
            Ok(d) => panic!("{:?}", d),
            Err(e) => panic!("{}", e),
        }

        assert_eq!(lexer.peek(), None);
    }

    fn test_parse_def_failure(s: &str) {
        let lexer = &mut Lexer::new(s).unwrap();
        let def = Def::parse(lexer);

        assert!(def.is_err());
    }

    #[test]
    fn test_never() {
        test_parse_def_sum("never<>", &Subtypes(vec![]));
    }

    #[test]
    fn test_void() {
        test_parse_def_prod("void[]", &Subtypes(vec![]));
    }

    #[test]
    fn test_bool() {
        test_parse_def_sum(
            "bool<true[], false[]>",
            &Subtypes(vec![
                Ty::Def(Box::new(Def::Prod(
                    Ident("true".to_owned()),
                    Prod(Subtypes(vec![])),
                ))),
                Ty::Def(Box::new(Def::Prod(
                    Ident("false".to_owned()),
                    Prod(Subtypes(vec![])),
                ))),
            ]),
        );
    }

    #[test]
    fn test_int_option() {
        test_parse_def_sum(
            "int_option<some[int], none[]>",
            &Subtypes(vec![
                Ty::Def(Box::new(Def::Prod(
                    Ident("some".to_owned()),
                    Prod(Subtypes(vec![Ty::Named(Ident("int".to_owned()))])),
                ))),
                Ty::Def(Box::new(Def::Prod(
                    Ident("none".to_owned()),
                    Prod(Subtypes(vec![])),
                ))),
            ]),
        );
    }

    #[test]
    fn test_color() {
        test_parse_def_sum(
            "color<red[], green[], blue[]>",
            &Subtypes(vec![
                Ty::Def(Box::new(Def::Prod(
                    Ident("red".to_owned()),
                    Prod(Subtypes(vec![])),
                ))),
                Ty::Def(Box::new(Def::Prod(
                    Ident("green".to_owned()),
                    Prod(Subtypes(vec![])),
                ))),
                Ty::Def(Box::new(Def::Prod(
                    Ident("blue".to_owned()),
                    Prod(Subtypes(vec![])),
                ))),
            ]),
        );
    }

    #[test]
    fn test_nesting() {
        test_parse_def_prod(
            "a[b<d[], e<>>, c[f[], g<>]]",
            &Subtypes(vec![
                Ty::Def(Box::new(Def::Sum(
                    Ident("b".to_owned()),
                    Sum(Subtypes(vec![
                        Ty::Def(Box::new(Def::Prod(
                            Ident("d".to_owned()),
                            Prod(Subtypes(vec![])),
                        ))),
                        Ty::Def(Box::new(Def::Sum(
                            Ident("e".to_owned()),
                            Sum(Subtypes(vec![])),
                        ))),
                    ])),
                ))),
                Ty::Def(Box::new(Def::Prod(
                    Ident("c".to_owned()),
                    Prod(Subtypes(vec![
                        Ty::Def(Box::new(Def::Prod(
                            Ident("f".to_owned()),
                            Prod(Subtypes(vec![])),
                        ))),
                        Ty::Def(Box::new(Def::Sum(
                            Ident("g".to_owned()),
                            Sum(Subtypes(vec![])),
                        ))),
                    ])),
                ))),
            ]),
        );
    }

    #[test]
    fn test_identifier() {
        test_parse_def_failure("mytype");
    }

    #[test]
    fn test_missing_brackets() {
        test_parse_def_failure("outer[inner[int, bool, char]");
    }
}
