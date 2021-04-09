use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Keyword, Token},
};

/// Parses the next Datatype
///
/// Params:
/// `unsigned`: Whether or not the unsigend modifier was applied
fn parse_dt<'a, I>(iter: &mut Peekable<I>, unsigned: bool) -> Option<ir::DataType>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
        Some(Token::Keyword(tmp)) => {
            iter.next();

            let raw = match tmp {
                Keyword::Integer if !unsigned => ir::DataType::I32,
                Keyword::Integer if unsigned => ir::DataType::U32,
                Keyword::Short if !unsigned => ir::DataType::I16,
                Keyword::Short if unsigned => ir::DataType::U16,
                Keyword::Void => ir::DataType::Void,
                _ => return None,
            };

            match iter.peek() {
                Some(Token::Asterisk) => {
                    iter.next();
                    Some(ir::DataType::Ptr(Box::new(raw)))
                }
                _ => Some(raw),
            }
        }
        _ => None,
    }
}

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::DataType>
where
    I: Iterator<Item = &'a Token>,
{
    let unsigned = match iter.peek() {
        Some(Token::Keyword(Keyword::Unsigned)) => {
            iter.next();
            true
        }
        _ => false,
    };

    parse_dt(iter, unsigned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
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
    fn uint() {
        let tokens = &[
            Token::Keyword(Keyword::Unsigned),
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test".to_owned()),
        ];

        assert_eq!(
            Some(ir::DataType::U32),
            parse(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn short() {
        let tokens = &[
            Token::Keyword(Keyword::Short),
            Token::Identifier("test".to_owned()),
        ];

        assert_eq!(
            Some(ir::DataType::I16),
            parse(&mut tokens.iter().peekable())
        );
    }
    #[test]
    fn ushort() {
        let tokens = &[
            Token::Keyword(Keyword::Unsigned),
            Token::Keyword(Keyword::Short),
            Token::Identifier("test".to_owned()),
        ];

        assert_eq!(
            Some(ir::DataType::U16),
            parse(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn void() {
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
