use crate::asm;

pub fn ret() -> Vec<asm::Instruction> {
    vec![asm::Instruction::Rts, asm::Instruction::Nop]
}
