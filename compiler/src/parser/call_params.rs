use std::iter::Peekable;

use super::{expression, statements::Variables};
use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

/// Parses the Arguments to a function being called
///
/// # Arguments:
/// Expects the Token-Stream to start after the Opening-Paran
///
/// # Behaviour:
/// Consumes the Token-Stream until a Closing-Paran has been reached,
/// which will also be consumed and then returns the Parsed out Parameters
/// accepted by the Function, if any
///
/// # Example:
/// ```
/// # use compiler::lexer::{Token, TokenMetadata};
/// # use compiler::parser::{call_params::parse, statements::Variables};
/// # use compiler::ir::{Variable, DataType};
/// # let mut variables = Variables::new();
/// # variables.insert("test_name".to_string(), Variable::new_str("test_name", DataType::U32));
/// # let empty_metadata = TokenMetadata { file_name: "test".to_string(), line: 1, };
/// let tokens = &[
///     (Token::Identifier("test_name".to_owned()), empty_metadata.clone()),
///     (Token::CloseParan, empty_metadata.clone()),
/// ];
///
/// // Parse the Tokens
/// let mut iter = tokens.iter().peekable();
/// parse(&mut iter, &variables);
///
/// // Expects that the entire Token-Stream has been consumed,
/// // including the Closing-Parans
/// assert_eq!(None, iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>, vars: &Variables) -> Option<Vec<ir::Expression>>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let mut result = Vec::new();

    while let Some(next_token) = iter.peek() {
        match next_token {
            (Token::CloseParan, _) => {
                iter.next();
                break;
            }
            (Token::Comma, _) => {
                iter.next();
            }
            _ => {
                let exp = match expression::parse(iter, vars) {
                    Some(e) => e,
                    None => break,
                };

                result.push(exp);
            }
        };
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Value;

    #[test]
    fn no_params() {
        let tokens = &[(
            Token::CloseParan,
            TokenMetadata {
                file_name: "test".to_string(),
                line: 1,
            },
        )];

        let expected = Some(vec![]);

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &Variables::new())
        );
    }

    #[test]
    fn one_constant_param() {
        let tokens = &[
            (
                Token::Constant(Value::Integer(2)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::CloseParan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected = Some(vec![ir::Expression::Constant(ir::Value::I32(2))]);

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &Variables::new())
        );
    }

    #[test]
    fn two_constant_param() {
        let tokens = &[
            (
                Token::Constant(Value::Integer(2)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Comma,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Constant(Value::Integer(3)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::CloseParan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected = Some(vec![
            ir::Expression::Constant(ir::Value::I32(2)),
            ir::Expression::Constant(ir::Value::I32(3)),
        ]);

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &Variables::new())
        );
    }
}
