use crate::asm;

/// This overwrites R2 and R3
///
/// Pushes the current PR onto the stack
/// jumps to the address in a sub-routine
/// and then restores the PR again from the stack
pub fn call(name: String) -> Vec<asm::Instruction> {
    vec![
        asm::Instruction::PushPR,         // Save previous PR, push value onto stack
        asm::Instruction::JsrLabel(name), // Jump-Sub-Routine to the Target Label
        asm::Instruction::Nop,
        asm::Instruction::PopPR,
    ]
}

pub fn ret() -> Vec<asm::Instruction> {
    vec![asm::Instruction::Rts, asm::Instruction::Nop]
}
