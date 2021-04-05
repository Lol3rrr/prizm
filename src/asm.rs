#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Register(u8),
    AtRegister(u8),
}

/// These Instructions are in the Intel Format
/// (Target, Source)
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Nop,
    Mov(u8, u8),
    MovI(u8, u8),
    MovL(Operand, Operand),
    Push(u8),
    PushPR, // STS.L
    Pop(u8),
    PopPR, // LDS.L
    Xor(u8, u8),
    AddI(u8, u8),
    CmpEq(u8, u8),
    BT(u8),
    BRA(u16),
    Jmp(u8),
    Jsr(u8),
    Rts,
    Shll(u8),
    Shll2(u8),
    Shll8(u8),
    Shll16(u8),
    Shlr(u8),
}

impl Instruction {
    pub fn to_byte(&self) -> [u8; 2] {
        match self {
            Instruction::Nop => [0x00, 0x09],
            Instruction::Mov(target, source) => {
                [0x60 | (target & 0x0f), 0x03 | ((source << 4) & 0xf0)]
            }
            Instruction::MovI(target, value) => [0xe0 | (target & 0x0f), *value],
            Instruction::MovL(Operand::Register(target), Operand::AtRegister(source)) => {
                [0x60 | (target & 0x0f), 0x02 | ((source << 4) & 0xf0)]
            }
            Instruction::MovL(Operand::AtRegister(target), Operand::Register(source)) => {
                [0x20 | (target & 0x0f), 0x02 | ((source << 4) & 0xf0)]
            }
            Instruction::Push(register) => [0x2f, 0x06 | ((register << 4) & 0xf0)],
            Instruction::PushPR => [0x4f, 0x22],
            Instruction::Pop(register) => [0x60 | (register & 0x0f), 0xf6],
            Instruction::PopPR => [0x4f, 0x26],
            Instruction::Xor(target, other) => {
                [0x20 | (target & 0x0f), 0x0a | ((other << 4) & 0xf0)]
            }
            Instruction::AddI(target, value) => [0x70 | (target & 0x0f), *value],
            Instruction::CmpEq(left, right) => [0x30 | (left & 0x0f), 0x00 | ((right << 4) & 0xf0)],
            Instruction::BT(disp) => [0x89, *disp],
            Instruction::BRA(disp) => {
                [0xa0 | (((disp & 0x0f00) >> 8) as u8), (disp & 0x00ff) as u8]
            }
            Instruction::Jmp(target) => [0x40 | (target & 0x0f), 0x2b],
            Instruction::Jsr(target) => [0x40 | (target & 0x0f), 0x0b],
            Instruction::Rts => [0x00, 0x0b],
            Instruction::Shll(target) => [0x40 | (target & 0x0f), 0x00],
            Instruction::Shll2(target) => [0x40 | (target & 0x0f), 0x08],
            Instruction::Shll8(target) => [0x40 | (target & 0x0f), 0x18],
            Instruction::Shll16(target) => [0x40 | (target & 0x0f), 0x28],
            Instruction::Shlr(target) => [0x40 | (target & 0x0f), 0x01],
            _ => unimplemented!("Combination {:?} is not yet implemented", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov() {
        // R1 -> R0
        assert_eq!([0x60, 0x13], Instruction::Mov(0, 1).to_byte());
    }
    #[test]
    fn movi() {
        // 0x12 -> R0
        assert_eq!([0xe0, 0x12], Instruction::MovI(0, 0x12).to_byte());
    }
    #[test]
    fn push() {
        // Push R0
        assert_eq!([0x2f, 0x06], Instruction::Push(0).to_byte());
    }
    #[test]
    fn pop() {
        // Pop R0
        assert_eq!([0x60, 0xf6], Instruction::Pop(0).to_byte());
    }
}
