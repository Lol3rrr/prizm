use std::iter::Peekable;

use super::scope;
use crate::parser::{call_params, condition, datatype, expression};
use crate::{
    const_eval, ir,
    lexer::{Keyword, Token, TokenMetadata},
};

/// Parses a Single Statement, like a single Line or a Loop
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<ir::Statement>>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let peeked = iter.peek()?;
    match peeked {
        (Token::Keyword(Keyword::Return), _) => {
            iter.next();
            let expression = match expression::parse(iter) {
                Some(exp) => exp,
                None => ir::Expression::Empty,
            };

            // Removes the next item if its a semicolon
            match iter.peek() {
                Some((Token::Semicolon, _)) => {
                    iter.next();
                }
                _ => {}
            };

            Some(vec![ir::Statement::Return(expression)])
        }
        (Token::Keyword(Keyword::While), _) => {
            iter.next();

            match iter.next() {
                Some((Token::OpenParan, _)) => {}
                _ => return None,
            };

            let cond = condition::parse(iter).unwrap();

            match iter.next() {
                Some((Token::CloseParan, _)) => {}
                _ => return None,
            };

            let inner = scope::parse_scope(iter).unwrap();

            Some(vec![ir::Statement::WhileLoop(cond, inner)])
        }
        (Token::Keyword(Keyword::For), _) => {
            iter.next();

            match iter.next() {
                Some((Token::OpenParan, _)) => {}
                _ => return None,
            };

            let first = parse(iter)?;

            let cond = condition::parse(iter).unwrap();

            match iter.peek() {
                Some((Token::Semicolon, _)) => {
                    iter.next();
                }
                _ => {}
            };

            let third = parse(iter)?;
            match iter.peek() {
                Some((Token::CloseParan, _)) => {
                    iter.next();
                }
                _ => {}
            };

            let mut inner_loop = scope::parse_scope(iter).unwrap();
            inner_loop.extend(third);

            let mut result = first;
            result.push(ir::Statement::WhileLoop(cond, inner_loop));

            Some(result)
        }
        (Token::Keyword(Keyword::If), _) => {
            iter.next();

            match iter.next() {
                Some((Token::OpenParan, _)) => {}
                _ => return None,
            };

            let cond = condition::parse(iter).unwrap();

            match iter.next() {
                Some((Token::CloseParan, _)) => {}
                _ => return None,
            };

            let inner = scope::parse_scope(iter).unwrap();

            Some(vec![ir::Statement::If(cond, inner)])
        }
        (Token::Keyword(_), _) => {
            let d_type = match datatype::parse(iter) {
                Some(d) => d,
                None => return None,
            };

            let var_name = match iter.peek() {
                Some((Token::Identifier(raw_name), _)) => {
                    iter.next();
                    raw_name.to_owned()
                }
                _ => return None,
            };

            match iter.next() {
                Some((Token::OpenSquareBrace, _)) => {
                    let raw_size = expression::parse(iter)?;
                    let size = const_eval::evaluate(raw_size)?;
                    iter.next();
                    iter.next();

                    Some(vec![ir::Statement::Declaration(
                        var_name,
                        ir::DataType::Array(Box::new(d_type), size),
                    )])
                }
                Some((Token::Equals, _)) => {
                    let value = expression::parse(iter)?;

                    // Removes the next item if its a semicolon
                    match iter.peek() {
                        Some((Token::Semicolon, _)) => {
                            iter.next();
                        }
                        _ => {}
                    };

                    Some(vec![
                        ir::Statement::Declaration(var_name.clone(), d_type),
                        ir::Statement::Assignment(var_name, value),
                    ])
                }
                Some((Token::Semicolon, _)) => {
                    Some(vec![ir::Statement::Declaration(var_name, d_type)])
                }
                Some((_, metadata)) => {
                    println!("{:?}", metadata);
                    None
                }
                _ => None,
            }
        }
        (Token::Identifier(name), _) => {
            iter.next();

            match iter.next() {
                Some((Token::Equals, _)) => {
                    let expression = match expression::parse(iter) {
                        Some(exp) => exp,
                        None => return None,
                    };

                    // Removes the next item if its a semicolon
                    match iter.peek() {
                        Some((Token::Semicolon, _)) => {
                            iter.next();
                        }
                        _ => {}
                    };

                    Some(vec![ir::Statement::Assignment(name.to_owned(), expression)])
                }
                Some((Token::OpenSquareBrace, _)) => {
                    let index_exp = expression::parse(iter)?;

                    match iter.peek() {
                        Some((Token::CloseSquareBrace, _)) => {
                            iter.next();
                        }
                        _ => {}
                    };

                    match iter.next() {
                        Some((Token::Equals, _)) => {}
                        _ => return None,
                    };

                    let exp = match expression::parse(iter) {
                        Some(e) => e,
                        None => return None,
                    };

                    match iter.peek() {
                        Some((Token::Semicolon, _)) => {
                            iter.next();
                        }
                        _ => {}
                    };

                    Some(vec![ir::Statement::DerefAssignment(
                        ir::Expression::Indexed(
                            Box::new(ir::Expression::Variable(name.clone())),
                            Box::new(index_exp),
                        ),
                        exp,
                    )])
                }
                Some((Token::OpenParan, _)) => {
                    let params = match call_params::parse(iter) {
                        Some(p) => p,
                        None => return None,
                    };

                    match iter.peek() {
                        Some((Token::Semicolon, _)) => {
                            iter.next();
                        }
                        _ => {}
                    };

                    Some(vec![ir::Statement::SingleExpression(ir::Expression::Call(
                        name.to_owned(),
                        params,
                    ))])
                }
                _ => return None,
            }
        }
        (Token::Asterisk, _) => {
            iter.next();

            let expression = expression::parse(iter)?;

            match iter.next() {
                Some((Token::Equals, _)) => {
                    let exp = match expression::parse(iter) {
                        Some(e) => e,
                        None => return None,
                    };

                    match iter.peek() {
                        Some((Token::Semicolon, _)) => {
                            iter.next();
                        }
                        _ => {}
                    };

                    Some(vec![ir::Statement::DerefAssignment(expression, exp)])
                }
                _ => return None,
            }
        }
        (Token::CloseCurlyBrace, _) => return None,
        _ => {
            println!("[Parse-Statements] Unexpected: {:?}", peeked);
            return None;
        }
    }
}
