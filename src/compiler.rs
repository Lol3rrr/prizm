mod backend;
mod ir;
mod lexer;
mod parser;

// The CPU in the casio calculators is 32Bit
pub fn compile(content: &str) -> Vec<u8> {
    let tokens = lexer::tokenize(content);

    let ir = parser::parse(&tokens);

    backend::generate(ir)
}