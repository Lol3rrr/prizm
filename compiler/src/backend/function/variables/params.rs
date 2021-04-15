use crate::{
    backend::{
        function::{VarOffset, VariableMetaData, VariableSize},
        internal,
    },
    ir,
};

// The initial Offset is 4, because there will always be
// the 32bit return PR-Value stored on the stack as well
// as both the previous SP and FP
const INITIAL_OFFSET: u8 = 4 * 3;

/// Calculates the Offsets for the Parameters passed to the Function
pub fn offsets(params: &[(String, ir::DataType)], var_stack_offset: u8, vars: &mut VarOffset) {
    let mut current_offset = INITIAL_OFFSET;
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
