use super::{Keyword, Token, Value};

pub fn parse(word: &str) -> Option<Token> {
    match word {
        "unsigned" => Some(Token::Keyword(Keyword::Unsigned)),
        "int" => Some(Token::Keyword(Keyword::Integer)),
        "short" => Some(Token::Keyword(Keyword::Short)),
        "void" => Some(Token::Keyword(Keyword::Void)),
        "return" => Some(Token::Keyword(Keyword::Return)),
        "while" => Some(Token::Keyword(Keyword::While)),
        "for" => Some(Token::Keyword(Keyword::For)),
        "if" => Some(Token::Keyword(Keyword::If)),
        _ if !word.is_empty() => {
            if let Ok(int_value) = word.parse() {
                return Some(Token::Constant(Value::Integer(int_value)));
            }
            if let Ok(uint_value) = word.parse() {
                return Some(Token::Constant(Value::UInteger(uint_value)));
            }

            Some(Token::Identifier(word.to_owned()))
        }
        _ => None,
    }
}
