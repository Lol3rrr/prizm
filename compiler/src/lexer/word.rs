use super::{Keyword, Token, Value};

pub fn parse(word: &str, tokens: &mut Vec<Token>) {
    match word {
        "unsigned" => tokens.push(Token::Keyword(Keyword::Unsigned)),
        "int" => tokens.push(Token::Keyword(Keyword::Integer)),
        "short" => tokens.push(Token::Keyword(Keyword::Short)),
        "void" => tokens.push(Token::Keyword(Keyword::Void)),
        "return" => tokens.push(Token::Keyword(Keyword::Return)),
        "while" => tokens.push(Token::Keyword(Keyword::While)),
        "for" => tokens.push(Token::Keyword(Keyword::For)),
        "if" => tokens.push(Token::Keyword(Keyword::If)),
        _ if !word.is_empty() => {
            if let Ok(int_value) = word.parse() {
                tokens.push(Token::Constant(Value::Integer(int_value)));
                return;
            }
            if let Ok(uint_value) = word.parse() {
                tokens.push(Token::Constant(Value::UInteger(uint_value)));
                return;
            }

            tokens.push(Token::Identifier(word.to_owned()));
        }
        _ => {}
    };
}
