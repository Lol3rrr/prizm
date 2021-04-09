use super::{ir, lexer::Token};

mod call_params;
mod comparison;
mod condition;
mod datatype;
mod expression;
mod func_args;
mod function;
mod statements;

pub fn parse(tokens: &[Token]) -> Vec<ir::Function> {
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
