use std::{collections::HashMap, iter::Peekable};

use crate::{
    ir::{self, Variable},
    lexer::{Token, TokenMetadata},
};

mod scope;
mod single;

pub type Variables = HashMap<String, Variable>;

/// Parses the Token-Stream into a List of Statements
///
/// # Example:
/// ```rust
/// # use compiler::lexer::{Token, TokenMetadata};
/// # use compiler::parser::statements::{parse, Variables};
/// # use compiler::ir::{Variable, DataType};
/// # let mut variables = Variables::new();
/// # variables.insert("test".to_owned(), Variable::new_str("test", DataType::U32));
/// # let empty_metadata = TokenMetadata { file_name: "test".to_owned(), line: 1, };
/// let tokens = &[
///     (Token::Identifier("test".to_owned()), empty_metadata.clone()),
///     (Token::Semicolon, empty_metadata.clone()),
/// ];
///
/// // Parse the Tokens
/// let mut iter = tokens.iter().peekable();
/// parse(&mut iter, &mut variables);
///
/// // Expect
/// assert_eq!(None, iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>, vars: &mut Variables) -> Vec<ir::Statement>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let mut result = Vec::new();

    while let Some(mut tmp) = single::parse(iter, vars) {
        result.append(&mut tmp);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::ir::DataType;
    use crate::lexer::{Keyword, Value};

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

        let mut vars = Variables::new();
        let i_var = Variable::new_str("i", DataType::U32);
        vars.insert("i".to_string(), i_var.clone());

        let expected: Vec<ir::Statement> = vec![ir::Statement::WhileLoop(
            ir::Condition {
                left: ir::Expression::Variable(i_var),
                comparison: ir::Comparison::Equal,
                right: ir::Expression::Constant(ir::Value::I32(0)),
            },
            vec![ir::Statement::SingleExpression(ir::Expression::Call(
                "test".to_owned(),
                vec![],
            ))],
        )];

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &mut vars));
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

        let i_var = Variable::new_str("i", DataType::I32);

        let expected: Vec<ir::Statement> = vec![
            ir::Statement::Declaration(i_var.clone()),
            ir::Statement::Assignment(i_var.clone(), ir::Expression::Constant(ir::Value::I32(0))),
            ir::Statement::WhileLoop(
                ir::Condition {
                    left: ir::Expression::Variable(i_var.clone()),
                    comparison: ir::Comparison::LessThan,
                    right: ir::Expression::Constant(ir::Value::I32(10)),
                },
                vec![
                    ir::Statement::SingleExpression(ir::Expression::Call(
                        "test".to_owned(),
                        vec![],
                    )),
                    ir::Statement::Assignment(
                        i_var.clone(),
                        ir::Expression::Operation(
                            ir::OP::Add,
                            vec![
                                ir::Expression::Variable(i_var.clone()),
                                ir::Expression::Constant(ir::Value::I32(1)),
                            ],
                        ),
                    ),
                ],
            ),
        ];

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &mut Variables::new())
        );
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

        let mut vars = Variables::new();
        let test_var = Variable::new_str("test", DataType::Ptr(Box::new(DataType::I32)));
        vars.insert("test".to_string(), test_var.clone());

        let expected = vec![ir::Statement::DerefAssignment(
            ir::Expression::Variable(test_var),
            ir::Expression::Constant(ir::Value::I32(0)),
        )];

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &mut vars));
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

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &mut Variables::new())
        );
    }
}
