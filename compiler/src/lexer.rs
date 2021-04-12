mod seperator;
mod tokenizer;
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
    If,
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
    OpenSquareBrace,
    CloseSquareBrace,
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

#[derive(Debug, PartialEq)]
pub struct TokenMetadata {
    pub file_name: String,
    pub line: usize,
}

pub fn tokenize(content: &str, file_name: String) -> Vec<(Token, TokenMetadata)> {
    tokenizer::tokenize(content, file_name)
}
