use crate::asm;

use super::store;

/// This overwrites R2 and R3
///
/// Pushes the current PR onto the stack
/// jumps to the address in a sub-routine
/// and then restores the PR again from the stack
pub fn call(address: u32) -> Vec<asm::Instruction> {
    let mut result = vec![
        asm::Instruction::PushPR, // Save previous PR, push value onto stack
    ];

    // Store the Target address into r2
    result.append(&mut store::store_u32(2, address));

    // JSR - Jump-Sub-Routine in r2
    result.push(asm::Instruction::Jsr(2));
    // Noop
    result.push(asm::Instruction::Nop);

    // Restore previous PR from stack
    result.push(asm::Instruction::PopPR);

    result
}

pub fn ret() -> Vec<asm::Instruction> {
    vec![asm::Instruction::Rts, asm::Instruction::Nop]
}
