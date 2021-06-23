use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

use super::{parse, Variables};

/// Parses all the Statements between two Curly Brackets `{}`, but
/// treats them all the same (no scope based variables or the like)
pub fn parse_scope<'a, I>(
    iter: &mut Peekable<I>,
    vars: &mut Variables,
) -> Option<Vec<ir::Statement>>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    // Expect an opening curly brace at the start
    match iter.next() {
        Some((Token::OpenCurlyBrace, _)) => {}
        _ => return None,
    };

    let inner = parse(iter, vars);

    // Expect a closing curly brace at the end
    match iter.next() {
        Some((Token::CloseCurlyBrace, _)) => {}
        _ => return None,
    }

    Some(inner)
}
