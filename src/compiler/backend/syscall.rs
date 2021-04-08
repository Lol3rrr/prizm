use crate::asm;

use super::internal;

pub fn generate(call_id: u16) -> Vec<asm::Instruction> {
    let mut result: Vec<asm::Instruction> = Vec::new();
    result.append(&mut internal::store::store_u16(0, call_id));

    // Store the Jump address for systemcalls into r2
    result.push(asm::Instruction::MovI(2, 0x80));
    result.push(asm::Instruction::Shll8(2));
    result.push(asm::Instruction::AddI(2, 0x02));
    result.push(asm::Instruction::Shll16(2));
    result.push(asm::Instruction::AddI(2, 0x70));

    result.push(asm::Instruction::PushPR);

    // Jump
    result.push(asm::Instruction::Jsr(2));
    // Noop after jump
    result.push(asm::Instruction::Nop);

    result.push(asm::Instruction::PopPR);

    result
}
