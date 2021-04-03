// Frame Pointer -> R14
// Stack Pointer -> R15

pub fn push_register(reg: u8) -> Vec<u8> {
    let register_num = reg & 0x0f;

    let mut result = Vec::new();

    // R15 - 4 -> R15, Register -> (R15)
    result.push(0x2f);
    result.push(0x06 | (register_num << 4));

    result
}

pub fn pop_register(reg: u8) -> Vec<u8> {
    let register_num = reg & 0x0f;

    let mut result = Vec::new();

    // (R15) -> Register, R15 + 4 -> R15
    result.push(0x60 | register_num);
    result.push(0xf6);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_r0() {
        let expected: Vec<u8> = vec![0x2f, 0x06];

        assert_eq!(expected, push_register(0));
    }
    #[test]
    fn push_r9() {
        let expected: Vec<u8> = vec![0x2f, 0x96];

        assert_eq!(expected, push_register(9));
    }

    #[test]
    fn pop_r0() {
        let expected: Vec<u8> = vec![0x60, 0xf6];

        assert_eq!(expected, pop_register(0));
    }
    #[test]
    fn pop_r9() {
        let expected: Vec<u8> = vec![0x69, 0xf6];

        assert_eq!(expected, pop_register(9));
    }
}
