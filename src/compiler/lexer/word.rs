use super::{Keyword, Token, Value};

pub fn parse(word: &str, tokens: &mut Vec<Token>) {
    match word {
        "int" => tokens.push(Token::Keyword(Keyword::Integer)),
        "uint" => tokens.push(Token::Keyword(Keyword::UInteger)),
        "void" => tokens.push(Token::Keyword(Keyword::Void)),
        "return" => tokens.push(Token::Keyword(Keyword::Return)),
        "while" => tokens.push(Token::Keyword(Keyword::While)),
        "for" => tokens.push(Token::Keyword(Keyword::For)),
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
