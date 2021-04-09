mod seperator;
mod word;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Integer,
    Short,
    Unsigned,
    Void,
    Return,
    While,
    For,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    UInteger(u32),
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
    Comma,
    Constant(Value),
    Asterisk,
    Slash,
    And,
    Equals,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
}

const SEPERATORS: &[char] = &[
    '(', ')', '{', '}', ';', ' ', '\n', '\t', '/', '*', '&', ',', '=', '<', '>', '+', '-',
];

pub fn tokenize(content: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut left_to_search = content;
    while let Some(index) = left_to_search.find(&SEPERATORS[..]) {
        let current = &left_to_search[..index];
        let seperator = &left_to_search[index..index + 1];

        word::parse(current, &mut result);
        seperator::parse(seperator, &mut result);

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
            Token::Equals,
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

    #[test]
    fn basic_ptr_stuff() {
        let content = "int* test = 2;";

        let expected = vec![
            Token::Keyword(Keyword::Integer),
            Token::Asterisk,
            Token::Identifier("test".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(2)),
            Token::Semicolon,
        ];

        assert_eq!(expected, tokenize(content));
    }

    #[test]
    fn for_loop() {
        let content = "for (int i = 0; i < 10; i++) {
            test(i);
        }";

        let expected = vec![
            Token::Keyword(Keyword::For),
            Token::OpenParan,
            Token::Keyword(Keyword::Integer),
            Token::Identifier("i".to_string()),
            Token::Equals,
            Token::Constant(Value::Integer(0)),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::LessThan,
            Token::Constant(Value::Integer(10)),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Plus,
            Token::Plus,
            Token::CloseParan,
            Token::OpenCurlyBrace,
            Token::Identifier("test".to_string()),
            Token::OpenParan,
            Token::Identifier("i".to_string()),
            Token::CloseParan,
            Token::Semicolon,
            Token::CloseCurlyBrace,
        ];

        assert_eq!(expected, tokenize(content));
    }
}
