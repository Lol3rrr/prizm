use super::{statement, Functions, Offsets};
use crate::compiler::ir;

const MAPPING_START: u32 = 0x00300000;

pub fn generate(
    func: &ir::Function,
    result: &mut Vec<u8>,
    offsets: &mut Offsets,
    functions: &Functions,
) {
    func.pretty_print();

    let mut tmp = Vec::new();
    for statement in func.3.iter() {
        tmp.append(&mut statement::generate(
            statement, result, offsets, functions,
        ));
    }

    let raw_offset = result.len() as u32;
    offsets.insert(func.0.clone(), raw_offset + MAPPING_START);

    result.append(&mut tmp);
}
