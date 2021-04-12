use crate::asm::{Instruction, Operand};

/// Converts the given Instruction into its appropriate
/// ByteCode Variant that can then be run on the Calculator
pub fn serialize(instr: &Instruction) -> [u8; 2] {
    match instr {
        Instruction::Nop => [0x00, 0x09],
        Instruction::Mov(target, source) => [0x60 | (target & 0x0f), 0x03 | ((source << 4) & 0xf0)],
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
        Instruction::Xor(target, other) => [0x20 | (target & 0x0f), 0x0a | ((other << 4) & 0xf0)],
        Instruction::Add(target, other) => [0x30 | (target & 0x0f), 0x0c | ((other << 4) & 0xf0)],
        Instruction::AddI(target, value) => [0x70 | (target & 0x0f), *value],
        Instruction::MulL(first, second) => [0x00 | (first & 0x0f), (second << 4) | 0x07],
        Instruction::CmpEq(left, right) => [0x30 | (left & 0x0f), (right << 4) | 0x00],
        Instruction::CmpHs(left, right) => [0x30 | (left & 0x0f), (right << 4) | 0x02],
        Instruction::CmpHi(left, right) => [0x30 | (left & 0x0f), (right << 4) | 0x06],
        Instruction::BT(disp) => [0x89, *disp],
        Instruction::BRA(disp) => [0xa0 | (((disp & 0x0f00) >> 8) as u8), (disp & 0x00ff) as u8],
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
        _ => unimplemented!("Combination {:?} is not yet implemented", instr),
    }
}
