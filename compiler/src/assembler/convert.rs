use crate::asm;

/// Converts the Assembly to the final ByteCode
pub fn to_bytes(instr: Vec<asm::Instruction>) -> Vec<u8> {
    let mut result = Vec::with_capacity(instr.len() * 2);

    for tmp in instr.iter() {
        result.extend_from_slice(&tmp.to_byte());
    }

    result
}
