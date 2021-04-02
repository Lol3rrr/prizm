use crate::compiler::ir;

use super::{function, syscall, Functions, Offsets};

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
                // Save previous PR
                // Move Stack pointer(r15) - 4 (32bit) (0xf8 is the Two's complement of 4)
                //result.push(0x7f);
                //result.push(0xf8);
                // Move PR into R0
                //result.push(0x00);
                //result.push(0x2a);
                // Move R0 at the location of the stack pointer
                //result.push(0x2f);
                //result.push(0x02);

                // Find the address of the target to jump to and store it into r2
                let target_bytes = call_target.to_be_bytes();
                // Storing the First byte
                result.push(0xe2);
                result.push(target_bytes[0]);
                // Shift r2 one byte left and add the second byte
                result.push(0x42);
                result.push(0x18);
                result.push(0x72);
                result.push(target_bytes[1]);
                // Shift r2 one byte left and add the third byte
                result.push(0x42);
                result.push(0x18);
                result.push(0x72);
                result.push(target_bytes[2]);
                // Shift r2 one byte left and add the third byte
                result.push(0x42);
                result.push(0x18);
                result.push(0x72);
                result.push(target_bytes[3]);

                // JSR - Jump-Sub-Routine in r2
                result.push(0x42);
                result.push(0x0b);

                // Noop
                result.push(0x00);
                result.push(0x09);

                // Restore PR
                // Load the value at the Stack pointer into r0
                //result.push(0x60);
                //result.push(0xf2);
                // Move R0 into PR
                //result.push(0x40);
                //result.push(0x2a);
                // Move Stack Pointer(r15) + 4 (32bit)
                //result.push(0x7f);
                //result.push(0x04);

                result
            }
        },
        _ => {
            println!("Unknown Expression: {:?}", exp);
            Vec::new()
        }
    }
}
