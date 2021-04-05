use std::collections::HashMap;

use super::{statement, Functions, Offsets};
use crate::{asm, compiler::ir};

const MAPPING_START: u32 = 0x00300000;

/// The Offsets are already stored in Two's Complement and
/// can simply be applied using the add immediate instruction
pub type VarOffset = HashMap<String, u8>;

fn get_variable_offset(statements: &[ir::Statement]) -> (VarOffset, u8) {
    let mut vars = VarOffset::new();
    let mut final_offset = 0;

    for tmp in statements.iter() {
        if let ir::Statement::Declaration(name, datatype) = tmp {
            let size: u8 = match datatype {
                ir::DataType::I32 | ir::DataType::U32 => 4,
                ir::DataType::Ptr(_) => 4,
                ir::DataType::Void => continue,
            };

            //let var_off = if final_offset > 0 {
            //    (final_offset ^ 0xff) + 1
            //} else {
            //    final_offset
            //};
            vars.insert(name.to_owned(), final_offset);
            final_offset += size;
        }
    }

    (vars, final_offset)
}

pub fn generate(
    func: &ir::Function,
    result: &mut Vec<asm::Instruction>,
    offsets: &mut Offsets,
    functions: &Functions,
) {
    func.pretty_print();

    let (var_offsets, stack_offset) = get_variable_offset(&func.3);

    let mut tmp = Vec::new();
    // Store the Previous FP(r14)/SP(r15) on the Stack
    tmp.push(asm::Instruction::Push(14));
    tmp.push(asm::Instruction::Push(15));

    // Move the Stack "stack_offset" bytes up (r15 - offset)
    tmp.push(asm::Instruction::AddI(15, (stack_offset ^ 0xff) + 1));
    // Move the new StackPtr(r15) into FP(r14) as base offset
    tmp.push(asm::Instruction::Mov(14, 15));

    for statement in func.3.iter() {
        tmp.append(&mut statement::generate(
            statement,
            result,
            offsets,
            functions,
            &var_offsets,
        ));
    }

    // Move the Stack "stack_offset" bytes down again (r15 + offset)
    tmp.push(asm::Instruction::AddI(15, stack_offset));
    // Restore the Previous FP and SP
    tmp.push(asm::Instruction::Pop(15));
    tmp.push(asm::Instruction::Pop(14));

    let raw_offset = result.len() as u32 * 2;
    offsets.insert(func.0.clone(), raw_offset + MAPPING_START);

    result.append(&mut tmp);
}
