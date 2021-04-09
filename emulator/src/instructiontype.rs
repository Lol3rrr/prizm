use sh::asm;

pub enum InstructionType {
    Branch,
    Other,
}

impl InstructionType {
    pub fn parse(instr: asm::Instruction) -> Self {
        match instr {
            asm::Instruction::BF(_)
            | asm::Instruction::BT(_)
            | asm::Instruction::BRA(_)
            | asm::Instruction::BSR(_)
            | asm::Instruction::Jmp(_)
            | asm::Instruction::Jsr(_)
            | asm::Instruction::Rts => InstructionType::Branch,
            _ => InstructionType::Other,
        }
    }
}
