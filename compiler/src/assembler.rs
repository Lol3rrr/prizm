use std::collections::HashMap;

use crate::asm;

mod entry;

fn asm_to_byte(instr: Vec<asm::Instruction>) -> Vec<u8> {
    let mut result = Vec::with_capacity(instr.len() * 2);

    for tmp in instr.iter() {
        result.extend_from_slice(&tmp.to_byte());
    }

    result
}

fn find_labels(instr: &[asm::Instruction]) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    let mut offset = 0;
    for tmp in instr.iter() {
        if let asm::Instruction::Label(name) = tmp {
            result.insert(name.to_owned(), offset);
        } else {
            offset += 2;
        }
    }

    result
}

pub fn assemble(instr: Vec<asm::Instruction>) -> Vec<u8> {
    let targets = find_labels(&instr);

    let entries = entry::to_entry_list(&instr, &targets);

    let generated = entry::entries_to_asm(entries);

    asm_to_byte(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_u8(raw: Vec<[u8; 2]>) -> Vec<u8> {
        let mut result = Vec::with_capacity(raw.len() * 2);

        for tmp in raw {
            result.push(tmp[0]);
            result.push(tmp[1]);
        }

        result
    }

    #[test]
    fn simple_loop() {
        let input = vec![
            asm::Instruction::Label("Start".to_owned()),
            asm::Instruction::Add(0, 1),
            asm::Instruction::JmpLabel("Start".to_owned()),
        ];

        let expected: Vec<[u8; 2]> = vec![
            asm::Instruction::Add(0, 1).to_byte(),
            asm::Instruction::BRA((0x3 ^ 0xffff) + 1).to_byte(),
            asm::Instruction::Nop.to_byte(),
        ];

        assert_eq!(to_u8(expected), assemble(input));
    }

    #[test]
    fn nested_loop() {
        let input = vec![
            asm::Instruction::Label("outer".to_owned()),
            asm::Instruction::Add(0, 1),
            asm::Instruction::Label("inner".to_owned()),
            asm::Instruction::Add(0, 2),
            asm::Instruction::JmpLabel("inner".to_owned()),
            asm::Instruction::JmpLabel("outer".to_owned()),
        ];

        let expected = vec![
            asm::Instruction::Add(0, 1).to_byte(),
            asm::Instruction::Add(0, 2).to_byte(),
            asm::Instruction::BRA((0x3 ^ 0xffff) + 1).to_byte(),
            asm::Instruction::Nop.to_byte(),
            asm::Instruction::BRA((0x6 ^ 0xffff) + 1).to_byte(),
            asm::Instruction::Nop.to_byte(),
        ];

        assert_eq!(to_u8(expected), assemble(input));
    }

    #[test]
    fn nested_with_end_branch_loop() {
        let input = vec![
            asm::Instruction::Label("outer".to_owned()),
            asm::Instruction::Add(0, 1),
            asm::Instruction::JmpLabel("outer_end".to_owned()),
            asm::Instruction::Label("inner".to_owned()),
            asm::Instruction::Add(0, 2),
            asm::Instruction::JmpLabel("inner".to_owned()),
            asm::Instruction::JmpLabel("outer".to_owned()),
            asm::Instruction::Label("outer_end".to_owned()),
        ];

        let expected = vec![
            asm::Instruction::Add(0, 1).to_byte(),
            asm::Instruction::BRA(5).to_byte(),
            asm::Instruction::Nop.to_byte(),
            asm::Instruction::Add(0, 2).to_byte(),
            asm::Instruction::BRA((0x3 ^ 0xffff) + 1).to_byte(),
            asm::Instruction::Nop.to_byte(),
            asm::Instruction::BRA((0x8 ^ 0xffff) + 1).to_byte(),
            asm::Instruction::Nop.to_byte(),
        ];

        assert_eq!(to_u8(expected), assemble(input));
    }
}