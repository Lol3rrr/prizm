use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Token, TokenMetadata, Value},
    parser::{call_params, statements::Variables},
};

use super::parse;

/// Parses a single Expression, so only Constants and Variables
pub fn parse_single<'a, I>(iter: &mut Peekable<I>, vars: &Variables) -> Option<ir::Expression>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    match iter.peek() {
        Some((Token::Constant(const_val), _)) => {
            iter.next().unwrap();
            match const_val {
                Value::Integer(value) => Some(ir::Expression::Constant(ir::Value::I32(*value))),
                Value::UInteger(value) => Some(ir::Expression::Constant(ir::Value::U32(*value))),
            }
        }
        Some((Token::Identifier(name), _)) => {
            iter.next().unwrap();
            match iter.peek() {
                Some((Token::OpenParan, _)) => {
                    iter.next();

                    let params = call_params::parse(iter, vars)?;
                    Some(ir::Expression::Call(name.to_owned(), params))
                }
                Some((Token::OpenSquareBrace, _)) => {
                    iter.next();

                    let index = parse(iter, vars)?;

                    match iter.next() {
                        Some((Token::CloseSquareBrace, _)) => {}
                        _ => return None,
                    };

                    let variable = match vars.get(name) {
                        Some(v) => v.clone(),
                        None => return None,
                    };

                    Some(ir::Expression::Dereference(Box::new(
                        ir::Expression::Indexed(
                            Box::new(ir::Expression::Variable(variable)),
                            Box::new(index),
                        ),
                    )))
                }
                _ => match vars.get(name) {
                    Some(variable) => Some(ir::Expression::Variable(variable.clone())),
                    None => None,
                },
            }
        }
        _ => None,
    }
}
