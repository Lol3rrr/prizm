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
    use crate::{
        lexer::{Keyword, Value},
        test_token_pair,
    };

    #[test]
    fn simple_function_with_return() {
        let tokens = &[
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("main".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::OpenCurlyBrace),
            test_token_pair!(Token::Keyword(Keyword::Return)),
            test_token_pair!(Token::Constant(Value::Integer(0))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::CloseCurlyBrace),
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
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("main".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::OpenCurlyBrace),
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("test".to_string())),
            test_token_pair!(Token::Equals),
            test_token_pair!(Token::Constant(Value::Integer(2))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::Keyword(Keyword::Return)),
            test_token_pair!(Token::Constant(Value::Integer(0))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::CloseCurlyBrace),
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration(ir::Variable::new_str("test", ir::DataType::I32)),
                ir::Statement::Assignment(
                    ir::Variable::new_str("test", ir::DataType::I32),
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
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("main".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::OpenCurlyBrace),
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("test_add".to_string())),
            test_token_pair!(Token::Equals),
            test_token_pair!(Token::Constant(Value::Integer(2))),
            test_token_pair!(Token::Plus),
            test_token_pair!(Token::Constant(Value::Integer(3))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::CloseCurlyBrace),
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration(ir::Variable::new_str("test_add", ir::DataType::I32)),
                ir::Statement::Assignment(
                    ir::Variable::new_str("test_add", ir::DataType::I32),
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
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("main".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::OpenCurlyBrace),
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("test_add".to_string())),
            test_token_pair!(Token::Equals),
            test_token_pair!(Token::Constant(Value::Integer(2))),
            test_token_pair!(Token::Plus),
            test_token_pair!(Token::Constant(Value::Integer(3))),
            test_token_pair!(Token::Plus),
            test_token_pair!(Token::Constant(Value::Integer(4))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::CloseCurlyBrace),
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration(ir::Variable::new_str("test_add", ir::DataType::I32)),
                ir::Statement::Assignment(
                    ir::Variable::new_str("test_add", ir::DataType::I32),
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
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("main".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::OpenCurlyBrace),
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("test_sub".to_string())),
            test_token_pair!(Token::Equals),
            test_token_pair!(Token::Constant(Value::Integer(2))),
            test_token_pair!(Token::Minus),
            test_token_pair!(Token::Constant(Value::Integer(3))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::CloseCurlyBrace),
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration(ir::Variable::new_str("test_sub", ir::DataType::I32)),
                ir::Statement::Assignment(
                    ir::Variable::new_str("test_sub", ir::DataType::I32),
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
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("main".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::OpenCurlyBrace),
            test_token_pair!(Token::Identifier("test_func".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::Keyword(Keyword::Return)),
            test_token_pair!(Token::Constant(Value::Integer(0))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::CloseCurlyBrace),
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
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("main".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::OpenCurlyBrace),
            test_token_pair!(Token::Keyword(Keyword::Integer)),
            test_token_pair!(Token::Identifier("test_var".to_string())),
            test_token_pair!(Token::Equals),
            test_token_pair!(Token::Identifier("test_func".to_string())),
            test_token_pair!(Token::OpenParan),
            test_token_pair!(Token::CloseParan),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::Keyword(Keyword::Return)),
            test_token_pair!(Token::Constant(Value::Integer(0))),
            test_token_pair!(Token::Semicolon),
            test_token_pair!(Token::CloseCurlyBrace),
        ];

        let expected = vec![ir::Function(
            "main".to_string(),
            ir::DataType::I32,
            vec![],
            vec![
                ir::Statement::Declaration(ir::Variable::new_str("test_var", ir::DataType::I32)),
                ir::Statement::Assignment(
                    ir::Variable::new_str("test_var", ir::DataType::I32),
                    ir::Expression::Call("test_func".to_string(), vec![]),
                ),
                ir::Statement::Return(ir::Expression::Constant(ir::Value::I32(0))),
            ],
        )];

        assert_eq!(expected, parse(tokens));
    }
}
