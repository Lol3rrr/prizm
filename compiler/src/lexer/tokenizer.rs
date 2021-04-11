use super::{seperator, word, Token, TokenMetadata};

pub fn tokenize(content: &str, file_name: String) -> Vec<(Token, TokenMetadata)> {
    let mut result = Vec::new();
    let mut line = 1;
    let mut last_char = 0;
    let mut current = 0;

    for tmp_char in content.chars() {
        match tmp_char {
            ' ' | '\t' | '\n' => {
                let raw_word = &content[last_char..current];
                if let Some(parsed_word) = word::parse(raw_word) {
                    result.push((
                        parsed_word,
                        TokenMetadata {
                            file_name: file_name.clone(),
                            line,
                        },
                    ));
                }

                last_char = current + 1;

                if tmp_char == '\n' {
                    line += 1;
                }
            }
            ';' | '(' | ')' | '{' | '}' | '/' | '*' | '&' | ',' | '=' | '+' | '-' | '<' | '>' => {
                let raw_word = &content[last_char..current];
                if let Some(parsed_word) = word::parse(raw_word) {
                    result.push((
                        parsed_word,
                        TokenMetadata {
                            file_name: file_name.clone(),
                            line,
                        },
                    ));
                }

                if let Some(tok) = seperator::parse(tmp_char) {
                    result.push((
                        tok,
                        TokenMetadata {
                            file_name: file_name.clone(),
                            line,
                        },
                    ))
                }

                last_char = current + 1;
            }
            _ => {}
        };
        current += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Keyword, Value};

    #[test]
    fn simple_program() {
        let content = "int main() {
            return 0;
        }";

        let expected = vec![
            (
                Token::Keyword(Keyword::Integer),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Identifier("main".to_string()),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::OpenParan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::CloseParan,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::OpenCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 1,
                },
            ),
            (
                Token::Keyword(Keyword::Return),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 2,
                },
            ),
            (
                Token::Constant(Value::Integer(0)),
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 2,
                },
            ),
            (
                Token::Semicolon,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 2,
                },
            ),
            (
                Token::CloseCurlyBrace,
                TokenMetadata {
                    file_name: "test".to_string(),
                    line: 3,
                },
            ),
        ];

        assert_eq!(expected, tokenize(content, "test".to_string()));
    }
}
