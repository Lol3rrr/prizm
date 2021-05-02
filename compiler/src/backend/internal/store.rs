use crate::asm;

pub fn store_u16(register: u8, value: u16) -> Vec<asm::Instruction> {
    // Optimize for the value 0
    if value == 0 {
        return vec![asm::Instruction::Xor(register, register)];
    }
    // If it can be set using a move with immediate value, do that
    if value <= 0x7f {
        return vec![asm::Instruction::MovI(register, value as u8)];
    }

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
    // Optimize for the value 0
    if value == 0 {
        return vec![asm::Instruction::Xor(register, register)];
    }
    // If it can be set using a move with immediate value, do that
    if value <= 0x7f {
        return vec![asm::Instruction::MovI(register, value as u8)];
    }
    // If the value is only 16bit or less, only generate that code
    if value <= 0xffff {
        return store_u16(register, value as u16);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u16() {
        let result = store_u16(0, 0x1234);

        let target_pc = (result.len() * 2) as u32 + emulator::CODE_MAPPING_OFFSET;

        let mut input = emulator::MockInput::new(vec![]);
        let mut display = emulator::MockDisplay::new();
        let mut test_em = emulator::Emulator::new_test(&mut input, &mut display, result);

        assert!(test_em.run_until(target_pc).is_ok());

        let expected_registers = [
            0x1234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000, 0x80000,
        ];
        let final_registers = test_em.clone_registers();

        assert_eq!(expected_registers, final_registers);
    }

    #[test]
    fn u32() {
        let result = store_u32(0, 0x12345678);

        let instr_count = result.len();
        let target_pc = (instr_count * 2) as u32 + emulator::CODE_MAPPING_OFFSET;

        let mut input = emulator::MockInput::new(vec![]);
        let mut display = emulator::MockDisplay::new();
        let mut test_em = emulator::Emulator::new_test(&mut input, &mut display, result);

        assert!(test_em.run_until(target_pc).is_ok());

        let expected_registers = [
            0x12345678, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000, 0x80000,
        ];
        let final_registers = test_em.clone_registers();

        assert_eq!(expected_registers, final_registers);
    }
}
