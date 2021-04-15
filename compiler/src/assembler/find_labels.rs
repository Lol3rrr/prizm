use std::collections::HashMap;

use crate::asm;

/// Maps all the Labels to their "final" Offset from the current "Set" of
/// Instructions
pub fn find(instr: &[asm::Instruction]) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    let mut offset = 0;
    for tmp in instr.iter() {
        if let asm::Instruction::Label(name) = tmp {
            result.insert(name.to_owned(), offset);
        } else {
            offset += 2;
        }
    }

    result
}
