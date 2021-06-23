use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

use super::{comparison, expression, statements::Variables};

/// Parses an Conditional like `(i < 0)` or `i < 0;` into a proper
/// ir::Conditional Struct to allow for easier and more uniform usage
/// in the entire Repo
///
/// # Example:
/// ```rust
/// # use compiler::lexer::{Token, TokenMetadata};
/// # use compiler::parser::condition::parse;
/// # use compiler::parser::statements::Variables;
/// # use compiler::ir::{Variable, DataType};
/// # let mut variables = Variables::new();
/// # variables.insert("test".to_owned(), Variable::new_str("test", DataType::U32));
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
/// parse(&mut iter, &variables);
///
/// // Expect that the Close-Paran is still left in the Stream
/// assert_eq!(Some(&(Token::CloseParan, empty_metadata)), iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>, vars: &Variables) -> Option<ir::Condition>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let left_comp = expression::parse(iter, vars).unwrap();
    let comp = comparison::parse(iter).unwrap();
    let right_comp = expression::parse(iter, vars).unwrap();

    Some(ir::Condition {
        left: left_comp,
        right: right_comp,
        comparison: comp,
    })
}

#[cfg(test)]
mod tests {
    use crate::ir::Variable;

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

        let mut vars = Variables::new();
        let left_var = Variable::new_str("test_left", ir::DataType::U32);
        let right_var = Variable::new_str("test_right", ir::DataType::U32);
        vars.insert(left_var.name.clone(), left_var.clone());
        vars.insert(right_var.name.clone(), right_var.clone());

        let expected = Some(ir::Condition {
            left: ir::Expression::Variable(left_var),
            right: ir::Expression::Variable(right_var),
            comparison: ir::Comparison::Equal,
        });

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }
}
