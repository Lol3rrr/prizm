use crate::{asm, ir};

use super::{
    function::{self, VarOffset},
    internal, syscall, Functions, Offsets,
};

// TODO
pub fn generate(
    exp: &ir::Expression,
    pre_asm: &mut Vec<asm::Instruction>,
    offsets: &mut Offsets,
    functions: &Functions,
    vars: &VarOffset,
) -> Vec<asm::Instruction> {
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
                    result.push(asm::Instruction::Mov(3 + i as u8, 0));
                }

                result.append(&mut syscall::generate(syscall_id as u16));
                result
            }
            _ => {
                let mut result = Vec::new();

                let arg_count = exps.len();
                // Generate arguments
                for arg_exp in exps.iter().rev() {
                    result.extend(generate(arg_exp, pre_asm, offsets, functions, vars));
                    result.push(asm::Instruction::Push(0));
                }

                // Save Previous PR
                result.push(asm::Instruction::PushPR);
                // Jump-To-Subroutine to actually execute function#
                result.push(asm::Instruction::JsrLabel(name.to_string()));
                result.push(asm::Instruction::Nop);
                // Restore Previous PR
                result.push(asm::Instruction::PopPR);

                // "Popping" all the Arguments from the Stack without storing
                // them anywhere
                for _ in 0..arg_count {
                    result.push(asm::Instruction::AddI(15, 4));
                }

                result
            }
        },
        ir::Expression::Constant(ir::Value::I32(val)) => {
            if *val == 0 {
                // XOR R0 with itself
                return vec![asm::Instruction::Xor(0, 0)];
            }

            internal::store::store_u32(0, *val as u32)
        }
        ir::Expression::Constant(ir::Value::U32(val)) => {
            if *val == 0 {
                // XOR R0 with itself
                return vec![asm::Instruction::Xor(0, 0)];
            }

            internal::store::store_u32(0, *val)
        }
        ir::Expression::Variable(var_name) => {
            let var = vars.get(var_name).unwrap();

            match var.data_type {
                ir::DataType::Array(_, _) => {
                    vec![
                        // Load FP into R0 (mov R14 -> R0)
                        asm::Instruction::Mov(0, 14),
                        // Add the Var-Offset to R0
                        asm::Instruction::AddI(0, var.offset),
                    ]
                }
                _ => {
                    let target_op = asm::Operand::Register(0);
                    let source_op = asm::Operand::AtRegister(1);

                    vec![
                        asm::Instruction::Push(1),
                        asm::Instruction::Mov(1, 14),
                        asm::Instruction::AddI(1, var.offset),
                        internal::mov_instr::get_mov(target_op, source_op, &var.data_type),
                        asm::Instruction::Pop(1),
                    ]
                }
            }
        }
        ir::Expression::Reference(var_name) => {
            let var = vars.get(var_name).unwrap();

            vec![
                // Load FP into R0 (mov R14 -> R0)
                asm::Instruction::Mov(0, 14),
                // Add the Var-Offset to R0
                asm::Instruction::AddI(0, var.offset),
            ]
        }
        ir::Expression::Operation(op, parts) => {
            let mut result = Vec::new();

            result.push(asm::Instruction::Push(1));

            let second = parts.get(1).unwrap();
            result.extend(generate(second, pre_asm, offsets, functions, vars));
            result.push(asm::Instruction::Push(0));

            let first = parts.get(0).unwrap();
            result.extend(generate(first, pre_asm, offsets, functions, vars));
            result.push(asm::Instruction::Pop(1));

            let op_instrs = match op {
                ir::OP::Add => vec![asm::Instruction::Add(0, 1)],
                ir::OP::Multiply => {
                    vec![asm::Instruction::MulL(0, 1), asm::Instruction::StsMacl(0)]
                }
                _ => unimplemented!("Operation: {:?}", op),
            };
            result.extend(op_instrs);

            result.push(asm::Instruction::Pop(1));

            result
        }
        ir::Expression::Indexed(root, offset) => {
            let mut result = Vec::new();
            // TODO
            // Replace this constant with a dynamic value based
            // on the size of the Elements in the Array
            const ELEMENT_SIZE: u8 = 4;

            result.push(asm::Instruction::Push(1));

            // Generate the Root
            result.extend(generate(root, pre_asm, offsets, functions, vars));
            result.push(asm::Instruction::Push(0));

            // Generate the Offset
            result.extend(generate(offset, pre_asm, offsets, functions, vars));
            result.push(asm::Instruction::MovI(1, ELEMENT_SIZE));
            result.push(asm::Instruction::MulL(0, 1));
            result.push(asm::Instruction::StsMacl(0));

            // Add them together
            result.push(asm::Instruction::Pop(1));
            result.push(asm::Instruction::Add(0, 1));

            result.push(asm::Instruction::Pop(1));

            result
        }
        ir::Expression::Dereference(exp) => {
            let mut result = Vec::new();

            result.extend(generate(exp, pre_asm, offsets, functions, vars));
            // Replace this MovL with the appropriate Mov depending
            // on the Alignment of the underlying Datatype
            let target_operand = asm::Operand::Register(0);
            let source_operand = asm::Operand::AtRegister(0);
            result.push(asm::Instruction::MovL(target_operand, source_operand));

            result
        }
        _ => {
            panic!("Unknown Expression: {:?}", exp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_add() {
        let expression = ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Constant(ir::Value::U32(1)),
                ir::Expression::Constant(ir::Value::U32(2)),
            ],
        );

        let mut pre_asm: Vec<asm::Instruction> = Vec::new();
        let mut offsets = Offsets::new();
        let functions = Functions::new();
        let vars = VarOffset::new();

        let result = generate(&expression, &mut pre_asm, &mut offsets, &functions, &vars);

        let target_pc = (result.len() * 2) as u32 + emulator::CODE_MAPPING_OFFSET;

        let mut input = emulator::MockInput::new(vec![]);
        let mut display = emulator::MockDisplay::new();
        let mut test_em = emulator::Emulator::new_test(&mut input, &mut display, result);

        assert!(test_em.run_until(target_pc).is_ok());

        let expected_registers = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000, 0x80000];
        let final_registers = test_em.clone_registers();

        assert_eq!(expected_registers, final_registers);
    }
}
