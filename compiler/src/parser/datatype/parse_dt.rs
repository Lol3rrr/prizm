use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Keyword, Token, TokenMetadata},
};

/// Parses the next Datatype
///
/// Params:
/// `unsigned`: Whether or not the unsigend modifier was applied
pub fn parse<'a, I>(iter: &mut Peekable<I>, unsigned: bool) -> Option<ir::DataType>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    match iter.peek() {
        Some((Token::Keyword(tmp), _)) => {
            iter.next();

            let raw = match tmp {
                Keyword::Integer if !unsigned => ir::DataType::I32,
                Keyword::Integer if unsigned => ir::DataType::U32,
                Keyword::Short if !unsigned => ir::DataType::I16,
                Keyword::Short if unsigned => ir::DataType::U16,
                Keyword::Void => ir::DataType::Void,
                _ => return None,
            };

            match iter.peek() {
                Some((Token::Asterisk, _)) => {
                    iter.next();
                    Some(ir::DataType::Ptr(Box::new(raw)))
                }
                _ => Some(raw),
            }
        }
        _ => None,
    }
}
