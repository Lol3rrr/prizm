use std::collections::HashMap;

use crate::asm;

#[derive(Debug)]
struct Jump {
    start: u32,
    target: u32,
}

#[derive(Debug)]
enum Entry {
    Instruction(asm::Instruction),
    Jump(Jump),
}

fn to_entry_list(instr: &[asm::Instruction], targets: &HashMap<String, u32>) -> Vec<Entry> {
    let mut result = Vec::new();

    for tmp in instr {
        match tmp {
            asm::Instruction::Label(_) => {}
            asm::Instruction::JmpLabel(name) => {
                let current = result.len() as u32;
                let target = *targets.get(name).unwrap();
                result.push(Entry::Jump(Jump {
                    start: current * 2,
                    target,
                }));
            }
            _ => {
                result.push(Entry::Instruction(tmp.to_owned()));
            }
        }
    }

    result
}

fn move_entries(entries: &mut Vec<Entry>, start: usize, offset: u32) {
    for entrie in entries.iter_mut() {
        if let Entry::Jump(tmp) = entrie {
            if tmp.start > start as u32 {
                tmp.start += offset;
            }
            if tmp.target > start as u32 {
                tmp.target += offset;
            }
        }
    }
}

const SMALL_JUMP_OFFSET: u32 = 2;
fn entries_to_asm(mut entries: Vec<Entry>) -> Vec<asm::Instruction> {
    let length = entries.len();
    let mut offset = 0;
    for index in 0..length {
        if let Entry::Jump(_) = entries.get(index).unwrap() {
            move_entries(&mut entries, offset, SMALL_JUMP_OFFSET);
            offset += 2;
        }
        offset += 2;
    }

    let mut result = Vec::new();
    for tmp in entries.drain(..) {
        match tmp {
            Entry::Jump(jmp) => {
                let start = jmp.start + 4;
                let target = jmp.target;

                let delta = if target < start {
                    let delta = (start - target) / 2;
                    let jmp_offset = ((delta ^ 0xffffffff) + 1) as u16;
                    jmp_offset
                } else {
                    ((target - start) / 2) as u16
                };
                result.push(asm::Instruction::BRA(delta));
                result.push(asm::Instruction::Nop);
            }
            Entry::Instruction(instr) => {
                result.push(instr);
            }
        };
    }

    result
}

fn asm_to_byte(instr: Vec<asm::Instruction>) -> Vec<u8> {
    let mut result = Vec::with_capacity(instr.len() * 2);

    for tmp in instr.iter() {
        result.extend_from_slice(&tmp.to_byte());
    }

    result
}

fn print_instructions(instr: &[asm::Instruction]) {
    for (index, tmp) in instr.iter().enumerate() {
        println!("[{:08x}] {:?}", index * 2, tmp);
    }
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
    print_instructions(&instr);

    let targets = find_labels(&instr);

    let entries = to_entry_list(&instr, &targets);

    let generated = entries_to_asm(entries);

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
