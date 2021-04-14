use sh::asm;

use crate::{
    backend::{expression, function::VarOffset, Functions, Offsets},
    ir,
};

use super::comparison;

/// Generates the Instructions needed for the given Condition
pub fn generate(
    cond: &ir::Condition,
    end_label: String,
    pre_asm: &mut Vec<asm::Instruction>,
    offsets: &mut Offsets,
    functions: &Functions,
    vars: &VarOffset,
) -> Vec<asm::Instruction> {
    let mut result = Vec::new();

    // Generate the Left-Side of the Expression
    result.append(&mut expression::generate(
        &cond.left, pre_asm, offsets, functions, vars,
    ));

    // Push the first result onto the stack
    result.push(asm::Instruction::Push(0));

    // Generate right side of the expression
    result.append(&mut expression::generate(
        &cond.right,
        pre_asm,
        offsets,
        functions,
        vars,
    ));

    // Pop left side from the stack again
    result.push(asm::Instruction::Pop(1));

    // R0 -> Right Side
    // R1 -> Left Side
    let n_register = 1;
    let m_register = 0;

    let comp_instr = comparison::generate(&cond.comparison, n_register, m_register, false).unwrap();
    result.push(comp_instr);

    // Branch over the jump to the end if the condition is true
    result.push(asm::Instruction::BT(1));
    // NO Nop needed because its not a delayed branch

    // Branch to the end of the loop
    result.push(asm::Instruction::JmpLabel(end_label));
    // Noop
    result.push(asm::Instruction::Nop);

    result
}
