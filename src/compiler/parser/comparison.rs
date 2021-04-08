use std::iter::Peekable;

use crate::compiler::{ir, lexer::Token};

/// Parses the Comparison Operator itself only consuming 2 entries
/// from the Iterator and most and only consuming them if they
/// are actually part of the Comparison
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Comparison>
where
    I: Iterator<Item = &'a Token>,
{
    let peeked = iter.peek();
    match peeked {
        Some(Token::Equals) => {
            iter.next();

            match iter.peek() {
                Some(Token::Equals) => {
                    iter.next();

                    Some(ir::Comparison::Equal)
                }
                _ => None,
            }
        }
        Some(Token::LessThan) => {
            iter.next();

            Some(ir::Comparison::LessThan)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_equals() {
        let tokens = &[Token::Equals, Token::Equals];

        let expected = Some(ir::Comparison::Equal);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn parse_less_than() {
        let tokens = &[Token::LessThan];

        let expected = Some(ir::Comparison::LessThan);

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
