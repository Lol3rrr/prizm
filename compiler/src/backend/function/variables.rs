use crate::{backend::internal, ir};

use super::{VarOffset, VariableMetaData, VariableSize};

/// Calculates the Offsets for the Varialbes used in the Function itself
fn var_offset(statements: &[ir::Statement], vars: &mut VarOffset, final_offset: &mut u8) {
    for tmp in statements.iter() {
        match tmp {
            ir::Statement::Declaration(name, datatype) => {
                let var_size = internal::get_size::var_size(&datatype);

                let size: u8 = match var_size {
                    VariableSize::Long => 4,
                    VariableSize::Word => 2,
                    VariableSize::Byte => 1,
                    VariableSize::Custom(s) => {
                        if s > 127 {
                            unimplemented!("Variable too big: {}", s);
                        }
                        s as u8
                    }
                };

                if *final_offset % size != 0 {
                    println!("Unaligned: At x{:X} with size x{:X}", final_offset, size);
                }

                vars.insert(
                    name.to_owned(),
                    VariableMetaData {
                        offset: *final_offset,
                        data_size: var_size,
                        data_type: datatype.clone(),
                    },
                );
                *final_offset += size;
            }
            ir::Statement::WhileLoop(_, tmp_statements) => {
                var_offset(tmp_statements, vars, final_offset);
            }
            _ => {}
        };
    }
}

/// Calculates the Offsets for the Parameters passed to the Function
fn param_offset(params: &[(String, ir::DataType)], var_stack_offset: u8, vars: &mut VarOffset) {
    // The initial Offset is 4, because there will always be
    // the 32bit return PR-Value stored on the stack as well
    // as both the previous SP and FP
    let mut current_offset = 4 * 3;
    for param in params.iter() {
        let (name, datatype) = param;
        let var_size = internal::get_size::var_size(&datatype);

        let size: u8 = match var_size {
            VariableSize::Long => 4,
            VariableSize::Word => 2,
            VariableSize::Byte => 1,
            VariableSize::Custom(s) => {
                if s > 127 {
                    unimplemented!("Variable too big: {}", s);
                }
                s as u8
            }
        };

        let param_offset = var_stack_offset + current_offset;
        if param_offset % size != 0 {
            println!("Unaligned: At x{:X} with size x{:X}", param_offset, size);
        }

        vars.insert(
            name.to_owned(),
            VariableMetaData {
                offset: param_offset,
                data_size: var_size,
                data_type: datatype.clone(),
            },
        );
        current_offset += size;
    }
}

/// Calculates the Offsets for Variables and Arguments for the specific
/// Function and then allows the rest of the backend to easily access
/// Variables in the function
pub fn get_offset(func: &ir::Function) -> (VarOffset, u8) {
    let mut vars = VarOffset::new();
    let mut final_offset = 0;

    var_offset(&func.3, &mut vars, &mut final_offset);
    param_offset(&func.2, final_offset, &mut vars);

    (vars, final_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_vars() {
        let func = ir::Function(
            "test".to_owned(),
            ir::DataType::Void,
            vec![],
            vec![ir::Statement::SingleExpression(ir::Expression::Call(
                "test".to_owned(),
                vec![],
            ))],
        );

        let expected_varoffset = VarOffset::new();
        let expected_total_offset = 0;

        let (result_var, result_total) = get_offset(&func);

        assert_eq!(expected_varoffset, result_var);
        assert_eq!(expected_total_offset, result_total);
    }

    #[test]
    fn one_var() {
        let func = ir::Function(
            "test".to_owned(),
            ir::DataType::Void,
            vec![],
            vec![
                ir::Statement::Declaration("test".to_owned(), ir::DataType::U32),
                ir::Statement::Assignment(
                    "test".to_owned(),
                    ir::Expression::Constant(ir::Value::U32(0)),
                ),
            ],
        );

        let mut expected_varoffset = VarOffset::new();
        expected_varoffset.insert(
            "test".to_owned(),
            VariableMetaData {
                offset: 0,
                data_type: ir::DataType::U32,
                data_size: VariableSize::Long,
            },
        );
        let expected_total_offset = 4;

        let (result_var, result_total) = get_offset(&func);

        assert_eq!(expected_varoffset, result_var);
        assert_eq!(expected_total_offset, result_total);
    }

    #[test]
    fn one_param() {
        let func = ir::Function(
            "test".to_owned(),
            ir::DataType::Void,
            vec![("var".to_owned(), ir::DataType::U32)],
            vec![ir::Statement::SingleExpression(ir::Expression::Call(
                "test".to_owned(),
                vec![],
            ))],
        );

        let mut expected_varoffset = VarOffset::new();
        expected_varoffset.insert(
            "var".to_owned(),
            VariableMetaData {
                offset: 12,
                data_type: ir::DataType::U32,
                data_size: VariableSize::Long,
            },
        );

        let expected_total_offset = 0;

        let (result_var, result_total) = get_offset(&func);

        assert_eq!(expected_varoffset, result_var);
        assert_eq!(expected_total_offset, result_total);
    }

    #[test]
    fn one_param_one_var() {
        let func = ir::Function(
            "test".to_owned(),
            ir::DataType::Void,
            vec![("var".to_owned(), ir::DataType::U32)],
            vec![
                ir::Statement::Declaration("test".to_owned(), ir::DataType::U32),
                ir::Statement::Assignment(
                    "test".to_owned(),
                    ir::Expression::Constant(ir::Value::U32(0)),
                ),
            ],
        );

        let mut expected_varoffset = VarOffset::new();
        // The Variable
        expected_varoffset.insert(
            "test".to_owned(),
            VariableMetaData {
                offset: 0,
                data_type: ir::DataType::U32,
                data_size: VariableSize::Long,
            },
        );
        // The Parameter
        expected_varoffset.insert(
            "var".to_owned(),
            VariableMetaData {
                offset: 16,
                data_type: ir::DataType::U32,
                data_size: VariableSize::Long,
            },
        );

        let expected_total_offset = 4;

        let (result_var, result_total) = get_offset(&func);

        assert_eq!(expected_varoffset, result_var);
        assert_eq!(expected_total_offset, result_total);
    }
}
