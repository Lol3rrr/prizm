use crate::compiler::{backend::function::VariableSize, ir};

pub fn get_size(tmp: &ir::DataType) -> VariableSize {
    match tmp {
        ir::DataType::U32 | ir::DataType::I32 | ir::DataType::Ptr(_) => VariableSize::Long,
        ir::DataType::Void => VariableSize::Byte,
    }
}
