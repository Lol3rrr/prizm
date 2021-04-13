use sh::asm;

use crate::{
    backend::{function::VariableSize, internal::get_size},
    ir,
};

pub fn get_mov(
    target: asm::Operand,
    source: asm::Operand,
    datatype: &ir::DataType,
) -> asm::Instruction {
    match get_size::assign_size(datatype) {
        VariableSize::Byte => asm::Instruction::MovB(target, source),
        VariableSize::Word => asm::Instruction::MovW(target, source),
        VariableSize::Long => asm::Instruction::MovL(target, source),
        VariableSize::Custom(_) => unimplemented!("Move for  custom Size"),
    }
}
