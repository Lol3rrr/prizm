// Frame Pointer -> R14
// Stack Pointer -> R15

pub fn push_register(reg: u8) -> Vec<u8> {
    let register_num = reg & 0x0f;

    let mut result = Vec::new();

    // Move Stack pointer(r15) - 4 (32bit) (0xf8 is the Two's complement of 4)
    result.push(0x7f);
    result.push(0xf8);

    // Move Register at the location of the stack pointer
    result.push(0x2f);
    result.push((register_num << 4) | 0x02);

    result
}

pub fn pop_register(reg: u8) -> Vec<u8> {
    let register_num = reg & 0x0f;

    let mut result = Vec::new();

    // Load the value at the Stack pointer into r0
    result.push(0x60 | register_num);
    result.push(0xf2);
    // Move Stack Pointer(r15) + 4 (32bit)
    result.push(0x7f);
    result.push(0x04);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_r0() {
        let expected: Vec<u8> = vec![0x7f, 0xf8, 0x2f, 0x02];

        assert_eq!(expected, push_register(0));
    }
    #[test]
    fn push_r9() {
        let expected: Vec<u8> = vec![0x7f, 0xf8, 0x2f, 0x92];

        assert_eq!(expected, push_register(9));
    }

    #[test]
    fn pop_r0() {
        let expected: Vec<u8> = vec![0x60, 0xf2, 0x7f, 0x04];

        assert_eq!(expected, pop_register(0));
    }
    #[test]
    fn pop_r9() {
        let expected: Vec<u8> = vec![0x69, 0xf2, 0x7f, 0x04];

        assert_eq!(expected, pop_register(9));
    }
}
