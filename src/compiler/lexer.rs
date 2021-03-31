#[derive(Debug, PartialEq)]
pub enum Keyword {
    Integer,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    OpenParan,
    CloseParan,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Semicolon,
    Constant(Value),
    Assignment,
    Plus,
    Minus,
}

fn parse_word(word: &str, tokens: &mut Vec<Token>) {
    match word {
        "int" => tokens.push(Token::Keyword(Keyword::Integer)),
        "return" => tokens.push(Token::Keyword(Keyword::Return)),
        "=" => tokens.push(Token::Assignment),
        "+" => tokens.push(Token::Plus),
        "-" => tokens.push(Token::Minus),
        _ if word.len() > 0 => {
            if let Ok(int_value) = word.parse() {
                tokens.push(Token::Constant(Value::Integer(int_value)));
                return;
            }

            tokens.push(Token::Identifier(word.to_owned()));
        }
        _ => {}
    };
}

fn parse_seperator(seperator: &str, tokens: &mut Vec<Token>) {
    match seperator {
        "(" => tokens.push(Token::OpenParan),
        ")" => tokens.push(Token::CloseParan),
        "{" => tokens.push(Token::OpenCurlyBrace),
        "}" => tokens.push(Token::CloseCurlyBrace),
        ";" => tokens.push(Token::Semicolon),
        _ => {}
    };
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut left_to_search = content;
    while let Some(index) = left_to_search.find(&['(', ')', '{', '}', ';', ' ', '\n'][..]) {
        let current = &left_to_search[..index];
        let seperator = &left_to_search[index..index + 1];

        parse_word(current, &mut result);
        parse_seperator(seperator, &mut result);

        left_to_search = &left_to_search[index + 1..];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_program() {
        let content = "
        int main() {
            return 0;
        }";

        let expected = vec![
            Token::Keyword(Keyword::Integer),
            Token::Identifier("main".to_owned()),
            Token::OpenParan,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Keyword(Keyword::Return),
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        assert_eq!(expected, tokenize(content));
    }

    #[test]
    fn variable_assignment() {
        let content = "int test = 2;";

        let expected = vec![
            Token::Keyword(Keyword::Integer),
            Token::Identifier("test".to_string()),
            Token::Assignment,
            Token::Constant(Value::Integer(2)),
            Token::Semicolon,
        ];

        assert_eq!(expected, tokenize(content));
    }

    #[test]
    fn arithemtic_plus() {
        let content = "2 + 2;";

        let expected = vec![
            Token::Constant(Value::Integer(2)),
            Token::Plus,
            Token::Constant(Value::Integer(2)),
            Token::Semicolon,
        ];

        assert_eq!(expected, tokenize(content));
    }
    #[test]
    fn arithemtic_minus() {
        let content = "2 - 2;";

        let expected = vec![
            Token::Constant(Value::Integer(2)),
            Token::Minus,
            Token::Constant(Value::Integer(2)),
            Token::Semicolon,
        ];

        assert_eq!(expected, tokenize(content));
    }
}
