use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

use super::{comparison, expression};

/// Parses an Conditional like `(i < 0)` or `i < 0;` into a proper
/// ir::Conditional Struct to allow for easier and more uniform usage
/// in the entire Repo
///
/// # Example:
/// ```rust
/// # use compiler::lexer::{Token, TokenMetadata};
/// # use compiler::parser::condition::parse;
/// # let empty_metadata = TokenMetadata { file_name: "test".to_owned(), line: 1, };
/// let tokens = &[
///     (Token::Identifier("test".to_owned()), empty_metadata.clone()),
///     (Token::Equals, empty_metadata.clone()),
///     (Token::Equals, empty_metadata.clone()),
///     (Token::Identifier("test".to_owned()), empty_metadata.clone()),
///     (Token::CloseParan, empty_metadata.clone()),
/// ];
///
/// // Parse the Tokens
/// let mut iter = tokens.iter().peekable();
/// parse(&mut iter);
///
/// // Expect that the Close-Paran is still left in the Stream
/// assert_eq!(Some(&(Token::CloseParan, empty_metadata)), iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Condition>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let left_comp = expression::parse(iter).unwrap();
    let comp = comparison::parse(iter).unwrap();
    let right_comp = expression::parse(iter).unwrap();

    Some(ir::Condition {
        left: left_comp,
        right: right_comp,
        comparison: comp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_conditional() {
        let tokens = &[
            (
                Token::Identifier("test_left".to_owned()),
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
            (
                Token::Equals,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_right".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected = Some(ir::Condition {
            left: ir::Expression::Variable("test_left".to_owned()),
            right: ir::Expression::Variable("test_right".to_owned()),
            comparison: ir::Comparison::Equal,
        });

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
