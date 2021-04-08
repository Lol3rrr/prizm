use crate::compiler::ir;

use super::{VarOffset, VariableMetaData, VariableSize};

fn offset(statements: &[ir::Statement], vars: &mut VarOffset, final_offset: &mut u8) {
    for tmp in statements.iter() {
        match tmp {
            ir::Statement::Declaration(name, datatype) => {
                let size: u8 = match datatype {
                    ir::DataType::I32 | ir::DataType::U32 => 4,
                    ir::DataType::Ptr(_) => 4,
                    ir::DataType::Void => 0,
                };
                let var_size = match datatype {
                    ir::DataType::U32 | ir::DataType::I32 | ir::DataType::Ptr(_) => {
                        VariableSize::Long
                    }
                    ir::DataType::Void => VariableSize::Byte,
                };

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
            ir::Statement::WhileLoop(_, _, _, tmp_statements) => {
                offset(tmp_statements, vars, final_offset);
            }
            _ => {}
        };
    }
}

pub fn get_offset(statements: &[ir::Statement]) -> (VarOffset, u8) {
    let mut vars = VarOffset::new();
    let mut final_offset = 0;

    offset(statements, &mut vars, &mut final_offset);

    (vars, final_offset)
}
