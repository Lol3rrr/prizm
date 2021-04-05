use std::collections::HashMap;

use crate::asm;

use super::ir;

mod expression;
mod function;
mod internal;
mod statement;
mod syscall;

pub type Offsets = HashMap<String, u32>;
pub type Functions = HashMap<String, ir::Function>;

fn initial_main_jump(result: &mut Vec<asm::Instruction>) {
    // Storing the First byte
    result.push(asm::Instruction::MovI(2, 0));
    // Shift r2 one byte left
    result.push(asm::Instruction::Shll8(2));
    // Add the Second Byte
    result.push(asm::Instruction::AddI(2, 0));
    // Shift r2 one byte left
    result.push(asm::Instruction::Shll8(2));
    // Add the Third Byte
    result.push(asm::Instruction::AddI(2, 0));
    // Shift r2 one byte left
    result.push(asm::Instruction::Shll8(2));
    // Add the Third Byte
    result.push(asm::Instruction::AddI(2, 0));

    // JMP
    result.push(asm::Instruction::Jmp(2));
    // Noop
    result.push(asm::Instruction::Nop);
}
fn fixup_main_jump(result: &mut Vec<asm::Instruction>, main_offset: u32) {
    let target_bytes = main_offset.to_be_bytes();
    result[0] = asm::Instruction::MovI(2, target_bytes[0]);
    result[2] = asm::Instruction::AddI(2, target_bytes[1]);
    result[4] = asm::Instruction::AddI(2, target_bytes[2]);
    result[6] = asm::Instruction::AddI(2, target_bytes[3]);
}

// TODO
pub fn generate(mut funcs: Vec<ir::Function>) -> Vec<u8> {
    let mut result = Vec::new();

    // The Jump to main
    initial_main_jump(&mut result);

    let mut functions = HashMap::<String, ir::Function>::new();
    for tmp in funcs.drain(..) {
        functions.insert(tmp.0.clone(), tmp);
    }

    let main_func = functions.get("main").unwrap();

    let mut offsets = HashMap::new();
    function::generate(main_func, &mut result, &mut offsets, &functions);

    fixup_main_jump(&mut result, *offsets.get("main").unwrap());

    let mut final_result = Vec::new();
    for instr in result {
        let tmp = instr.to_byte();
        final_result.push(tmp[0]);
        final_result.push(tmp[1]);
    }

    final_result
}
