use super::Token;

const SEPERATORS: [(char, Token); 16] = [
    ('(', Token::OpenParan),
    (')', Token::CloseParan),
    ('[', Token::OpenSquareBrace),
    (']', Token::CloseSquareBrace),
    ('{', Token::OpenCurlyBrace),
    ('}', Token::CloseCurlyBrace),
    (';', Token::Semicolon),
    ('/', Token::Slash),
    ('*', Token::Asterisk),
    ('&', Token::And),
    (',', Token::Comma),
    ('=', Token::Equals),
    ('+', Token::Plus),
    ('-', Token::Minus),
    ('<', Token::LessThan),
    ('>', Token::GreaterThan),
];

/// Parses the given Character as a Token
pub fn parse(seperator: char) -> Option<Token> {
    let (_, tok) = SEPERATORS
        .iter()
        .find(|(elem_char, _)| *elem_char == seperator)?;
    Some(tok.clone())
}

/// Checks if the given Character is a valid seperating Token
pub fn is_token(tmp: char) -> bool {
    SEPERATORS
        .iter()
        .find(|(elem_char, _)| *elem_char == tmp)
        .is_some()
}
