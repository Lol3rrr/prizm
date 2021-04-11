use std::collections::HashMap;

use super::{statement, Functions, Offsets};
use crate::{asm, ir};

mod variables;

const MAPPING_START: u32 = 0x00300000;

#[derive(Debug, PartialEq)]
pub enum VariableSize {
    Byte,
    Word,
    Long,
}
#[derive(Debug, PartialEq)]
pub struct VariableMetaData {
    pub offset: u8,
    pub data_size: VariableSize,
    pub data_type: ir::DataType,
}

/// The Offsets are already stored in Two's Complement and
/// can simply be applied using the add immediate instruction
pub type VarOffset = HashMap<String, VariableMetaData>;

pub fn generate(
    func: &ir::Function,
    result: &mut Vec<asm::Instruction>,
    offsets: &mut Offsets,
    functions: &Functions,
) {
    func.pretty_print();

    let (var_offsets, stack_offset) = variables::get_offset(&func);

    let mut tmp = vec![
        asm::Instruction::Label(func.0.clone()),
        // Store the Previous FP(r14)/SP(r15) on the Stack
        asm::Instruction::Push(14),
        asm::Instruction::Push(15),
    ];

    if stack_offset > 0 {
        // Move the Stack "stack_offset" bytes up (r15 - offset)
        tmp.push(asm::Instruction::AddI(15, (stack_offset ^ 0xff) + 1));
    }

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

    let stack_reset = vec![
        asm::Instruction::AddI(15, stack_offset), // Move the Stack back
        asm::Instruction::Pop(15),                // Restore the SP
        asm::Instruction::Pop(14),                // Restore the FP
    ];
    let stack_reset_size = stack_reset.len();

    let mut ret_instrs = Vec::with_capacity(1);
    for (index, tmp_instr) in tmp.iter().enumerate() {
        match tmp_instr {
            asm::Instruction::Rts => {
                ret_instrs.push(index);
            }
            _ => {}
        };
    }

    while ret_instrs.len() > 0 {
        let mut index = ret_instrs.remove(0);
        for other in ret_instrs.iter_mut() {
            *other += stack_reset_size;
        }

        for tmp_stack in stack_reset.iter() {
            tmp.insert(index, tmp_stack.clone());
            index += 1;
        }
    }

    let raw_offset = result.len() as u32 * 2;
    offsets.insert(func.0.clone(), raw_offset + MAPPING_START);

    result.append(&mut tmp);
}
