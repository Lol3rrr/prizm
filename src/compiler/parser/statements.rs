use std::iter::Peekable;

use crate::compiler::{
    ir,
    lexer::{Keyword, Token},
};

use super::{call_params, comparison, datatype, expression};

fn parse_scope<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<ir::Statement>>
where
    I: Iterator<Item = &'a Token>,
{
    // Expect an opening curly brace at the start
    match iter.next() {
        Some(Token::OpenCurlyBrace) => {}
        _ => return None,
    };

    let inner = parse(iter);

    // Expect a closing curly brace at the end
    match iter.next() {
        Some(Token::CloseCurlyBrace) => {}
        _ => return None,
    }

    Some(inner)
}

fn parse_single<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<ir::Statement>>
where
    I: Iterator<Item = &'a Token>,
{
    let peeked = iter.peek()?;
    match peeked {
        Token::Keyword(Keyword::Return) => {
            iter.next();
            let expression = match expression::parse(iter) {
                Some(exp) => exp,
                None => ir::Expression::Empty,
            };

            // Removes the next item if its a semicolon
            iter.next_if_eq(&&Token::Semicolon);

            Some(vec![ir::Statement::Return(expression)])
        }
        Token::Keyword(Keyword::While) => {
            iter.next();

            match iter.next() {
                Some(Token::OpenParan) => {}
                _ => return None,
            };

            let left_exp = match expression::parse(iter) {
                Some(exp) => exp,
                None => return None,
            };

            let comp = match comparison::parse(iter) {
                Some(c) => c,
                None => return None,
            };

            let right_exp = match expression::parse(iter) {
                Some(exp) => exp,
                None => return None,
            };

            match iter.next() {
                Some(Token::CloseParan) => {}
                _ => return None,
            };

            let inner = parse_scope(iter).unwrap();

            Some(vec![ir::Statement::WhileLoop(
                left_exp, comp, right_exp, inner,
            )])
        }
        Token::Keyword(Keyword::For) => {
            iter.next();

            match iter.next() {
                Some(Token::OpenParan) => {}
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

            let left_comp = expression::parse(iter).unwrap();
            let comp = comparison::parse(iter).unwrap();
            let right_comp = expression::parse(iter).unwrap();

            iter.next_if_eq(&&Token::Semicolon);

            let third = parse_single(iter).unwrap();
            iter.next_if_eq(&&Token::CloseParan);

            let mut inner_loop = parse_scope(iter).unwrap();
            inner_loop.extend(third);

            let mut result = first;
            result.push(ir::Statement::WhileLoop(
                left_comp, comp, right_comp, inner_loop,
            ));

            Some(result)
        }
        Token::Keyword(_) => {
            let d_type = match datatype::parse(iter) {
                Some(d) => d,
                None => return None,
            };

            let var_name = match iter.peek() {
                Some(Token::Identifier(raw_name)) => raw_name.to_owned(),
                _ => return None,
            };

            // Removes the next item if its a semicolon
            iter.next_if_eq(&&Token::Semicolon);

            Some(vec![ir::Statement::Declaration(var_name, d_type)])
        }
        Token::Identifier(name) => {
            iter.next();

            match iter.next() {
                Some(Token::Equals) => {
                    let expression = match expression::parse(iter) {
                        Some(exp) => exp,
                        None => return None,
                    };

                    // Removes the next item if its a semicolon
                    iter.next_if_eq(&&Token::Semicolon);

                    Some(vec![ir::Statement::Assignment(name.to_owned(), expression)])
                }
                Some(Token::OpenParan) => {
                    let params = match call_params::parse(iter) {
                        Some(p) => p,
                        None => return None,
                    };

                    iter.next_if_eq(&&Token::Semicolon);

                    Some(vec![ir::Statement::SingleExpression(ir::Expression::Call(
                        name.to_owned(),
                        params,
                    ))])
                }
                _ => return None,
            }
        }
        Token::Asterisk => {
            iter.next();

            let expression = expression::parse(iter)?;

            match iter.next() {
                Some(Token::Equals) => {
                    let exp = match expression::parse(iter) {
                        Some(e) => e,
                        None => return None,
                    };

                    iter.next_if_eq(&&Token::Semicolon);

                    Some(vec![ir::Statement::DerefAssignment(expression, exp)])
                }
                _ => return None,
            }
        }
        Token::CloseCurlyBrace => return None,
        _ => {
            println!("[Parse-Statements] Unexpected: {:?}", peeked);
            return None;
        }
    }
}

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Vec<ir::Statement>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    while let Some(mut tmp) = parse_single(iter) {
        result.append(&mut tmp);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::compiler::lexer::Value;

    use super::*;

    #[test]
    fn while_loop() {
        let tokens = &[
            Token::Keyword(Keyword::While),
            Token::OpenParan,
            Token::Identifier("i".to_string()),
            Token::Equals,
            Token::Equals,
            Token::Constant(Value::Integer(0)),
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Identifier("test".to_owned()),
            Token::OpenParan,
            Token::CloseParan,
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected: Vec<ir::Statement> = vec![ir::Statement::WhileLoop(
            ir::Expression::Variable("i".to_string()),
            ir::Comparison::Equal,
            ir::Expression::Constant(ir::Value::I32(0)),
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
            Token::Keyword(Keyword::For),
            Token::OpenParan,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("i".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::LessThan,
            Token::Constant(Value::Integer(10)),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Equals,
            Token::Identifier("i".to_string()),
            Token::Plus,
            Token::Constant(Value::Integer(1)),
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Identifier("test".to_owned()),
            Token::OpenParan,
            Token::CloseParan,
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected: Vec<ir::Statement> = vec![
            ir::Statement::Declaration("i".to_string(), ir::DataType::I32),
            ir::Statement::Assignment("i".to_string(), ir::Expression::Constant(ir::Value::I32(0))),
            ir::Statement::WhileLoop(
                ir::Expression::Variable("i".to_string()),
                ir::Comparison::LessThan,
                ir::Expression::Constant(ir::Value::I32(10)),
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
            Token::Asterisk,
            Token::Identifier("test".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
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
            Token::Asterisk,
            Token::OpenParan,
            Token::Constant(Value::Integer(2)),
            Token::Plus,
            Token::Constant(Value::Integer(3)),
            Token::CloseParan,
            Token::Equals,
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
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
