pub fn store_32bit_r(reg: u8, value: u32) -> Vec<u8> {
    let mut result = Vec::new();

    let im_mov_first = 0xe0 | (reg & 0x0f);
    let im_add_first = 0x70 | (reg & 0x0f);
    let im_shift_first = 0x40 | (reg & 0x0f);

    let target_bytes = value.to_be_bytes();
    // Storing the First byte
    result.push(im_mov_first);
    result.push(target_bytes[0]);
    // Shift register one byte left and add the second byte
    result.push(im_shift_first);
    result.push(0x18);
    result.push(im_add_first);
    result.push(target_bytes[1]);
    // Shift register one byte left and add the third byte
    result.push(im_shift_first);
    result.push(0x18);
    result.push(im_add_first);
    result.push(target_bytes[2]);
    // Shift register one byte left and add the third byte
    result.push(im_shift_first);
    result.push(0x18);
    result.push(im_add_first);
    result.push(target_bytes[3]);

    result
}
