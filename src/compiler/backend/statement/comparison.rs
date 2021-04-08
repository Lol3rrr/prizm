use crate::{asm, compiler::ir};

pub fn generate(
    comp: &ir::Comparison,
    left_reg: u8,
    right_reg: u8,
    signed: bool,
) -> Option<asm::Instruction> {
    match comp {
        ir::Comparison::Equal => Some(asm::Instruction::CmpEq(left_reg, right_reg)),
        ir::Comparison::LessThan if signed => Some(asm::Instruction::CmpGe(right_reg, left_reg)),
        ir::Comparison::LessThan if !signed => Some(asm::Instruction::CmpHs(right_reg, left_reg)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comp_eq_unsigned() {
        let expected = Some(asm::Instruction::CmpEq(0, 1));

        assert_eq!(expected, generate(&ir::Comparison::Equal, 0, 1, false));
    }
    #[test]
    fn comp_eq_signed() {
        let expected = Some(asm::Instruction::CmpEq(0, 1));

        assert_eq!(expected, generate(&ir::Comparison::Equal, 0, 1, true));
    }

    #[test]
    fn comp_less_than_unsigned() {
        let expected = Some(asm::Instruction::CmpHs(1, 0));

        assert_eq!(expected, generate(&ir::Comparison::LessThan, 0, 1, false));
    }
    #[test]
    fn comp_less_than_signed() {
        let expected = Some(asm::Instruction::CmpGe(1, 0));

        assert_eq!(expected, generate(&ir::Comparison::LessThan, 0, 1, true));
    }
}
