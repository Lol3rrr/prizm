use std::iter::Peekable;

use super::{
    ir,
    lexer::{Keyword, Token},
};

mod expression;

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
            Token::Keyword(raw_datatype) => {
                iter.next();

                let datatype = match raw_datatype {
                    Keyword::Integer => ir::DataType::I64,
                    _ => break,
                };

                let var_name = match iter.peek() {
                    Some(Token::Identifier(raw_name)) => raw_name.to_owned(),
                    _ => break,
                };

                result.push(ir::Statement::Declaration(var_name, datatype));

                // Removes the next item if its a semicolon
                iter.next_if_eq(&&Token::Semicolon);
            }
            Token::Identifier(name) => {
                iter.next();

                match iter.next() {
                    Some(Token::Assignment) => {}
                    _ => break,
                };

                let expression = match expression::parse(iter) {
                    Some(exp) => exp,
                    None => break,
                };

                result.push(ir::Statement::Assignment(name.to_owned(), expression));

                // Removes the next item if its a semicolon
                iter.next_if_eq(&&Token::Semicolon);
            }
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
        Some(Token::Keyword(Keyword::Integer)) => ir::DataType::I64,
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

    // TODO
    // Parse Arguments

    match iter.next() {
        Some(Token::CloseParan) => {}
        _ => return None,
    };
    match iter.next() {
        Some(Token::OpenCurlyBrace) => {}
        _ => return None,
    };

    let statements = parse_statements(iter);

    Some(ir::Function(name, datatype, statements))
}

pub fn parse(tokens: &[Token]) -> Vec<ir::Function> {
    let mut functions = Vec::new();

    let mut iter = tokens.iter().peekable();
    while let Some(token) = iter.peek() {
        println!("[Parse] Peeked: {:?}", token);

        if let Some(func) = parse_function(&mut iter) {
            functions.push(func);
        }
    }

    functions
}

#[cfg(test)]
mod tests {
    use ir::DataType;

    use super::*;

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
            Token::Assignment,
            Token::Constant(Value::Integer(2)),
            Token::Semicolon,
            Token::Keyword(Keyword::Return),
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I64,
            vec![
                ir::Statement::Declaration("test".to_string(), DataType::I64),
                ir::Statement::Assignment(
                    "test".to_string(),
                    ir::Expression::Constant(ir::Value::I64(2)),
                ),
                ir::Statement::Return(ir::Expression::Constant(ir::Value::I64(0))),
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
            Token::Assignment,
            Token::Constant(Value::Integer(2)),
            Token::Plus,
            Token::Constant(Value::Integer(3)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I64,
            vec![
                ir::Statement::Declaration("test_add".to_string(), DataType::I64),
                ir::Statement::Assignment(
                    "test_add".to_string(),
                    ir::Expression::Operation(
                        ir::OP::Add,
                        vec![
                            ir::Expression::Constant(ir::Value::I64(2)),
                            ir::Expression::Constant(ir::Value::I64(3)),
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
            Token::Assignment,
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
            ir::DataType::I64,
            vec![
                ir::Statement::Declaration("test_add".to_string(), DataType::I64),
                ir::Statement::Assignment(
                    "test_add".to_string(),
                    ir::Expression::Operation(
                        ir::OP::Add,
                        vec![
                            ir::Expression::Constant(ir::Value::I64(2)),
                            ir::Expression::Operation(
                                ir::OP::Add,
                                vec![
                                    ir::Expression::Constant(ir::Value::I64(3)),
                                    ir::Expression::Constant(ir::Value::I64(4)),
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
            Token::Assignment,
            Token::Constant(Value::Integer(2)),
            Token::Minus,
            Token::Constant(Value::Integer(3)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I64,
            vec![
                ir::Statement::Declaration("test_sub".to_string(), DataType::I64),
                ir::Statement::Assignment(
                    "test_sub".to_string(),
                    ir::Expression::Operation(
                        ir::OP::Substract,
                        vec![
                            ir::Expression::Constant(ir::Value::I64(2)),
                            ir::Expression::Constant(ir::Value::I64(3)),
                        ],
                    ),
                ),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }
}
