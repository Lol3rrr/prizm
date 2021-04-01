use std::iter::Peekable;

use super::datatype::parse as parse_datatype;
use crate::compiler::{ir, lexer::Token};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<(String, ir::DataType)>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    while let Some(peeked) = iter.peek() {
        match peeked {
            Token::CloseParan => {
                iter.next();
                break;
            }
            Token::Comma => {
                iter.next();
            }
            _ => {
                let datatype = parse_datatype(iter)?;
                let name = match iter.peek() {
                    Some(Token::Identifier(n)) => {
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
    use crate::compiler::lexer::Keyword;

    #[test]
    fn no_args() {
        let tokens = &[Token::CloseParan];

        let expected = Some(vec![]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn one_arg() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test_param".to_owned()),
            Token::CloseParan,
        ];

        let expected = Some(vec![("test_param".to_string(), ir::DataType::I32)]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn two_args() {
        let tokens = &[
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test_param".to_owned()),
            Token::Comma,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test_param_2".to_owned()),
            Token::CloseParan,
        ];

        let expected = Some(vec![
            ("test_param".to_string(), ir::DataType::I32),
            ("test_param_2".to_string(), ir::DataType::I32),
        ]);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
