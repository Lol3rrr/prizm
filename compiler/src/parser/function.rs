use std::iter::Peekable;

use crate::{
    ir,
    lexer::{Token, TokenMetadata},
};

use super::{datatype, func_args, statements};

/// Parses the Token-Stream into a single Function defined in the Program
///
/// # Example
/// ```rust
/// # use compiler::lexer::{Token, TokenMetadata, Keyword};
/// # use compiler::parser::function::parse;
/// # let empty_metadata = TokenMetadata { file_name: "test".to_owned(), line: 1, };
/// let tokens = &[
///     (Token::Keyword(Keyword::Void), empty_metadata.clone()),
///     (Token::Identifier("test".to_owned()), empty_metadata.clone()),
///     (Token::OpenParan, empty_metadata.clone()),
///     (Token::CloseParan, empty_metadata.clone()),
///     (Token::OpenCurlyBrace, empty_metadata.clone()),
///     (Token::CloseCurlyBrace, empty_metadata.clone()),
/// ];
///
/// // Parse the Tokens
/// let mut iter = tokens.iter().peekable();
/// parse(&mut iter);
///
/// // Expect
/// assert_eq!(None, iter.next());
/// ```
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Function>
where
    I: Iterator<Item = &'a (Token, TokenMetadata)>,
{
    let dt = datatype::parse(iter)?;

    let name = match iter.next() {
        Some((Token::Identifier(n), _)) => n.to_owned(),
        Some((_, metadata)) => {
            println!("Expected Identifier: {:?}", metadata);
            return None;
        }
        None => return None,
    };

    match iter.next() {
        Some((Token::OpenParan, _)) => {}
        Some((_, metadata)) => {
            println!("Expected Open-Paranthese: {:?}", metadata);
            return None;
        }
        None => return None,
    };

    let args = func_args::parse(iter)?;

    match iter.next() {
        Some((Token::OpenCurlyBrace, _)) => {}
        Some((_, metadata)) => {
            println!("Expected Open-Curly-Brace: {:?}", metadata);
            return None;
        }
        None => return None,
    };

    let statements = statements::parse(iter);

    match iter.next() {
        Some((Token::CloseCurlyBrace, _)) => {}
        Some((_, metadata)) => {
            println!("Expected Closing-Curly-Brace: {:?}", metadata);
            return None;
        }
        None => return None,
    };

    Some(ir::Function(name, dt, args, statements))
}
