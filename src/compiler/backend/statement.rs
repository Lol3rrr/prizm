use super::{expression, function::VarOffset, internal, Functions, Offsets};
use crate::{
    asm,
    compiler::ir::{self, Statement},
};

pub fn generate(
    statement: &ir::Statement,
    pre_asm: &mut Vec<asm::Instruction>,
    offsets: &mut Offsets,
    functions: &Functions,
    vars: &VarOffset,
) -> Vec<asm::Instruction> {
    match statement {
        Statement::DerefAssignment(name, exp) => {
            let mut result = Vec::new();

            // Evaluate the expression first
            result.append(&mut expression::generate(
                exp, pre_asm, offsets, functions, vars,
            ));

            // Load FP into R1
            result.push(asm::Instruction::Mov(1, 14));
            // Add the Offset to R1 to get address of local variable into R1
            let offset = *vars.get(name).unwrap();
            result.push(asm::Instruction::AddI(1, offset));

            // Load the Address that is stored in the variable into R2
            result.push(asm::Instruction::MovL(
                asm::Operand::Register(2),
                asm::Operand::AtRegister(1),
            ));

            // MOV.L R0 -> (R2)
            result.push(asm::Instruction::MovL(
                asm::Operand::AtRegister(2),
                asm::Operand::Register(0),
            ));

            result
        }
        Statement::Assignment(name, exp) => {
            let mut result = Vec::new();

            result.append(&mut expression::generate(
                exp, pre_asm, offsets, functions, vars,
            ));

            // Load FP into R1
            result.push(asm::Instruction::Mov(1, 14));
            // Add the Offset to R1 to get address of local variable into R1
            let offset = *vars.get(name).unwrap();
            result.push(asm::Instruction::AddI(1, offset));

            // MOV.L R0 -> (R1)
            result.push(asm::Instruction::MovL(
                asm::Operand::AtRegister(1),
                asm::Operand::Register(0),
            ));

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
            result.push(asm::Instruction::Push(0));

            // Generate right side of the expression
            result.append(&mut expression::generate(
                right, pre_asm, offsets, functions, vars,
            ));

            // Pop left side from the stack again
            result.push(asm::Instruction::Pop(1));

            // R0 -> Right Side
            // R1 -> Left Side
            let n_register = 1; // usually 1 but 0 for testing right now
            let m_register = 0;

            match comp {
                ir::Comparison::Equal => {
                    result.push(asm::Instruction::CmpEq(n_register, m_register));
                }
            };

            let mut generated_inner = Vec::new();
            // Generates the inner code
            for tmp in inner.iter() {
                generated_inner.append(&mut generate(tmp, pre_asm, offsets, functions, vars));
            }

            // Branch over the jump to the end if the condition is true
            result.push(asm::Instruction::BT(1));
            // NO Nop needed because its not a delayed branch

            // Branch to the end of the loop
            let raw_disp: u32 = generated_inner.len() as u32 * 2 + 4;
            if raw_disp > 4094 {
                unimplemented!(
                    "Cannot support loops where the jump is more than 4094 Bytes: {}",
                    raw_disp
                );
            }
            let disp: u16 = (raw_disp / 2) as u16;
            result.push(asm::Instruction::BRA(disp));
            // Noop
            result.push(asm::Instruction::Nop);

            // The jump back to the top
            let raw_back_disp = (generated_inner.len() * 2 + result.len() * 2 + 4) as u32;
            if raw_back_disp > 4096 {
                unimplemented!(
                    "Cannot support loops where the jump back is more than 4096 Bytes: {}",
                    raw_back_disp
                );
            }
            let back_disp = (((raw_back_disp / 2) ^ 0xffffffff) + 0x01) as u16; // In Two's complement
            generated_inner.push(asm::Instruction::BRA(back_disp));
            // Noop
            generated_inner.push(asm::Instruction::Nop);

            result.append(&mut generated_inner);

            result
        }
        ir::Statement::Declaration(_, _) => Vec::new(),
    }
}
