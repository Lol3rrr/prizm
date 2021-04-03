use super::{expression, internal, Functions, Offsets};
use crate::compiler::ir::{self, Statement};

pub fn generate(
    statement: &ir::Statement,
    pre_asm: &mut Vec<u8>,
    offsets: &mut Offsets,
    functions: &Functions,
) -> Vec<u8> {
    match statement {
        Statement::DerefAssignment(name, exp) => {
            let mut result = Vec::new();

            println!("Deref-Assign: {} = {:?}", name, exp);

            // Evaluate the expression first
            result.append(&mut expression::generate(exp, pre_asm, offsets, functions));

            // TODO
            // Actually assign the resulting value to the variable

            result
        }
        Statement::Return(exp) => {
            let mut result = Vec::new();

            result.append(&mut expression::generate(exp, pre_asm, offsets, functions));

            result.append(&mut internal::funcs::ret());

            result
        }
        Statement::SingleExpression(exp) => expression::generate(exp, pre_asm, offsets, functions),
        _ => {
            println!("Unexpected: {:?}", statement);
            Vec::new()
        }
    }
}
