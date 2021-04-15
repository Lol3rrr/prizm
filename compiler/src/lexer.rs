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

/// General Metadata associated with a single Token
/// that can be used to give more helping Compiler-Errors
#[derive(Debug, PartialEq, Clone)]
pub struct TokenMetadata {
    /// The File in which the Token is located
    pub file_name: String,
    /// The Line on which the Token occured
    pub line: usize,
}

/// Tokenizes the given Content into the Tokens accepted
/// by the Compiler
pub fn tokenize(content: &str, file_name: String) -> Vec<(Token, TokenMetadata)> {
    tokenizer::tokenize(content, file_name)
}
