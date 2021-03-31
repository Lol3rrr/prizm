use std::iter::Peekable;

use crate::compiler::{
    ir,
    lexer::{Token, Value},
};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Expression>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
        Some(Token::Constant(constant_value)) => {
            iter.next();

            let left_side = match constant_value {
                Value::Integer(value) => ir::Expression::Constant(ir::Value::I64(*value)),
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
        _ => None,
    }
}
