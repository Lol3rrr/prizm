use crate::ir;

use super::VarOffset;

mod params;
mod vars;

/// Calculates the Offsets for Variables and Arguments for the specific
/// Function and then allows the rest of the backend to easily access
/// Variables in the function
pub fn get_offset(func: &ir::Function) -> (VarOffset, u8) {
    let mut vars = VarOffset::new();
    let mut final_offset = 0;

    vars::offsets(&func.3, &mut vars, &mut final_offset);
    params::offsets(&func.2, final_offset, &mut vars);

    (vars, final_offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        backend::function::{VariableMetaData, VariableSize},
        ir::Variable,
    };

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
                ir::Statement::Declaration(Variable::new_str("test", ir::DataType::U32)),
                ir::Statement::Assignment(
                    Variable::new_str("test", ir::DataType::U32),
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
                ir::Statement::Declaration(Variable::new_str("test", ir::DataType::U32)),
                ir::Statement::Assignment(
                    Variable::new_str("test", ir::DataType::U32),
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
