use crate::asm;

pub fn store_u16(register: u8, value: u16) -> Vec<asm::Instruction> {
    const RAW_MASK: u16 = 0x7f;
    const F_MASK: u16 = RAW_MASK << 9;
    const S_MASK: u16 = RAW_MASK << 2;
    const T_MASK: u16 = RAW_MASK >> 5;

    vec![
        // Store first 7 bit
        asm::Instruction::MovI(register, ((value & F_MASK) >> 9) as u8),
        // Shift 8 bit left
        asm::Instruction::Shll8(register),
        // Shift 1 bit right
        asm::Instruction::Shlr(register),
        // Add the next 7 bit
        asm::Instruction::AddI(register, ((value & S_MASK) >> 2) as u8),
        // Shift 2 bit left
        asm::Instruction::Shll2(register),
        // Add the last 2 bit
        asm::Instruction::AddI(register, (value & T_MASK) as u8),
    ]
}

pub fn store_u32(register: u8, value: u32) -> Vec<asm::Instruction> {
    const RAW_MASK: u32 = 0x7f_u32;
    const MASKS: [u32; 5] = [
        RAW_MASK << 25,
        RAW_MASK << 18,
        RAW_MASK << 11,
        RAW_MASK << 4,
        RAW_MASK >> 3,
    ];

    let mut result: Vec<asm::Instruction> = vec![asm::Instruction::Xor(register, register)];
    for (index, mask) in MASKS.iter().enumerate() {
        let shift = 32_usize.saturating_sub((index + 1) * 7);

        result.push(asm::Instruction::AddI(
            register,
            ((value & mask) >> shift) as u8,
        ));
        if shift > 0 {
            result.push(asm::Instruction::Shll2(register));
            result.push(asm::Instruction::Shll2(register));
            result.push(asm::Instruction::Shll2(register));
            result.push(asm::Instruction::Shll(register));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u16_0x31f0() {
        let expected: Vec<asm::Instruction> = vec![
            asm::Instruction::MovI(0, 24),
            asm::Instruction::Shll8(0),
            asm::Instruction::Shlr(0),
            asm::Instruction::AddI(0, 124),
            asm::Instruction::Shll2(0),
            asm::Instruction::AddI(0, 0),
        ];

        assert_eq!(expected, store_u16(0, 0x31f0));
    }
}
