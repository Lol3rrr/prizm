use core::panic;

use internal::mov_instr;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use super::{expression, function::VarOffset, internal, Functions, Offsets};
use crate::{
    asm,
    ir::{self, Statement},
};

mod comparison;
mod condition;

/// Generate the Instructions for the given Statement
pub fn generate(
    statement: &ir::Statement,
    pre_asm: &mut Vec<asm::Instruction>,
    offsets: &mut Offsets,
    functions: &Functions,
    vars: &VarOffset,
) -> Vec<asm::Instruction> {
    match statement {
        Statement::DerefAssignment(destination, exp) => {
            let mut result = Vec::new();

            // Evaluate the Target first
            result.extend(expression::generate(
                destination,
                pre_asm,
                offsets,
                functions,
                vars,
            ));
            // Push the Destination onto the Stack
            result.push(asm::Instruction::Push(0));

            // Evaluate the Expression itself
            result.append(&mut expression::generate(
                exp, pre_asm, offsets, functions, vars,
            ));

            result.push(asm::Instruction::Pop(1));

            let op_target = asm::Operand::AtRegister(1);
            let op_source = asm::Operand::Register(0);

            let mov = match destination {
                ir::Expression::Variable(name) => {
                    let var = vars.get(name).unwrap();

                    let data_type = match &var.data_type {
                        ir::DataType::Ptr(tmp) => tmp,
                        _ => panic!("Cannot dereference something that is not a PTR-Type"),
                    };

                    // MOV R0 -> (R2)
                    mov_instr::get_mov(op_target, op_source, &data_type)
                }
                _ => asm::Instruction::MovB(op_target, op_source),
            };
            result.push(mov);

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
            let var = vars.get(name).unwrap();
            result.push(asm::Instruction::AddI(1, var.offset));

            let op_target = asm::Operand::AtRegister(1);
            let op_source = asm::Operand::Register(0);

            // MOV R0 -> (R1)
            result.push(mov_instr::get_mov(op_target, op_source, &var.data_type));

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
        Statement::WhileLoop(cond, inner) => {
            let mut result = Vec::new();
            let id: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            let start_label = format!("WHILE_START_{}", id);
            let end_label = format!("WHILE_END_{}", id);

            result.push(asm::Instruction::Label(start_label.clone()));

            result.extend(condition::generate(
                &cond,
                end_label.clone(),
                pre_asm,
                offsets,
                functions,
                vars,
            ));

            let mut generated_inner = Vec::new();
            // Generates the inner code
            for tmp in inner.iter() {
                generated_inner.append(&mut generate(tmp, pre_asm, offsets, functions, vars));
            }

            // The jump back to the top
            generated_inner.push(asm::Instruction::JmpLabel(start_label));
            // Noop
            generated_inner.push(asm::Instruction::Nop);

            result.append(&mut generated_inner);
            result.push(asm::Instruction::Label(end_label));

            result
        }
        ir::Statement::If(cond, inner) => {
            let mut result = Vec::new();
            let id: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            let end_label = format!("IF_END_{}", id);

            result.extend(condition::generate(
                cond,
                end_label.clone(),
                pre_asm,
                offsets,
                functions,
                vars,
            ));

            // Generates the inner code
            for tmp in inner.iter() {
                result.extend(generate(tmp, pre_asm, offsets, functions, vars));
            }

            result.push(asm::Instruction::Label(end_label));

            result
        }
        ir::Statement::Declaration(_, _) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::function::{VariableMetaData, VariableSize};

    #[test]
    fn assign_constant() {
        let statement = ir::Statement::Assignment(
            "test".to_owned(),
            ir::Expression::Constant(ir::Value::U32(0)),
        );

        let mut pre_asm: Vec<asm::Instruction> = Vec::new();
        let mut offsets = Offsets::new();
        let functions = Functions::new();
        let mut vars = VarOffset::new();
        vars.insert(
            "test".to_owned(),
            VariableMetaData {
                offset: (4 ^ 0xff) + 1,
                data_size: VariableSize::Word,
                data_type: ir::DataType::U32,
            },
        );

        let expected: Vec<asm::Instruction> = vec![
            asm::Instruction::Xor(0, 0),
            asm::Instruction::Mov(1, 14),
            asm::Instruction::AddI(1, 252),
            asm::Instruction::MovL(asm::Operand::AtRegister(1), asm::Operand::Register(0)),
        ];

        assert_eq!(
            expected,
            generate(&statement, &mut pre_asm, &mut offsets, &functions, &vars)
        );
    }

    #[test]
    fn assign_var_plus_const() {
        let statement = ir::Statement::Assignment(
            "test".to_owned(),
            ir::Expression::Operation(
                ir::OP::Add,
                vec![
                    ir::Expression::Variable("test".to_owned()),
                    ir::Expression::Constant(ir::Value::U32(1)),
                ],
            ),
        );

        let mut pre_asm: Vec<asm::Instruction> = Vec::new();
        let mut offsets = Offsets::new();
        let functions = Functions::new();
        let mut vars = VarOffset::new();
        vars.insert(
            "test".to_owned(),
            VariableMetaData {
                offset: (4 ^ 0xff) + 1,
                data_size: VariableSize::Long,
                data_type: ir::DataType::U32,
            },
        );

        let expected_registers = [
            0x1, 0x7fffc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000, 0x80000,
        ];

        let result = generate(&statement, &mut pre_asm, &mut offsets, &functions, &vars);

        let target_pc = (result.len() * 2) as u32 + emulator::CODE_MAPPING_OFFSET;

        let mut input = emulator::MockInput::new(vec![]);
        let mut display = emulator::MockDisplay::new();
        let mut test_em = emulator::Emulator::new_test(&mut input, &mut display, result);

        assert!(test_em.run_until(target_pc).is_ok());

        let final_registers = test_em.clone_registers();
        let final_heap = test_em.clone_heap();

        assert_eq!(expected_registers, final_registers);

        let var_offset = 0x7FFFC;
        let data = 1u32.to_be_bytes();
        assert_eq!(data[0], *final_heap.get(var_offset + 0).unwrap());
        assert_eq!(data[1], *final_heap.get(var_offset + 1).unwrap());
        assert_eq!(data[2], *final_heap.get(var_offset + 2).unwrap());
        assert_eq!(data[3], *final_heap.get(var_offset + 3).unwrap());
    }
}
