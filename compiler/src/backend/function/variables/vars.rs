use crate::{
    backend::{
        function::{VarOffset, VariableMetaData, VariableSize},
        internal,
    },
    ir,
};

/// Calculates the Offsets for the Varialbes used in the Function itself
pub fn offsets(statements: &[ir::Statement], vars: &mut VarOffset, final_offset: &mut u8) {
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
                offsets(tmp_statements, vars, final_offset);
            }
            _ => {}
        };
    }
}
