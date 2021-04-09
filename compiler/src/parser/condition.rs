use std::iter::Peekable;

use crate::{ir, lexer::Token};

use super::{comparison, expression};

/// Parses an Conditional like `(i < 0)` or `i < 0;` into a proper
/// ir::Conditional Struct to allow for easier and more uniform usage
/// in the entire Repo
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Condition>
where
    I: Iterator<Item = &'a Token>,
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
            Token::Identifier("test_left".to_owned()),
            Token::Equals,
            Token::Equals,
            Token::Identifier("test_right".to_owned()),
        ];

        let expected = Some(ir::Condition {
            left: ir::Expression::Variable("test_left".to_owned()),
            right: ir::Expression::Variable("test_right".to_owned()),
            comparison: ir::Comparison::Equal,
        });

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
