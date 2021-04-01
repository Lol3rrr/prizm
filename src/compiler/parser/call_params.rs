use std::iter::Peekable;

use super::expression;
use crate::compiler::{ir, lexer::Token};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<ir::Expression>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    while let Some(next_token) = iter.peek() {
        match next_token {
            Token::CloseParan => {
                iter.next();
                break;
            }
            Token::Comma => {
                iter.next();
            }
            _ => {
                let exp = match expression::parse(iter) {
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
    use crate::compiler::lexer::Value;

    #[test]
    fn no_params() {
        let tokens = &[Token::CloseParan];

        let expected = Some(vec![]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn one_constant_param() {
        let tokens = &[Token::Constant(Value::Integer(2)), Token::CloseParan];

        let expected = Some(vec![ir::Expression::Constant(ir::Value::I32(2))]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn two_constant_param() {
        let tokens = &[
            Token::Constant(Value::Integer(2)),
            Token::Comma,
            Token::Constant(Value::Integer(3)),
            Token::CloseParan,
        ];

        let expected = Some(vec![
            ir::Expression::Constant(ir::Value::I32(2)),
            ir::Expression::Constant(ir::Value::I32(3)),
        ]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
