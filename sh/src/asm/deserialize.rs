use crate::asm::{Instruction, Operand};

/// Parses the given 16-Bit-Instruction
pub fn deserialize(raw: u16) -> Instruction {
    let bytes = raw.to_be_bytes();
    let nibbles = [
        (bytes[0] & 0xf0) >> 4,
        bytes[0] & 0x0f,
        (bytes[1] & 0xf0) >> 4,
        bytes[1] & 0x0f,
    ];

    match (nibbles[0], nibbles[1], nibbles[2], nibbles[3]) {
        (0x0, 0x0, 0x0, 0x9) => Instruction::Nop,

        (0x6, n_reg, m_reg, 0x3) => Instruction::Mov(n_reg, m_reg),
        (0xe, n_reg, val_1, val_2) => Instruction::MovI(n_reg, (val_1 << 4) | val_2),
        (0x9, n_reg, d_1, d_2) => Instruction::MovW(
            Operand::Register(n_reg),
            Operand::Displacement8((d_1 << 4) | d_2),
        ),
        (0xd, n_reg, d_1, d_2) => Instruction::MovL(
            Operand::Register(n_reg),
            Operand::Displacement8((d_1 << 4) | d_2),
        ),
        (0x6, n_reg, m_reg, 0x1) => {
            Instruction::MovW(Operand::Register(n_reg), Operand::AtRegister(m_reg))
        }
        (0x6, n_reg, m_reg, 0x2) => {
            Instruction::MovL(Operand::Register(n_reg), Operand::AtRegister(m_reg))
        }
        (0x2, n_reg, m_reg, 0x0) => {
            Instruction::MovB(Operand::AtRegister(n_reg), Operand::Register(m_reg))
        }
        (0x2, n_reg, m_reg, 0x1) => {
            Instruction::MovW(Operand::AtRegister(n_reg), Operand::Register(m_reg))
        }
        (0x2, n_reg, m_reg, 0x2) => {
            Instruction::MovL(Operand::AtRegister(n_reg), Operand::Register(m_reg))
        }
        (0x6, n_reg, m_reg, 0x6) => Instruction::PopOther(n_reg, m_reg),
        (0x4, n_reg, 0x2, 0x6) => Instruction::PopPROther(n_reg),
        (0x2, n_reg, m_reg, 0x6) => Instruction::PushOther(m_reg, n_reg),
        (0x4, n_reg, 0x2, 0x2) => Instruction::PushPROther(n_reg),
        (0x0, n_reg, 0x1, 0xa) => Instruction::StsMacl(n_reg),

        (0x8, 0xb, d_1, d_2) => Instruction::BF((d_1 << 4) | d_2),
        (0x8, 0x9, d_1, d_2) => Instruction::BT((d_1 << 4) | d_2),
        (0xa, d_1, d_2, d_3) => {
            Instruction::BRA(((d_1 as u16) << 8) | ((d_2 as u16) << 4) | (d_3 as u16))
        }
        (0xb, d_1, d_2, d_3) => {
            Instruction::BSR(((d_1 as u16) << 8) | ((d_2 as u16) << 4) | (d_3 as u16))
        }
        (0x4, m_reg, 0x2, 0xb) => Instruction::Jmp(m_reg),
        (0x4, m_reg, 0x0, 0xb) => Instruction::Jsr(m_reg),
        (0x0, 0x0, 0x0, 0xb) => Instruction::Rts,

        (0x3, n_reg, m_reg, 0x0) => Instruction::CmpEq(n_reg, m_reg),
        (0x3, n_reg, m_reg, 0x2) => Instruction::CmpHs(n_reg, m_reg),
        (0x3, n_reg, m_reg, 0x6) => Instruction::CmpHi(n_reg, m_reg),

        (0x3, n_reg, m_reg, 0xc) => Instruction::Add(n_reg, m_reg),
        (0x7, n_reg, val_1, val_2) => Instruction::AddI(n_reg, (val_1 << 4) | val_2),
        (0x0, n_reg, m_reg, 0x7) => Instruction::MulL(n_reg, m_reg),

        (0x4, n_reg, 0x0, 0x0) => Instruction::Shll(n_reg),
        (0x4, n_reg, 0x0, 0x8) => Instruction::Shll2(n_reg),
        (0x4, n_reg, 0x1, 0x8) => Instruction::Shll8(n_reg),
        (0x4, n_reg, 0x2, 0x8) => Instruction::Shll16(n_reg),
        (0x4, n_reg, 0x0, 0x1) => Instruction::Shlr(n_reg),
        (0x4, n_reg, 0x0, 0x9) => Instruction::Shlr2(n_reg),
        (0x4, n_reg, 0x1, 09) => Instruction::Shlr8(n_reg),
        (0x4, n_reg, 0x2, 0x9) => Instruction::Shlr16(n_reg),
        (0x2, n_reg, m_reg, 0xa) => Instruction::Xor(n_reg, m_reg),

        (p1, p2, p3, p4) => Instruction::Literal((p1 << 4) | p2, (p3 << 4) | p4),
    }
}
