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

// TODO
pub fn generate(mut funcs: Vec<ir::Function>) -> Vec<asm::Instruction> {
    let mut result = vec![asm::Instruction::JmpLabel("main".to_owned())];

    let functions = Functions::new();
    let mut offsets = HashMap::new();
    for tmp in funcs.drain(..) {
        function::generate(&tmp, &mut result, &mut offsets, &functions);
    }

    result
}
