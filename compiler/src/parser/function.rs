use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Keyword, Token, TokenMetadata},
};

use super::{func_args, statements};

/// Parses the Token-Stream into a single Function defined in the Program
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Function>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let datatype = match iter.next() {
        Some((Token::Keyword(Keyword::Integer), _)) => ir::DataType::I32,
        Some((_, metadata)) => {
            println!("{:?}", metadata);
            return None;
        }
        None => return None,
    };

    let name = match iter.next() {
        Some((Token::Identifier(n), _)) => n.to_owned(),
        Some((_, metadata)) => {
            println!("{:?}", metadata);
            return None;
        }
        None => return None,
    };

    match iter.next() {
        Some((Token::OpenParan, _)) => {}
        Some((_, metadata)) => {
            println!("{:?}", metadata);
            return None;
        }
        None => return None,
    };

    let args = func_args::parse(iter)?;

    match iter.next() {
        Some((Token::OpenCurlyBrace, _)) => {}
        Some((_, metadata)) => {
            println!("{:?}", metadata);
            return None;
        }
        None => return None,
    };

    let statements = statements::parse(iter);

    Some(ir::Function(name, datatype, args, statements))
}
