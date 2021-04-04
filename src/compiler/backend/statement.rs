use super::{expression, function::VarOffset, internal, Functions, Offsets};
use crate::compiler::ir::{self, Statement};

pub fn generate(
    statement: &ir::Statement,
    pre_asm: &mut Vec<u8>,
    offsets: &mut Offsets,
    functions: &Functions,
    vars: &VarOffset,
) -> Vec<u8> {
    match statement {
        Statement::DerefAssignment(name, exp) => {
            let mut result = Vec::new();

            // Evaluate the expression first
            result.append(&mut expression::generate(
                exp, pre_asm, offsets, functions, vars,
            ));

            // Load FP into R1
            result.push(0x61);
            result.push(0xe3);
            // Add the Offset to R1 to get address of local variable into R1
            let offset = *vars.get(name).unwrap();
            result.push(0x71);
            result.push(offset);

            // Load the Address that is stored in the variable into R1
            result.push(0x61);
            result.push(0x12);

            // MOV.L R0 -> (R1)
            result.push(0x21);
            result.push(0x02);

            result
        }
        Statement::Assignment(name, exp) => {
            let mut result = Vec::new();

            result.append(&mut expression::generate(
                exp, pre_asm, offsets, functions, vars,
            ));

            // Load FP into R1
            result.push(0x61);
            result.push(0xe3);
            // Add the Offset to R1 to get address of local variable into R1
            let offset = *vars.get(name).unwrap();
            result.push(0x71);
            result.push(offset);

            // MOV.L R0 -> (R1)
            result.push(0x21);
            result.push(0x02);

            result
        }
        Statement::Return(exp) => {
            let mut result = Vec::new();

            result.append(&mut expression::generate(
                exp, pre_asm, offsets, functions, vars,
            ));

            result.append(&mut internal::funcs::ret());

            result
        }
        Statement::SingleExpression(exp) => {
            expression::generate(exp, pre_asm, offsets, functions, vars)
        }
        Statement::WhileLoop(left, comp, right, inner) => {
            let mut result = Vec::new();

            // Generate the Left-Side of the Expression
            result.append(&mut expression::generate(
                left, pre_asm, offsets, functions, vars,
            ));

            // Push the first result onto the stack
            result.append(&mut internal::stack::push_register(0));

            // Generate right side of the expression
            result.append(&mut expression::generate(
                right, pre_asm, offsets, functions, vars,
            ));

            // Pop left side from the stack again
            result.append(&mut internal::stack::pop_register(1));

            // R0 -> Right Side
            // R1 -> Left Side
            let n_register = 1; // usually 1 but 0 for testing right now
            let m_register = 0;

            match comp {
                ir::Comparison::Equal => {
                    result.push(0x30 | (n_register & 0x0f));
                    result.push(0x00 | ((m_register & 0x0f) << 4));
                }
            };

            let mut generated_inner = Vec::new();
            // Generates the inner code
            for tmp in inner.iter() {
                generated_inner.append(&mut generate(tmp, pre_asm, offsets, functions, vars));
            }

            // Branch over the jump to the end if the condition is true
            result.push(0x89);
            result.push(0x02);
            // Noop
            result.push(0x00);
            result.push(0x09);

            // Branch to the end of the loop
            let raw_disp: u32 = generated_inner.len() as u32 + 4;
            if raw_disp > 4094 {
                unimplemented!(
                    "Cannot support loops where the jump is more than 4094 Bytes: {}",
                    raw_disp
                );
            }
            let disp: u16 = (raw_disp / 2) as u16;
            let disp_bytes = disp.to_be_bytes();
            result.push(0xa0 | (disp_bytes[0] & 0x0f));
            result.push(disp_bytes[1]);

            // Noop
            result.push(0x00);
            result.push(0x09);

            // The jump back to the top
            let raw_back_disp = (generated_inner.len() + result.len() + 2) as u32;
            if raw_back_disp > 4096 {
                unimplemented!(
                    "Cannot support loops where the jump back is more than 4096 Bytes: {}",
                    raw_back_disp
                );
            }
            let back_disp = (((raw_back_disp / 2) ^ 0xffffffff) + 0x01) as u16; // In Two's complement
            let back_disp_bytes = back_disp.to_be_bytes();
            generated_inner.push(0xa0 | (back_disp_bytes[0] & 0x0f));
            generated_inner.push(back_disp_bytes[1]);

            // Noop
            generated_inner.push(0x00);
            generated_inner.push(0x09);

            result.append(&mut generated_inner);

            result
        }
        ir::Statement::Declaration(_, _) => Vec::new(),
        _ => {
            println!("Unexpected: {:?}", statement);
            Vec::new()
        }
    }
}
