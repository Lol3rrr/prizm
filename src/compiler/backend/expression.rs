use crate::compiler::ir;

use super::{function, internal, syscall, Functions, Offsets};

// TODO
pub fn generate(
    exp: &ir::Expression,
    pre_asm: &mut Vec<u8>,
    offsets: &mut Offsets,
    functions: &Functions,
) -> Vec<u8> {
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
                let call_target = match offsets.get(name) {
                    Some(o) => *o,
                    None => {
                        function::generate(
                            functions.get(name).unwrap(),
                            pre_asm,
                            offsets,
                            functions,
                        );

                        *offsets.get(name).unwrap()
                    }
                };

                let mut result = Vec::new();
                result.append(&mut internal::funcs::call(call_target));

                result
            }
        },
        ir::Expression::Constant(ir::Value::I32(val)) => {
            if *val == 0 {
                // XOR R0 with itself
                return vec![0x20, 0x0a];
            }

            unimplemented!("Generating i32 constant other than 0: {}", *val);

            Vec::new()
        }
        _ => {
            println!("Unknown Expression: {:?}", exp);
            Vec::new()
        }
    }
}
