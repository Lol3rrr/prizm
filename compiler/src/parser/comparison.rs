use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

/// Parses the Comparison Operator itself only consuming 2 entries
/// from the Iterator and most and only consuming them if they
/// are actually part of the Comparison
///
/// # Example:
/// ```rust
/// # use compiler::lexer::{Token, TokenMetadata};
/// # use compiler::parser::comparison::parse;
/// # let empty_metadata = TokenMetadata { file_name: "test".to_owned(), line: 1, };
/// let tokens = &[
///     (Token::Equals, empty_metadata.clone()),
///     (Token::Equals, empty_metadata.clone()),
/// ];
///
/// // Parse the Tokens
/// let mut iter = tokens.iter().peekable();
/// parse(&mut iter);
///
/// // Expects that the entire Token-Stream has been consumed
/// assert_eq!(None, iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Comparison>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let peeked = iter.peek();
    match peeked {
        Some((Token::Equals, _)) => {
            iter.next();

            match iter.peek() {
                Some((Token::Equals, _)) => {
                    iter.next();

                    Some(ir::Comparison::Equal)
                }
                _ => None,
            }
        }
        Some((Token::LessThan, _)) => {
            iter.next();

            Some(ir::Comparison::LessThan)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_equals() {
        let tokens = &[
            (
                Token::Equals,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Equals,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected = Some(ir::Comparison::Equal);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn parse_less_than() {
        let tokens = &[(
            Token::LessThan,
            TokenMetadata {
                file_name: "test".to_string(),
                line: 1,
            },
        )];

        let expected = Some(ir::Comparison::LessThan);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
