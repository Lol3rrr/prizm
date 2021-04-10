use super::Token;

pub fn parse(seperator: char) -> Option<Token> {
    match seperator {
        '(' => Some(Token::OpenParan),
        ')' => Some(Token::CloseParan),
        '{' => Some(Token::OpenCurlyBrace),
        '}' => Some(Token::CloseCurlyBrace),
        ';' => Some(Token::Semicolon),
        '/' => Some(Token::Slash),
        '*' => Some(Token::Asterisk),
        '&' => Some(Token::And),
        ',' => Some(Token::Comma),
        '=' => Some(Token::Equals),
        '+' => Some(Token::Plus),
        '-' => Some(Token::Minus),
        '<' => Some(Token::LessThan),
        '>' => Some(Token::GreaterThan),
        _ => None,
    }
}
