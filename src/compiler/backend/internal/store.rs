pub fn store_u16(register: u8, value: u16) -> Vec<u8> {
    const RAW_MASK: u16 = 0x7f;
    const F_MASK: u16 = 0x0000 | (RAW_MASK << 9);
    const S_MASK: u16 = 0x0000 | (RAW_MASK << 2);
    const T_MASK: u16 = 0x0000 | (RAW_MASK >> 5);

    vec![
        0xe0 | (register & 0x0f), // Stores the first 7 bit from value in register
        ((value & F_MASK) >> 9) as u8,
        0x40 | (register & 0x0f), // Shifts 8 bit left (<<)
        0x18,
        0x40 | (register & 0x0f), // Shifts 1 bit right (>>)
        0x01,
        0x70 | (register & 0x0f), // Adds the next 7 bit to the register
        ((value & S_MASK) >> 2) as u8,
        0x40 | (register & 0x0f), // Shift one bit left (<<)
        0x00,
        0x40 | (register & 0x0f), // Shift one bit left (<<)
        0x00,
        0x70 | (register & 0x0f), // Add the last 2 bit to the register
        (value & T_MASK) as u8,
    ]
}

pub fn store_u32(register: u8, value: u32) -> Vec<u8> {
    const RAW_MASK: u32 = 0x7f as u32;
    const MASKS: [u32; 5] = [
        RAW_MASK << 25,
        RAW_MASK << 18,
        RAW_MASK << 11,
        RAW_MASK << 4,
        RAW_MASK >> 3,
    ];

    let mut result: Vec<u8> = Vec::new();
    for (index, mask) in MASKS.iter().enumerate() {
        let shift = (32 as usize).saturating_sub((index + 1) * 7);

        result.push(0x70 | (register & 0x0f));
        result.push(((value & mask) >> shift) as u8);
        if shift > 0 {
            result.push(0x40 | (register & 0x0f)); // Shift 2 bit left
            result.push(0x08);
            result.push(0x40 | (register & 0x0f)); // Shift 2 bit left
            result.push(0x08);
            result.push(0x40 | (register & 0x0f)); // Shift 2 bit left
            result.push(0x08);
            result.push(0x40 | (register & 0x0f)); // Shift 1 bit left
            result.push(0x00);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u16_0x31f0() {
        let expected: Vec<u8> = vec![224, 24, 64, 24, 64, 1, 112, 124, 64, 0, 64, 0, 112, 0];

        assert_eq!(expected, store_u16(0, 0x31f0));
    }

    #[test]
    fn store_u32_0x31f00101() {
        let expected: Vec<u8> = vec![];

        assert_eq!(expected, store_u32(0, 0x31f00101));
    }
}
