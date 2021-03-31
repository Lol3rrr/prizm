mod ir;
mod lexer;
mod parser;

pub fn compile(content: &str) -> Vec<u8> {
    let mut result = Vec::new();

    let tokens = lexer::tokenize(content);

    let ir = parser::parse(&tokens);

    println!("IR: {:?}", ir);

    result
}
