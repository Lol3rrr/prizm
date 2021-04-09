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
    result.push(asm::Instruction::MovL(
        asm::Operand::Register(2),
        asm::Operand::Displacement8(1), // 0x00 + 0x04 + 1 * 0x04 = 0x08
    ));
    // The previous move can not be followed by a Branch instruction
    result.push(asm::Instruction::Nop);

    // JMP
    result.push(asm::Instruction::Jmp(2));
    result.push(asm::Instruction::Nop);

    // The value that will actually be loaded
    result.push(asm::Instruction::Literal(0x00, 0x00)); // PC 0x08
    result.push(asm::Instruction::Literal(0x00, 0x00)); // PC 0x0a
}
fn fixup_main_jump(result: &mut Vec<asm::Instruction>, main_offset: u32) {
    let target_bytes = main_offset.to_be_bytes();

    result[4] = asm::Instruction::Literal(target_bytes[0], target_bytes[1]);
    result[5] = asm::Instruction::Literal(target_bytes[2], target_bytes[3]);
}

// TODO
pub fn generate(mut funcs: Vec<ir::Function>) -> Vec<asm::Instruction> {
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

    result
}
