use crate::lexer::Token;

use super::{Lexer, Parse, ParseError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident(pub String);

impl Parse for Ident {
    fn parse(lexer: &mut Lexer) -> Result<Self, ParseError> {
        match lexer.peek() {
            Some(Token::Identifier) => Ok(Self({
                lexer.next()?;
                lexer.tok_string()
            })),
            _ => ParseError::err(lexer.tok_string(), "Ident: expected identifier"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_ident(name: &str, success: bool) {
        let lexer = &mut Lexer::new(name).unwrap();
        let ident = Ident::parse(lexer);
        if success {
            assert_eq!(ident, Ok(Ident(name.to_owned())));
            assert_eq!(lexer.peek(), None);
        } else {
            assert!(ident.is_err());
        }
    }

    #[test]
    fn test_simple() {
        test_parse_ident("hi", true);
    }

    #[test]
    fn test_simple_num() {
        test_parse_ident("hi2", true);
    }

    #[test]
    fn test_underscore_single() {
        test_parse_ident("_hi", true);
    }

    #[test]
    fn test_underscore_many() {
        test_parse_ident("___hi", true);
    }

    #[test]
    fn test_underscore_wrap() {
        test_parse_ident("__index__", true);
    }

    #[test]
    fn test_snake_case() {
        test_parse_ident("snake_case", true);
    }

    #[test]
    fn test_camel_case() {
        test_parse_ident("CamelCase", true);
    }

    #[test]
    fn test_pascal_case() {
        test_parse_ident("pascalCase", true);
    }

    #[test]
    fn test_start_with_num() {
        test_parse_ident("10810ident", false);
    }

    #[test]
    fn test_num() {
        test_parse_ident("10", false);
    }

    #[test]
    fn test_illegal_character() {
        test_parse_ident("identifier@name", false);
    }
}
