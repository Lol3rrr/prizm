use std::iter::Peekable;

use crate::compiler::{ir, lexer::Token};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Comparison>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
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
        _ => None,
    }
}
