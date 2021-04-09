use super::Token;

pub fn parse(seperator: &str, tokens: &mut Vec<Token>) {
    match seperator {
        "(" => tokens.push(Token::OpenParan),
        ")" => tokens.push(Token::CloseParan),
        "{" => tokens.push(Token::OpenCurlyBrace),
        "}" => tokens.push(Token::CloseCurlyBrace),
        ";" => tokens.push(Token::Semicolon),
        "/" => tokens.push(Token::Slash),
        "*" => tokens.push(Token::Asterisk),
        "&" => tokens.push(Token::And),
        "," => tokens.push(Token::Comma),
        "=" => tokens.push(Token::Equals),
        "+" => tokens.push(Token::Plus),
        "-" => tokens.push(Token::Minus),
        "<" => tokens.push(Token::LessThan),
        ">" => tokens.push(Token::GreaterThan),
        _ => {}
    };
}
