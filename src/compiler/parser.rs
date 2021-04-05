use std::iter::Peekable;

use super::{
    ir,
    lexer::{Keyword, Token},
};

mod call_params;
mod comparison;
mod datatype;
mod expression;
mod func_args;

fn parse_statements<'a, I>(iter: &mut Peekable<I>) -> Vec<ir::Statement>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    while let Some(peeked) = iter.peek() {
        match peeked {
            Token::Keyword(Keyword::Return) => {
                iter.next();
                let expression = match expression::parse(iter) {
                    Some(exp) => exp,
                    None => ir::Expression::Empty,
                };

                result.push(ir::Statement::Return(expression));

                // Removes the next item if its a semicolon
                iter.next_if_eq(&&Token::Semicolon);
            }
            Token::Keyword(Keyword::While) => {
                iter.next();

                match iter.next() {
                    Some(Token::OpenParan) => {}
                    _ => break,
                };

                let left_exp = match expression::parse(iter) {
                    Some(exp) => exp,
                    None => break,
                };

                let comp = match comparison::parse(iter) {
                    Some(c) => c,
                    None => break,
                };

                let right_exp = match expression::parse(iter) {
                    Some(exp) => exp,
                    None => break,
                };

                match iter.next() {
                    Some(Token::CloseParan) => {}
                    _ => break,
                };

                match iter.next() {
                    Some(Token::OpenCurlyBrace) => {}
                    _ => break,
                };

                let inner = parse_statements(iter);

                result.push(ir::Statement::WhileLoop(left_exp, comp, right_exp, inner));
            }
            Token::Keyword(_) => {
                let d_type = match datatype::parse(iter) {
                    Some(d) => d,
                    None => break,
                };

                let var_name = match iter.peek() {
                    Some(Token::Identifier(raw_name)) => raw_name.to_owned(),
                    _ => break,
                };

                result.push(ir::Statement::Declaration(var_name, d_type));

                // Removes the next item if its a semicolon
                iter.next_if_eq(&&Token::Semicolon);
            }
            Token::Identifier(name) => {
                iter.next();

                match iter.next() {
                    Some(Token::Equals) => {
                        let expression = match expression::parse(iter) {
                            Some(exp) => exp,
                            None => break,
                        };

                        result.push(ir::Statement::Assignment(name.to_owned(), expression));

                        // Removes the next item if its a semicolon
                        iter.next_if_eq(&&Token::Semicolon);
                    }
                    Some(Token::OpenParan) => {
                        let params = match call_params::parse(iter) {
                            Some(p) => p,
                            None => break,
                        };

                        result.push(ir::Statement::SingleExpression(ir::Expression::Call(
                            name.to_owned(),
                            params,
                        )));

                        iter.next_if_eq(&&Token::Semicolon);
                    }
                    _ => break,
                };
            }
            Token::Asterisk => {
                iter.next();

                let var_name = match iter.next() {
                    Some(Token::Identifier(n)) => n.to_owned(),
                    _ => break,
                };

                match iter.next() {
                    Some(Token::Equals) => {
                        let exp = match expression::parse(iter) {
                            Some(e) => e,
                            None => break,
                        };

                        result.push(ir::Statement::DerefAssignment(var_name, exp));

                        iter.next_if_eq(&&Token::Semicolon);
                    }
                    _ => break,
                };
            }
            Token::CloseCurlyBrace => break,
            _ => {
                println!("[Parse-Statements] Unexpected: {:?}", peeked);
                break;
            }
        };
    }

    result
}

fn parse_function<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Function>
where
    I: Iterator<Item = &'a Token>,
{
    let datatype = match iter.next() {
        Some(Token::Keyword(Keyword::Integer)) => ir::DataType::I32,
        _ => return None,
    };

    let name = match iter.next() {
        Some(Token::Identifier(n)) => n.to_owned(),
        _ => return None,
    };

    match iter.next() {
        Some(Token::OpenParan) => {}
        _ => return None,
    };

    let args = func_args::parse(iter)?;

    match iter.next() {
        Some(Token::OpenCurlyBrace) => {}
        _ => return None,
    };

    let statements = parse_statements(iter);

    Some(ir::Function(name, datatype, args, statements))
}

pub fn parse(tokens: &[Token]) -> Vec<ir::Function> {
    let mut functions = Vec::new();

    let mut iter = tokens.iter().peekable();
    while iter.peek().is_some() {
        if let Some(func) = parse_function(&mut iter) {
            functions.push(func);
        }
    }

    functions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::Value;

    #[test]
    fn simple_function_with_return() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Keyword(Keyword::Return),
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![ir::Statement::Return(ir::Expression::Constant(
                ir::Value::I32(0),
            ))],
        )];

        assert_eq!(expected, parse(tokens));
    }

    #[test]
    fn simple_function_with_return_and_variable() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(2)),
            Token::Semicolon,
            Token::Keyword(Keyword::Return),
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration("test".to_string(), ir::DataType::I32),
                ir::Statement::Assignment(
                    "test".to_string(),
                    ir::Expression::Constant(ir::Value::I32(2)),
                ),
                ir::Statement::Return(ir::Expression::Constant(ir::Value::I32(0))),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }

    #[test]
    fn simple_addition() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test_add".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(2)),
            Token::Plus,
            Token::Constant(Value::Integer(3)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration("test_add".to_string(), ir::DataType::I32),
                ir::Statement::Assignment(
                    "test_add".to_string(),
                    ir::Expression::Operation(
                        ir::OP::Add,
                        vec![
                            ir::Expression::Constant(ir::Value::I32(2)),
                            ir::Expression::Constant(ir::Value::I32(3)),
                        ],
                    ),
                ),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }
    #[test]
    fn nested_addition() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test_add".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(2)),
            Token::Plus,
            Token::Constant(Value::Integer(3)),
            Token::Plus,
            Token::Constant(Value::Integer(4)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration("test_add".to_string(), ir::DataType::I32),
                ir::Statement::Assignment(
                    "test_add".to_string(),
                    ir::Expression::Operation(
                        ir::OP::Add,
                        vec![
                            ir::Expression::Constant(ir::Value::I32(2)),
                            ir::Expression::Operation(
                                ir::OP::Add,
                                vec![
                                    ir::Expression::Constant(ir::Value::I32(3)),
                                    ir::Expression::Constant(ir::Value::I32(4)),
                                ],
                            ),
                        ],
                    ),
                ),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }

    #[test]
    fn simple_substraction() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test_sub".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(2)),
            Token::Minus,
            Token::Constant(Value::Integer(3)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration("test_sub".to_string(), ir::DataType::I32),
                ir::Statement::Assignment(
                    "test_sub".to_string(),
                    ir::Expression::Operation(
                        ir::OP::Substract,
                        vec![
                            ir::Expression::Constant(ir::Value::I32(2)),
                            ir::Expression::Constant(ir::Value::I32(3)),
                        ],
                    ),
                ),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }

    #[test]
    fn simple_call() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Identifier("test_func".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::Semicolon,
            Token::Keyword(Keyword::Return),
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::SingleExpression(ir::Expression::Call(
                    "test_func".to_string(),
                    vec![],
                )),
                ir::Statement::Return(ir::Expression::Constant(ir::Value::I32(0))),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }
    #[test]
    fn call_as_assignment() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test_var".to_string()),
            Token::Equals,
            Token::Identifier("test_func".to_string()),
            Token::OpenParan,
            Token::CloseParan,
            Token::Semicolon,
            Token::Keyword(Keyword::Return),
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration("test_var".to_string(), ir::DataType::I32),
                ir::Statement::Assignment(
                    "test_var".to_string(),
                    ir::Expression::Call("test_func".to_string(), vec![]),
                ),
                ir::Statement::Return(ir::Expression::Constant(ir::Value::I32(0))),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }
}
