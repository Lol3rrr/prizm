use crate::compiler::ir;

use super::syscall;

// TODO
pub fn generate(exp: &ir::Expression) -> Vec<u8> {
    match exp {
        ir::Expression::Call(name, exps) => match name.as_str() {
            "__syscall" => {
                let syscall_id = match exps.get(0) {
                    Some(ir::Expression::Constant(ir::Value::I32(val))) => *val,
                    _ => return Vec::new(),
                };

                syscall::generate(syscall_id as u16)
            }
            _ => {
                // Save previous PR
                // Find the address of the target to jump to
                // JSR - Jump-Sub-Routine
                // Noop

                println!("Generating call to '{}'", name);
                Vec::new()
            }
        },
        _ => {
            println!("Unknown Expression: {:?}", exp);
            Vec::new()
        }
    }
}
