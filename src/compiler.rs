mod backend;
mod ir;
mod lexer;
mod parser;

// The CPU in the casio calculators is 32Bit
// Instr-DOCS: http://shared-ptr.com/sh_insns.html
// Memory-Stuff: https://www.cemetech.net/forum/viewtopic.php?t=9334
pub fn compile(content: &str) -> Vec<u8> {
    let tokens = lexer::tokenize(content);

    let ir = parser::parse(&tokens);

    backend::generate(ir)
}
