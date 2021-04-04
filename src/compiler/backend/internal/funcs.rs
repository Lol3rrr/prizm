use super::immediate_val;

/// This overwrites R2 and R3
///
/// Pushes the current PR onto the stack
/// jumps to the address in a sub-routine
/// and then restores the PR again from the stack
pub fn call(address: u32) -> Vec<u8> {
    let mut result = Vec::new();

    // Save previous PR, push value onto stack
    result.push(0x4f);
    result.push(0x22);

    // Store the Target address into r2
    result.append(&mut immediate_val::store_32bit_r(2, address));

    // JSR - Jump-Sub-Routine in r2
    result.push(0x42);
    result.push(0x0b);

    // Noop
    result.push(0x00);
    result.push(0x09);

    // Restore previous PR from stack
    result.push(0x4f);
    result.push(0x26);

    result
}

pub fn ret() -> Vec<u8> {
    let mut result = Vec::new();

    // RTS - Return-from-subroutine
    result.push(0x00);
    result.push(0x0b);

    // Noop
    result.push(0x00);
    result.push(0x09);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling() {
        let expected: Vec<u8> = vec![
            3, 42, 47, 6, 226, 18, 66, 24, 114, 52, 66, 24, 114, 86, 66, 24, 114, 120, 66, 11, 0,
            9, 99, 246, 67, 42,
        ];

        assert_eq!(expected, call(0x12345678));
    }

    #[test]
    fn returning() {
        let expected: Vec<u8> = vec![0x00, 0x0b, 0x00, 0x09];

        assert_eq!(expected, ret());
    }
}
