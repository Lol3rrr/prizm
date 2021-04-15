use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Keyword, Token, TokenMetadata},
};

mod parse_dt;

/// Parses a Token-Stream into a concrete Datatype
///
/// # Example:
/// ```rust
/// # use compiler::lexer::{Token, TokenMetadata, Keyword};
/// # use compiler::parser::datatype::parse;
/// # let empty_metadata = TokenMetadata { file_name: "test".to_owned(), line: 1, };
/// let tokens = &[
///     (Token::Keyword(Keyword::Integer), empty_metadata.clone()),
///     (Token::Identifier("test".to_owned()), empty_metadata.clone()),
/// ];
///
/// // Parse the Tokens
/// let mut iter = tokens.iter().peekable();
/// parse(&mut iter);
///
/// // Expect the Identifier to be left in the Token-Stream
/// assert_eq!(Some(&(Token::Identifier("test".to_owned()), empty_metadata)), iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::DataType>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let unsigned = match iter.peek() {
        Some((Token::Keyword(Keyword::Unsigned), _)) => {
            iter.next();
            true
        }
        _ => false,
    };

    parse_dt::parse(iter, unsigned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        assert_eq!(
            Some(ir::DataType::I32),
            parse(&mut tokens.iter().peekable())
        );
    }
    #[test]
    fn uint() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Unsigned),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        assert_eq!(
            Some(ir::DataType::U32),
            parse(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn short() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Short),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        assert_eq!(
            Some(ir::DataType::I16),
            parse(&mut tokens.iter().peekable())
        );
    }
    #[test]
    fn ushort() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Unsigned),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Keyword(Keyword::Short),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        assert_eq!(
            Some(ir::DataType::U16),
            parse(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn void() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Void),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        assert_eq!(
            Some(ir::DataType::Void),
            parse(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn simple_int_ptr() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Asterisk,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        assert_eq!(
            Some(ir::DataType::Ptr(Box::new(ir::DataType::I32))),
            parse(&mut tokens.iter().peekable())
        );
    }
}
