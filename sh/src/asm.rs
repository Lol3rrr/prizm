#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Register(u8),
    AtRegister(u8),
    Displacement8(u8),
}

/// These Instructions are in the Intel Format
/// (Target, Source)
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Nop,
    Mov(u8, u8),
    MovI(u8, u8),
    MovB(Operand, Operand),
    MovW(Operand, Operand),
    MovL(Operand, Operand),
    STS(u8),
    Push(u8),
    /// In the Format of (Source, StackRegister)
    PushOther(u8, u8),
    PushPR, // STS.L
    /// The Register that contains the StackPtr
    PushPROther(u8), // STS.L,
    Pop(u8),
    /// In the Format of (Destination, StackRegister)
    PopOther(u8, u8),
    PopPR, // LDS.L
    /// The Register that contains the StackPtr
    PopPROther(u8),
    Xor(u8, u8),
    Add(u8, u8),
    AddI(u8, u8),
    /// Multiplies the two Registers together and stores
    /// the resulting value in the MACL Register
    /// Rn + Rm -> MACL
    MulL(u8, u8),
    CmpEq(u8, u8),
    /// First >= Second (unsigned)
    CmpHs(u8, u8),
    /// First >= Second (signed)
    CmpGe(u8, u8),
    /// First > Second (unsigned)
    CmpHi(u8, u8),
    /// First > Second (signed)
    CmpGt(u8, u8),
    Label(String),
    BT(u8),
    BF(u8),
    BRA(u16),
    BSR(u16),
    Jmp(u8),
    JmpLabel(String),
    Jsr(u8),
    JsrLabel(String),
    Rts,
    Shll(u8),
    Shll2(u8),
    Shll8(u8),
    Shll16(u8),
    Shlr(u8),
    Shlr2(u8),
    Shlr8(u8),
    Shlr16(u8),
    /// Loads the MACL Register into the given Register
    StsMacl(u8),
    /// Pushes the MACL Register onto the Stack,
    /// The given Register is used as the StackPtr (usually R15)
    StsLMacl(u8),
    Literal(u8, u8),
}

impl Instruction {
    pub fn to_byte(&self) -> [u8; 2] {
        match self {
            Instruction::Nop => [0x00, 0x09],
            Instruction::Mov(target, source) => {
                [0x60 | (target & 0x0f), 0x03 | ((source << 4) & 0xf0)]
            }
            Instruction::MovI(target, value) => [0xe0 | (target & 0x0f), *value],
            Instruction::MovW(Operand::Register(target), Operand::AtRegister(source)) => {
                [0x60 | (target & 0x0f), 0x01 | ((source << 4) & 0xf0)]
            }
            Instruction::MovL(Operand::Register(target), Operand::AtRegister(source)) => {
                [0x60 | (target & 0x0f), 0x02 | ((source << 4) & 0xf0)]
            }
            Instruction::MovB(Operand::AtRegister(target), Operand::Register(source)) => {
                [0x20 | (target & 0x0f), 0x00 | ((source << 4) & 0xf0)]
            }
            Instruction::MovW(Operand::AtRegister(target), Operand::Register(source)) => {
                [0x20 | (target & 0x0f), 0x01 | ((source << 4) & 0xf0)]
            }
            Instruction::MovL(Operand::AtRegister(target), Operand::Register(source)) => {
                [0x20 | (target & 0x0f), 0x02 | ((source << 4) & 0xf0)]
            }
            Instruction::MovW(Operand::Register(target), Operand::Displacement8(disp)) => {
                [0x90 | (target & 0x0f), *disp]
            }
            Instruction::MovL(Operand::Register(target), Operand::Displacement8(disp)) => {
                [0xd0 | (target & 0x0f), *disp]
            }
            Instruction::Push(register) => [0x2f, 0x06 | ((register << 4) & 0xf0)],
            Instruction::PushPR => [0x4f, 0x22],
            Instruction::Pop(register) => [0x60 | (register & 0x0f), 0xf6],
            Instruction::PopPR => [0x4f, 0x26],
            Instruction::Xor(target, other) => {
                [0x20 | (target & 0x0f), 0x0a | ((other << 4) & 0xf0)]
            }
            Instruction::Add(target, other) => {
                [0x30 | (target & 0x0f), 0x0c | ((other << 4) & 0xf0)]
            }
            Instruction::AddI(target, value) => [0x70 | (target & 0x0f), *value],
            Instruction::MulL(first, second) => [0x00 | (first & 0x0f), (second << 4) | 0x07],
            Instruction::CmpEq(left, right) => [0x30 | (left & 0x0f), (right << 4) | 0x00],
            Instruction::CmpHs(left, right) => [0x30 | (left & 0x0f), (right << 4) | 0x02],
            Instruction::CmpHi(left, right) => [0x30 | (left & 0x0f), (right << 4) | 0x06],
            Instruction::BT(disp) => [0x89, *disp],
            Instruction::BRA(disp) => {
                [0xa0 | (((disp & 0x0f00) >> 8) as u8), (disp & 0x00ff) as u8]
            }
            Instruction::BSR(disp) => [0xb0 | (((disp & 0x0f00) >> 8) as u8), (disp & 0xff) as u8],
            Instruction::Jmp(target) => [0x40 | (target & 0x0f), 0x2b],
            Instruction::Jsr(target) => [0x40 | (target & 0x0f), 0x0b],
            Instruction::Rts => [0x00, 0x0b],
            Instruction::Shll(target) => [0x40 | (target & 0x0f), 0x00],
            Instruction::Shll2(target) => [0x40 | (target & 0x0f), 0x08],
            Instruction::Shll8(target) => [0x40 | (target & 0x0f), 0x18],
            Instruction::Shll16(target) => [0x40 | (target & 0x0f), 0x28],
            Instruction::Shlr(target) => [0x40 | (target & 0x0f), 0x01],
            Instruction::Shlr2(target) => [0x40 | (target & 0x0f), 0x09],
            Instruction::Shlr8(target) => [0x40 | (target & 0x0f), 0x19],
            Instruction::Shlr16(target) => [0x40 | (target & 0x0f), 0x29],
            Instruction::StsMacl(target) => [0x00 | (target & 0x0f), 0x1a],
            Instruction::StsLMacl(stack) => [0x40 | (stack & 0x0f), 0x12],
            Instruction::Literal(first, second) => [*first, *second],
            _ => unimplemented!("Combination {:?} is not yet implemented", self),
        }
    }

    pub fn parse(raw: u16) -> Self {
        let bytes = raw.to_be_bytes();
        let nibbles = [
            (bytes[0] & 0xf0) >> 4,
            bytes[0] & 0x0f,
            (bytes[1] & 0xf0) >> 4,
            bytes[1] & 0x0f,
        ];

        match (nibbles[0], nibbles[1], nibbles[2], nibbles[3]) {
            (0x0, 0x0, 0x0, 0x9) => Self::Nop,

            (0x6, n_reg, m_reg, 0x3) => Self::Mov(n_reg, m_reg),
            (0xe, n_reg, val_1, val_2) => Self::MovI(n_reg, (val_1 << 4) | val_2),
            (0x9, n_reg, d_1, d_2) => Self::MovW(
                Operand::Register(n_reg),
                Operand::Displacement8((d_1 << 4) | d_2),
            ),
            (0xd, n_reg, d_1, d_2) => Self::MovL(
                Operand::Register(n_reg),
                Operand::Displacement8((d_1 << 4) | d_2),
            ),
            (0x6, n_reg, m_reg, 0x1) => {
                Self::MovW(Operand::Register(n_reg), Operand::AtRegister(m_reg))
            }
            (0x6, n_reg, m_reg, 0x2) => {
                Self::MovL(Operand::Register(n_reg), Operand::AtRegister(m_reg))
            }
            (0x2, n_reg, m_reg, 0x0) => {
                Self::MovB(Operand::AtRegister(n_reg), Operand::Register(m_reg))
            }
            (0x2, n_reg, m_reg, 0x1) => {
                Self::MovW(Operand::AtRegister(n_reg), Operand::Register(m_reg))
            }
            (0x2, n_reg, m_reg, 0x2) => {
                Self::MovL(Operand::AtRegister(n_reg), Operand::Register(m_reg))
            }
            (0x6, n_reg, m_reg, 0x6) => Self::PopOther(n_reg, m_reg),
            (0x4, n_reg, 0x2, 0x6) => Self::PopPROther(n_reg),
            (0x2, n_reg, m_reg, 0x6) => Self::PushOther(m_reg, n_reg),
            (0x4, n_reg, 0x2, 0x2) => Self::PushPROther(n_reg),
            (0x0, n_reg, 0x1, 0xa) => Self::StsMacl(n_reg),

            (0x8, 0xb, d_1, d_2) => Self::BF((d_1 << 4) | d_2),
            (0x8, 0x9, d_1, d_2) => Self::BT((d_1 << 4) | d_2),
            (0xa, d_1, d_2, d_3) => {
                Self::BRA(((d_1 as u16) << 8) | ((d_2 as u16) << 4) | (d_3 as u16))
            }
            (0xb, d_1, d_2, d_3) => {
                Self::BSR(((d_1 as u16) << 8) | ((d_2 as u16) << 4) | (d_3 as u16))
            }
            (0x4, m_reg, 0x2, 0xb) => Self::Jmp(m_reg),
            (0x4, m_reg, 0x0, 0xb) => Self::Jsr(m_reg),
            (0x0, 0x0, 0x0, 0xb) => Self::Rts,

            (0x3, n_reg, m_reg, 0x0) => Self::CmpEq(n_reg, m_reg),
            (0x3, n_reg, m_reg, 0x2) => Self::CmpHs(n_reg, m_reg),
            (0x3, n_reg, m_reg, 0x6) => Self::CmpHi(n_reg, m_reg),

            (0x3, n_reg, m_reg, 0xc) => Self::Add(n_reg, m_reg),
            (0x7, n_reg, val_1, val_2) => Self::AddI(n_reg, (val_1 << 4) | val_2),
            (0x0, n_reg, m_reg, 0x7) => Self::MulL(n_reg, m_reg),

            (0x4, n_reg, 0x0, 0x0) => Self::Shll(n_reg),
            (0x4, n_reg, 0x0, 0x8) => Self::Shll2(n_reg),
            (0x4, n_reg, 0x1, 0x8) => Self::Shll8(n_reg),
            (0x4, n_reg, 0x2, 0x8) => Self::Shll16(n_reg),
            (0x4, n_reg, 0x0, 0x1) => Self::Shlr(n_reg),
            (0x4, n_reg, 0x0, 0x9) => Self::Shlr2(n_reg),
            (0x4, n_reg, 0x1, 09) => Self::Shlr8(n_reg),
            (0x4, n_reg, 0x2, 0x9) => Self::Shlr16(n_reg),
            (0x2, n_reg, m_reg, 0xa) => Self::Xor(n_reg, m_reg),

            (p1, p2, p3, p4) => Self::Literal((p1 << 4) | p2, (p3 << 4) | p4),
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
