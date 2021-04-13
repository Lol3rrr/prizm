use crate::{backend::function::VariableSize, ir};

pub fn var_size(tmp: &ir::DataType) -> VariableSize {
    match tmp {
        ir::DataType::U32 | ir::DataType::I32 | ir::DataType::Ptr(_) => VariableSize::Long,
        ir::DataType::U16 | ir::DataType::I16 => VariableSize::Word,
        ir::DataType::Void => VariableSize::Byte,
        ir::DataType::Array(other_tmp, count) => {
            let single_size = match var_size(&other_tmp) {
                VariableSize::Byte => 1,
                VariableSize::Word => 2,
                VariableSize::Long => 4,
                VariableSize::Custom(t) => t,
            };
            let size = single_size * count;
            VariableSize::Custom(size)
        }
    }
}

pub fn assign_size(tmp: &ir::DataType) -> VariableSize {
    match tmp {
        ir::DataType::U32 | ir::DataType::I32 | ir::DataType::Ptr(_) => VariableSize::Long,
        ir::DataType::U16 | ir::DataType::I16 => VariableSize::Word,
        ir::DataType::Void => VariableSize::Byte,
        ir::DataType::Array(other_tmp, _) => assign_size(&other_tmp),
    }
}
