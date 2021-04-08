use crate::{asm, compiler::ir};

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

            let target_op = asm::Operand::Register(0);
            let source_op = asm::Operand::AtRegister(0);
            let mov = match var.data_size {
                function::VariableSize::Byte => asm::Instruction::MovB(target_op, source_op),
                function::VariableSize::Word => asm::Instruction::MovW(target_op, source_op),
                function::VariableSize::Long => asm::Instruction::MovL(target_op, source_op),
            };

            vec![
                asm::Instruction::Mov(0, 14),
                asm::Instruction::AddI(0, var.offset),
                mov,
            ]
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

            let op_instr = match op {
                ir::OP::Add => asm::Instruction::Add(0, 1),
                _ => unimplemented!("Operation: {:?}", op),
            };
            result.push(op_instr);

            result.push(asm::Instruction::Pop(1));

            result
        }
        _ => {
            println!("Unknown Expression: {:?}", exp);
            Vec::new()
        }
    }
}
