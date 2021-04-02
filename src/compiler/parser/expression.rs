use std::iter::Peekable;

use super::call_params;
use crate::compiler::{
    ir,
    lexer::{Token, Value},
};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Expression>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
        Some(Token::Constant(_)) | Some(Token::Identifier(_)) => {
            let first = iter.next().unwrap();

            let left_side = match first {
                Token::Constant(constant_value) => match constant_value {
                    Value::Integer(value) => ir::Expression::Constant(ir::Value::I32(*value)),
                },
                Token::Identifier(name) => match iter.peek() {
                    Some(Token::OpenParan) => {
                        iter.next();

                        let params = call_params::parse(iter)?;
                        ir::Expression::Call(name.to_owned(), params)
                    }
                    _ => ir::Expression::Variable(name.to_owned()),
                },
                _ => return None,
            };

            match iter.peek() {
                Some(Token::Plus) | Some(Token::Minus) => {
                    let operation = match iter.next() {
                        Some(Token::Plus) => ir::OP::Add,
                        Some(Token::Minus) => ir::OP::Substract,
                        _ => return None,
                    };

                    let right_side = match parse(iter) {
                        Some(r) => r,
                        None => return None,
                    };

                    Some(ir::Expression::Operation(
                        operation,
                        vec![left_side, right_side],
                    ))
                }
                _ => Some(left_side),
            }
        }
        Some(Token::And) | Some(Token::Asterisk) => {
            let first = iter.next().unwrap();

            let var_name = match iter.peek() {
                Some(Token::Identifier(name)) => {
                    iter.next();
                    name.to_string()
                }
                _ => return None,
            };

            match first {
                Token::And => Some(ir::Expression::Reference(var_name)),
                Token::Asterisk => Some(ir::Expression::Dereference(var_name)),
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant() {
        let tokens = &[Token::Constant(Value::Integer(2))];

        let expected = Some(ir::Expression::Constant(ir::Value::I32(2)));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable() {
        let tokens = &[Token::Identifier("test".to_string())];

        let expected = Some(ir::Expression::Variable("test".to_string()));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn constant_plus_constant() {
        let tokens = &[
            Token::Constant(Value::Integer(2)),
            Token::Plus,
            Token::Constant(Value::Integer(3)),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Constant(ir::Value::I32(2)),
                ir::Expression::Constant(ir::Value::I32(3)),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable_plus_variable() {
        let tokens = &[
            Token::Identifier("test_1".to_string()),
            Token::Plus,
            Token::Identifier("test_2".to_string()),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Variable("test_1".to_string()),
                ir::Expression::Variable("test_2".to_string()),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn reference_to_variable() {
        let tokens = &[Token::And, Token::Identifier("test".to_string())];

        let expected = Some(ir::Expression::Reference("test".to_string()));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn dereference_variable() {
        let tokens = &[Token::Asterisk, Token::Identifier("test".to_string())];

        let expected = Some(ir::Expression::Dereference("test".to_string()));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
