use sh::asm;

mod assembler;
mod backend;
mod ir;
mod lexer;
mod optimizer;
mod parser;

// The CPU in the casio calculators is 32Bit
// Instr-DOCS: http://shared-ptr.com/sh_insns.html
// Memory-Stuff: https://www.cemetech.net/forum/viewtopic.php?t=9334
/// Returns the Raw Binary Instructions for the Calculator
pub fn compile(content: &str, file: String) -> Vec<u8> {
    let tokens = lexer::tokenize(content, file);

    let raw_ir = parser::parse(&tokens);

    let ir = optimizer::optimize(raw_ir);

    let instr = backend::generate(ir);

    assembler::assemble(instr)
}

pub fn compile_file(file: String) -> Vec<u8> {
    let content = std::fs::read_to_string(file.clone()).unwrap();
    compile(&content, file)
}
