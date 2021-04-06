use std::collections::HashMap;

use crate::asm;

mod labels;

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

    let mut targets = find_labels(&instr);

    let mut final_result = Vec::new();
    for tmp_instr in instr.iter() {
        match tmp_instr {
            asm::Instruction::Label(_) => {}
            asm::Instruction::JmpLabel(name) => {
                match targets.get(name) {
                    Some(jump_target) => {
                        let current = final_result.len() as u32 + 4;
                        let target = *jump_target;

                        let offset = if target < current {
                            let delta = (current - target) / 2;
                            let jmp_offset = ((delta ^ 0xffffffff) + 1) as u16;

                            final_result
                                .extend_from_slice(&asm::Instruction::BRA(jmp_offset).to_byte());
                            final_result.extend_from_slice(&asm::Instruction::Nop.to_byte());
                            2
                        } else if target > current {
                            let delta = (target - current) / 2;

                            final_result
                                .extend_from_slice(&asm::Instruction::BRA(delta as u16).to_byte());
                            final_result.extend_from_slice(&asm::Instruction::Nop.to_byte());
                            2
                        } else {
                            unimplemented!("Jumps currently need to lead somewhere");
                        };

                        labels::move_labels(&mut targets, current, offset);
                    }
                    None => {
                        println!("Unknown Label: {}", name);
                    }
                };
            }
            _ => {
                let tmp = tmp_instr.to_byte();
                final_result.push(tmp[0]);
                final_result.push(tmp[1]);
            }
        };
    }

    final_result
}
