use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Keyword, Token},
};

use super::{func_args, statements};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Function>
where
    I: Iterator<Item = &'a Token>,
{
    let datatype = match iter.next() {
        Some(Token::Keyword(Keyword::Integer)) => ir::DataType::I32,
        _ => return None,
    };

    let name = match iter.next() {
        Some(Token::Identifier(n)) => n.to_owned(),
        _ => return None,
    };

    match iter.next() {
        Some(Token::OpenParan) => {}
        _ => return None,
    };

    let args = func_args::parse(iter)?;

    match iter.next() {
        Some(Token::OpenCurlyBrace) => {}
        _ => return None,
    };

    let statements = statements::parse(iter);

    Some(ir::Function(name, datatype, args, statements))
}
