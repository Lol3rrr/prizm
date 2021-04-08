use crate::asm;

pub fn store_u16(register: u8, value: u16) -> Vec<asm::Instruction> {
    let bytes = value.to_be_bytes();
    vec![
        asm::Instruction::MovW(
            asm::Operand::Register(register),
            asm::Operand::Displacement8(2),
        ),
        asm::Instruction::Nop,
        asm::Instruction::BRA(1),
        asm::Instruction::Nop,
        asm::Instruction::Literal(bytes[0], bytes[1]),
    ]
}

pub fn store_u32(register: u8, value: u32) -> Vec<asm::Instruction> {
    let other_reg = if register == 1 { 0 } else { 1 };
    let mut result = vec![asm::Instruction::Push(other_reg)];

    // First 2 Bytes into the register
    result.append(&mut store_u16(register, (value >> 16) as u16));

    // Next 2 Bytes into the other register
    result.append(&mut store_u16(other_reg, value as u16));

    result.extend_from_slice(&[
        // Shift first 2 bytes into correct spot
        asm::Instruction::Shll16(register),
        // Zero out top 2 byte of other_reg
        asm::Instruction::Shll16(other_reg),
        asm::Instruction::Shlr16(other_reg),
        // Add these two together
        asm::Instruction::Add(register, other_reg),
        // Restore previous value in other_reg
        asm::Instruction::Pop(other_reg),
    ]);

    result
}
