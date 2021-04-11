use std::collections::HashMap;

use crate::asm;

#[derive(Debug)]
pub struct Jump {
    pub start: u32,
    pub target: u32,
}

#[derive(Debug)]
pub enum Entry {
    Instruction(asm::Instruction),
    Jump(Jump),
    Jsr(Jump),
}

pub fn to_entry_list(instr: &[asm::Instruction], targets: &HashMap<String, u32>) -> Vec<Entry> {
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
            asm::Instruction::JsrLabel(name) => {
                let current = result.len() as u32;
                let target = *targets.get(name).unwrap();
                result.push(Entry::Jsr(Jump {
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

pub fn move_entries(entries: &mut Vec<Entry>, start: usize, offset: u32) {
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

fn calc_delta(jmp: Jump) -> u16 {
    let start = jmp.start + 4;
    let target = jmp.target;

    if target < start {
        let delta = (start - target) / 2;
        ((delta ^ 0xffffffff) + 1) as u16
    } else {
        ((target - start) / 2) as u16
    }
}

const SMALL_JUMP_OFFSET: u32 = 2;
pub fn entries_to_asm(mut entries: Vec<Entry>) -> Vec<asm::Instruction> {
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
                let delta = calc_delta(jmp);
                result.push(asm::Instruction::BRA(delta));
                result.push(asm::Instruction::Nop);
            }
            Entry::Jsr(jmp) => {
                let delta = calc_delta(jmp);
                result.push(asm::Instruction::BSR(delta));
                result.push(asm::Instruction::Nop);
            }
            Entry::Instruction(instr) => {
                result.push(instr);
            }
        };
    }

    result
}
