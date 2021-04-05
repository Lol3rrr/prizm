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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling() {
        let expected: Vec<asm::Instruction> = vec![
            asm::Instruction::PushPR,
            asm::Instruction::AddI(2, 9),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll(2),
            asm::Instruction::AddI(2, 13),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll(2),
            asm::Instruction::AddI(2, 10),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll(2),
            asm::Instruction::AddI(2, 103),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll2(2),
            asm::Instruction::Shll(2),
            asm::Instruction::AddI(2, 8),
            asm::Instruction::Jsr(2),
            asm::Instruction::Nop,
            asm::Instruction::PopPR,
        ];

        assert_eq!(expected, call(0x12345678));
    }

    #[test]
    fn returning() {
        let expected: Vec<asm::Instruction> = vec![asm::Instruction::Rts, asm::Instruction::Nop];

        assert_eq!(expected, ret());
    }
}
