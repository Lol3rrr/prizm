use super::{
    ir,
    lexer::{Token, TokenMetadata},
};

pub mod call_params;
pub mod comparison;
pub mod condition;
pub mod datatype;
pub mod expression;
pub mod func_args;
pub mod function;
pub mod statements;

/// Parses the Tokens into the Compilers-IR that represents the actual
/// Program in a more Abstract way
pub fn parse(tokens: &[(Token, TokenMetadata)]) -> Vec<ir::Function> {
    let mut functions = Vec::new();

    let mut iter = tokens.iter().peekable();
    while iter.peek().is_some() {
        if let Some(func) = function::parse(&mut iter) {
            functions.push(func);
        }
    }

    functions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Keyword, Value};

    #[test]
    fn simple_function_with_return() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
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
                Token::OpenCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Keyword(Keyword::Return),
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
                Token::CloseCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
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
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
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
                Token::OpenCurlyBrace,
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
                Token::Constant(Value::Integer(2)),
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
                Token::Keyword(Keyword::Return),
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
                Token::CloseCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
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
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
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
                Token::OpenCurlyBrace,
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
                Token::Identifier("test_add".to_string()),
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
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
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
                Token::OpenCurlyBrace,
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
                Token::Identifier("test_add".to_string()),
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
                Token::Plus,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Constant(Value::Integer(4)),
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
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
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
                Token::OpenCurlyBrace,
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
                Token::Identifier("test_sub".to_string()),
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
                Token::Constant(Value::Integer(2)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Minus,
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
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
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
                Token::OpenCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_func".to_string()),
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
                Token::Keyword(Keyword::Return),
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
                Token::CloseCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
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
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
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
                Token::OpenCurlyBrace,
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
                Token::Identifier("test_var".to_string()),
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
                Token::Identifier("test_func".to_string()),
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
                Token::Keyword(Keyword::Return),
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
                Token::CloseCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
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
