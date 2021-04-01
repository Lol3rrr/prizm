use std::iter::Peekable;

use crate::compiler::{
    ir,
    lexer::{Keyword, Token},
};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::DataType>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
        Some(Token::Keyword(tmp)) => {
            iter.next();

            let first_part = match tmp {
                Keyword::Integer => ir::DataType::I32,
                Keyword::Void => ir::DataType::Void,
                _ => return None,
            };

            match iter.peek() {
                Some(Token::Asterisk) => {
                    iter.next();
                    Some(ir::DataType::Ptr(Box::new(first_part)))
                }
                _ => Some(first_part),
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_int() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test".to_owned()),
        ];

        assert_eq!(
            Some(ir::DataType::I32),
            parse(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn simple_void() {
        let tokens = &[
            Token::Keyword(Keyword::Void),
            Token::Identifier("test".to_owned()),
        ];

        assert_eq!(
            Some(ir::DataType::Void),
            parse(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn simple_int_ptr() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Asterisk,
            Token::Identifier("test".to_owned()),
        ];

        assert_eq!(
            Some(ir::DataType::Ptr(Box::new(ir::DataType::I32))),
            parse(&mut tokens.iter().peekable())
        );
    }
}
