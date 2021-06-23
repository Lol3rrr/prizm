use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

use super::statements::Variables;

mod single;

/// Parses the Token-Stream into a single Expression that may
/// be made up of different Sub-Expressions
///
/// # Example:
/// ```rust
/// # use compiler::lexer::{Token, TokenMetadata};
/// # use compiler::parser::expression::parse;
/// # use compiler::parser::statements::Variables;
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
/// parse(&mut iter, &variables);
///
/// // Expect the ending Semicolon to still be left in the Token-Stream
/// assert_eq!(Some(&(Token::Semicolon, empty_metadata)), iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>, vars: &Variables) -> Option<ir::Expression>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    match iter.peek() {
        Some((Token::OpenParan, _)) => {
            iter.next();
            let inner = parse(iter, vars);

            match iter.next() {
                Some((Token::CloseParan, _)) => {}
                _ => return None,
            };

            inner
        }
        Some((Token::Constant(_), _)) | Some((Token::Identifier(_), _)) => {
            let left_side = single::parse_single(iter, vars)?;

            match iter.peek() {
                Some((Token::Plus, _))
                | Some((Token::Minus, _))
                | Some((Token::Asterisk, _))
                | Some((Token::Slash, _)) => {
                    let operation = match iter.next() {
                        Some((Token::Plus, _)) => ir::OP::Add,
                        Some((Token::Minus, _)) => ir::OP::Substract,
                        Some((Token::Asterisk, _)) => ir::OP::Multiply,
                        Some((Token::Slash, _)) => ir::OP::Divide,
                        _ => return None,
                    };

                    let right_side = match parse(iter, vars) {
                        Some(r) => r,
                        None => return None,
                    };

                    match (&operation, right_side) {
                        (ir::OP::Multiply, ir::Expression::Operation(other_op, mut additions))
                        | (ir::OP::Divide, ir::Expression::Operation(other_op, mut additions)) => {
                            let right_left = additions.remove(0);

                            let exp =
                                ir::Expression::Operation(operation, vec![left_side, right_left]);

                            Some(ir::Expression::Operation(
                                other_op,
                                vec![exp, additions.remove(0)],
                            ))
                        }
                        (_, right) => {
                            Some(ir::Expression::Operation(operation, vec![left_side, right]))
                        }
                    }
                }
                _ => Some(left_side),
            }
        }
        Some((Token::And, _)) => {
            iter.next().unwrap();

            let var_name = match iter.peek() {
                Some((Token::Identifier(name), _)) => {
                    iter.next();
                    name.to_string()
                }
                _ => return None,
            };

            match vars.get(&var_name) {
                Some(variable) => Some(ir::Expression::Reference(variable.clone())),
                None => None,
            }
        }
        Some((Token::Asterisk, _)) => {
            iter.next().unwrap();

            let inner = parse(iter, vars).unwrap();

            Some(ir::Expression::Dereference(Box::new(inner)))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ir::Variable, lexer::Value};

    #[test]
    fn constant() {
        let tokens = &[(
            Token::Constant(Value::Integer(2)),
            TokenMetadata {
                file_name: "test".to_string(),
                line: 1,
            },
        )];

        let expected = Some(ir::Expression::Constant(ir::Value::I32(2)));

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &Variables::new())
        );
    }
    #[test]
    fn variable() {
        let tokens = &[(
            Token::Identifier("test".to_string()),
            TokenMetadata {
                file_name: "test".to_string(),
                line: 1,
            },
        )];

        let var = Variable::new_str("test", ir::DataType::U32);
        let expected = Some(ir::Expression::Variable(var.clone()));

        let mut vars = Variables::new();
        vars.insert(var.name.clone(), var);

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }

    #[test]
    fn constant_plus_constant() {
        let tokens = &[
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
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Constant(ir::Value::I32(2)),
                ir::Expression::Constant(ir::Value::I32(3)),
            ],
        ));

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &Variables::new())
        );
    }
    #[test]
    fn variable_plus_variable() {
        let tokens = &[
            (
                Token::Identifier("test_1".to_string()),
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
                Token::Identifier("test_2".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let mut vars = Variables::new();
        let test_1_var = Variable::new_str("test_1", ir::DataType::U32);
        let test_2_var = Variable::new_str("test_2", ir::DataType::U32);
        vars.insert(test_1_var.name.clone(), test_1_var.clone());
        vars.insert(test_2_var.name.clone(), test_2_var.clone());

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Variable(test_1_var),
                ir::Expression::Variable(test_2_var),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }
    #[test]
    fn variable_multiply_variable() {
        let tokens = &[
            (
                Token::Identifier("test_1".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Asterisk,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_2".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let mut vars = Variables::new();
        let test_1_var = Variable::new_str("test_1", ir::DataType::U32);
        let test_2_var = Variable::new_str("test_2", ir::DataType::U32);
        vars.insert(test_1_var.name.clone(), test_1_var.clone());
        vars.insert(test_2_var.name.clone(), test_2_var.clone());

        let expected = Some(ir::Expression::Operation(
            ir::OP::Multiply,
            vec![
                ir::Expression::Variable(test_1_var),
                ir::Expression::Variable(test_2_var),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }
    #[test]
    fn variable_multiply_variable_plus_variable() {
        let tokens = &[
            (
                Token::Identifier("test_1".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Asterisk,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_2".to_string()),
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
                Token::Identifier("test_3".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let mut vars = Variables::new();
        let test_1_var = Variable::new_str("test_1", ir::DataType::U32);
        let test_2_var = Variable::new_str("test_2", ir::DataType::U32);
        let test_3_var = Variable::new_str("test_3", ir::DataType::U32);
        vars.insert(test_1_var.name.clone(), test_1_var.clone());
        vars.insert(test_2_var.name.clone(), test_2_var.clone());
        vars.insert(test_3_var.name.clone(), test_3_var.clone());

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Operation(
                    ir::OP::Multiply,
                    vec![
                        ir::Expression::Variable(test_1_var),
                        ir::Expression::Variable(test_2_var),
                    ],
                ),
                ir::Expression::Variable(test_3_var),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }
    #[test]
    fn variable_divide_variable() {
        let tokens = &[
            (
                Token::Identifier("test_1".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Slash,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_2".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let mut vars = Variables::new();
        let test_1_var = Variable::new_str("test_1", ir::DataType::U32);
        let test_2_var = Variable::new_str("test_2", ir::DataType::U32);
        vars.insert(test_1_var.name.clone(), test_1_var.clone());
        vars.insert(test_2_var.name.clone(), test_2_var.clone());

        let expected = Some(ir::Expression::Operation(
            ir::OP::Divide,
            vec![
                ir::Expression::Variable(test_1_var),
                ir::Expression::Variable(test_2_var),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }
    #[test]
    fn variable_divide_variable_plus_variable() {
        let tokens = &[
            (
                Token::Identifier("test_1".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Slash,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_2".to_string()),
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
                Token::Identifier("test_3".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let mut vars = Variables::new();
        let test_1_var = Variable::new_str("test_1", ir::DataType::U32);
        let test_2_var = Variable::new_str("test_2", ir::DataType::U32);
        let test_3_var = Variable::new_str("test_3", ir::DataType::U32);
        vars.insert(test_1_var.name.clone(), test_1_var.clone());
        vars.insert(test_2_var.name.clone(), test_2_var.clone());
        vars.insert(test_3_var.name.clone(), test_3_var.clone());

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Operation(
                    ir::OP::Divide,
                    vec![
                        ir::Expression::Variable(test_1_var),
                        ir::Expression::Variable(test_2_var),
                    ],
                ),
                ir::Expression::Variable(test_3_var),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }

    #[test]
    fn reference_to_variable() {
        let tokens = &[
            (
                Token::And,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_1".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let mut vars = Variables::new();
        let test_1_var = Variable::new_str("test_1", ir::DataType::U32);
        vars.insert(test_1_var.name.clone(), test_1_var.clone());

        let expected = Some(ir::Expression::Reference(test_1_var));

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }
    #[test]
    fn dereference_variable() {
        let tokens = &[
            (
                Token::Asterisk,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_1".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
        ];

        let mut vars = Variables::new();
        let test_1_var =
            Variable::new_str("test_1", ir::DataType::Ptr(Box::new(ir::DataType::U32)));
        vars.insert(test_1_var.name.clone(), test_1_var.clone());

        let expected = Some(ir::Expression::Dereference(Box::new(
            ir::Expression::Variable(test_1_var),
        )));

        assert_eq!(expected, parse(&mut tokens.iter().peekable(), &vars));
    }
    #[test]
    fn dereference_constant() {
        let tokens = &[
            (
                Token::Asterisk,
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
        ];

        let expected = Some(ir::Expression::Dereference(Box::new(
            ir::Expression::Constant(ir::Value::I32(0)),
        )));

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &Variables::new())
        );
    }
    #[test]
    fn dereference_calc() {
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
                Token::Constant(Value::Integer(1)),
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
                Token::Constant(Value::Integer(2)),
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
        ];

        let expected = Some(ir::Expression::Dereference(Box::new(
            ir::Expression::Operation(
                ir::OP::Add,
                vec![
                    ir::Expression::Constant(ir::Value::I32(1)),
                    ir::Expression::Constant(ir::Value::I32(2)),
                ],
            ),
        )));

        assert_eq!(
            expected,
            parse(&mut tokens.iter().peekable(), &Variables::new())
        );
    }
}
