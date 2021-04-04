use crate::compiler::ir;

use super::{
    function::{self, VarOffset},
    internal, syscall, Functions, Offsets,
};

// TODO
pub fn generate(
    exp: &ir::Expression,
    pre_asm: &mut Vec<u8>,
    offsets: &mut Offsets,
    functions: &Functions,
    vars: &VarOffset,
) -> Vec<u8> {
    match exp {
        ir::Expression::Call(name, exps) => match name.as_str() {
            "__syscall" => {
                if exps.len() != 5 {
                    panic!("Invalid Argument Count for Syscall, expected 5 (ID, p1, p2, p3, p4)");
                }

                let syscall_id = match exps.get(0) {
                    Some(ir::Expression::Constant(ir::Value::I32(val))) => *val,
                    _ => return Vec::new(),
                };

                let mut result = Vec::new();

                for i in 1..=4 {
                    result.append(&mut generate(
                        exps.get(i).unwrap(),
                        pre_asm,
                        offsets,
                        functions,
                        vars,
                    ));
                    result.extend_from_slice(&[0x63 + i as u8, 0x03]);
                }

                result.append(&mut syscall::generate(syscall_id as u16));
                result
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

                // TODO
                // Generate arguments

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

            internal::store::store_u32(0, *val as u32)
        }
        ir::Expression::Constant(ir::Value::U32(val)) => {
            if *val == 0 {
                // XOR R0 with itself
                return vec![0x20, 0x0a];
            }

            internal::store::store_u32(0, *val)
        }

        ir::Expression::Reference(var_name) => {
            let mut result = Vec::new();

            // Load FP into R0 (mov R14 -> R0)
            result.push(0x60);
            result.push(0xe3);

            // Add the Var-Offset to R0
            let offset = *vars.get(var_name).unwrap();
            result.push(0x70);
            result.push(offset);

            result
        }
        _ => {
            println!("Unknown Expression: {:?}", exp);
            Vec::new()
        }
    }
}
