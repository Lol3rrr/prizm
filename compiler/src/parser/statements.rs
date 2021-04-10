use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Keyword, Token, TokenMetadata},
};

use super::{call_params, condition, datatype, expression};

/// Parses all the Statements between two Curly Brackets `{}`, but
/// treats them all the same (no scope based variables or the like)
fn parse_scope<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<ir::Statement>>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    // Expect an opening curly brace at the start
    match iter.next() {
        Some((Token::OpenCurlyBrace, _)) => {}
        _ => return None,
    };

    let inner = parse(iter);

    // Expect a closing curly brace at the end
    match iter.next() {
        Some((Token::CloseCurlyBrace, _)) => {}
        _ => return None,
    }

    Some(inner)
}

fn parse_single<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<ir::Statement>>
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

            let inner = parse_scope(iter).unwrap();

            Some(vec![ir::Statement::WhileLoop(cond, inner)])
        }
        (Token::Keyword(Keyword::For), _) => {
            iter.next();

            match iter.next() {
                Some((Token::OpenParan, _)) => {}
                _ => return None,
            };

            let first = match parse_single(iter) {
                Some(mut tmp) => {
                    let first = tmp.remove(0);
                    match first {
                        ir::Statement::Declaration(_, _) => {
                            let mut result = vec![first];
                            result.append(&mut parse_single(iter).unwrap());
                            result
                        }
                        _ => vec![first],
                    }
                }
                _ => panic!("Unexpected"),
            };

            let cond = condition::parse(iter).unwrap();

            match iter.peek() {
                Some((Token::Semicolon, _)) => {
                    iter.next();
                }
                _ => {}
            };

            let third = parse_single(iter).unwrap();
            match iter.peek() {
                Some((Token::CloseParan, _)) => {
                    iter.next();
                }
                _ => {}
            };

            let mut inner_loop = parse_scope(iter).unwrap();
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

            let inner = parse_scope(iter).unwrap();

            Some(vec![ir::Statement::If(cond, inner)])
        }
        (Token::Keyword(_), _) => {
            let d_type = match datatype::parse(iter) {
                Some(d) => d,
                None => return None,
            };

            let var_name = match iter.peek() {
                Some((Token::Identifier(raw_name), _)) => raw_name.to_owned(),
                _ => return None,
            };

            // Removes the next item if its a semicolon
            match iter.peek() {
                Some((Token::Semicolon, _)) => {
                    iter.next();
                }
                _ => {}
            };

            Some(vec![ir::Statement::Declaration(var_name, d_type)])
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

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Vec<ir::Statement>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let mut result = Vec::new();

    while let Some(mut tmp) = parse_single(iter) {
        result.append(&mut tmp);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::lexer::Value;

    use super::*;

    #[test]
    fn while_loop() {
        let tokens = &[
            (
                Token::Keyword(Keyword::While),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::OpenParan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("i".to_string()),
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
                Token::Constant(Value::Integer(0)),
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
            (
                Token::OpenCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::OpenParan,
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
            (
                Token::Semicolon,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::CloseCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected: Vec<ir::Statement> = vec![ir::Statement::WhileLoop(
            ir::Condition {
                left: ir::Expression::Variable("i".to_string()),
                comparison: ir::Comparison::Equal,
                right: ir::Expression::Constant(ir::Value::I32(0)),
            },
            vec![ir::Statement::SingleExpression(ir::Expression::Call(
                "test".to_owned(),
                vec![],
            ))],
        )];

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn for_loop() {
        let tokens = &[
            (
                Token::Keyword(Keyword::For),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::OpenParan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("i".to_string()),
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
                Token::Constant(Value::Integer(0)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Semicolon,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("i".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::LessThan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Constant(Value::Integer(10)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Semicolon,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("i".to_string()),
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
                Token::Identifier("i".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Plus,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Constant(Value::Integer(1)),
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
            (
                Token::OpenCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::OpenParan,
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
            (
                Token::Semicolon,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::CloseCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected: Vec<ir::Statement> = vec![
            ir::Statement::Declaration("i".to_string(), ir::DataType::I32),
            ir::Statement::Assignment("i".to_string(), ir::Expression::Constant(ir::Value::I32(0))),
            ir::Statement::WhileLoop(
                ir::Condition {
                    left: ir::Expression::Variable("i".to_string()),
                    comparison: ir::Comparison::LessThan,
                    right: ir::Expression::Constant(ir::Value::I32(10)),
                },
                vec![
                    ir::Statement::SingleExpression(ir::Expression::Call(
                        "test".to_owned(),
                        vec![],
                    )),
                    ir::Statement::Assignment(
                        "i".to_owned(),
                        ir::Expression::Operation(
                            ir::OP::Add,
                            vec![
                                ir::Expression::Variable("i".to_owned()),
                                ir::Expression::Constant(ir::Value::I32(1)),
                            ],
                        ),
                    ),
                ],
            ),
        ];

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn deref_assign_variable() {
        let tokens = &[
            (
                Token::Asterisk,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test".to_string()),
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
                Token::Constant(Value::Integer(0)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Semicolon,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected = vec![ir::Statement::DerefAssignment(
            ir::Expression::Variable("test".to_string()),
            ir::Expression::Constant(ir::Value::I32(0)),
        )];

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn deref_assign_calc() {
        let tokens = &[
            (
                Token::Asterisk,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::OpenParan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Constant(Value::Integer(2)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Plus,
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
            (
                Token::Equals,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Constant(Value::Integer(0)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Semicolon,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let expected = vec![ir::Statement::DerefAssignment(
            ir::Expression::Operation(
                ir::OP::Add,
                vec![
                    ir::Expression::Constant(ir::Value::I32(2)),
                    ir::Expression::Constant(ir::Value::I32(3)),
                ],
            ),
            ir::Expression::Constant(ir::Value::I32(0)),
        )];

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
