use crate::{backend::internal, ir};

use super::{VarOffset, VariableMetaData, VariableSize};

fn offset(statements: &[ir::Statement], vars: &mut VarOffset, final_offset: &mut u8) {
    for tmp in statements.iter() {
        match tmp {
            ir::Statement::Declaration(name, datatype) => {
                let var_size = internal::get_size::get_size(&datatype);

                let size: u8 = match var_size {
                    VariableSize::Long => 4,
                    VariableSize::Word => 2,
                    VariableSize::Byte => 1,
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

#[test]
fn no_vars() {
    let statements = &[ir::Statement::SingleExpression(ir::Expression::Call(
        "test".to_owned(),
        vec![],
    ))];

    let expected_varoffset = VarOffset::new();
    let expected_total_offset = 0;

    let (result_var, result_total) = get_offset(statements);

    assert_eq!(expected_varoffset, result_var);
    assert_eq!(expected_total_offset, result_total);
}
#[test]
fn one_var() {
    let statements = &[
        ir::Statement::Declaration("test".to_owned(), ir::DataType::U32),
        ir::Statement::Assignment(
            "test".to_owned(),
            ir::Expression::Constant(ir::Value::U32(0)),
        ),
    ];

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

    let (result_var, result_total) = get_offset(statements);

    assert_eq!(expected_varoffset, result_var);
    assert_eq!(expected_total_offset, result_total);
}
