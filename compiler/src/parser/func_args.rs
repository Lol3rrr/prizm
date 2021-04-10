use std::iter::Peekable;

use super::datatype::parse as parse_datatype;
use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<(String, ir::DataType)>>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let mut result = Vec::new();

    while let Some(peeked) = iter.peek() {
        match peeked {
            (Token::CloseParan, _) => {
                iter.next();
                break;
            }
            (Token::Comma, _) => {
                iter.next();
            }
            _ => {
                let datatype = parse_datatype(iter)?;
                let name = match iter.peek() {
                    Some((Token::Identifier(n), _)) => {
                        iter.next();
                        n.to_owned()
                    }
                    _ => return None,
                };

                result.push((name, datatype));
            }
        };
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Keyword;

    #[test]
    fn no_args() {
        let tokens = &[(
            Token::CloseParan,
            TokenMetadata {
                file_name: "test".to_string(),
                line: 1,
            },
        )];

        let expected = Some(vec![]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn one_arg() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_param".to_owned()),
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

        let expected = Some(vec![("test_param".to_string(), ir::DataType::I32)]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn two_args() {
        let tokens = &[
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("test_param".to_owned()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Comma,
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
                Token::Identifier("test_param_2".to_owned()),
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

        let expected = Some(vec![
            ("test_param".to_string(), ir::DataType::I32),
            ("test_param_2".to_string(), ir::DataType::I32),
        ]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
