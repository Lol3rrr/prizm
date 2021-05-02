/// Extends an 8bit value to a 32 signed value
///
/// ```
/// # use emulator::general;
/// #
/// let previous = 0x8f;
/// let result = general::sign_extend_u8(previous);
/// assert_eq!(0xFFFFFF8f, result);
/// ```
pub fn sign_extend_u8(raw_value: u8) -> u32 {
    if (raw_value & 0x80) == 0 {
        0x000000FF & (raw_value as u32)
    } else {
        0xFFFFFF00 | (raw_value as u32)
    }
}

/// Extends a 12bit value to a 32 signed value
///
/// ```
/// # use emulator::general;
/// #
/// let previous = 0x810;
/// let result = general::sign_extend_u12(previous);
/// assert_eq!(0xFFFFF810, result);
/// ```
pub fn sign_extend_u12(raw_value: u16) -> u32 {
    if (raw_value & 0x800) == 0 {
        0x00000FFF & (raw_value as u32)
    } else {
        0xFFFFF000 | (raw_value as u32)
    }
}

/// Extends as 16bit value to a 32 signed value
///
/// ```
/// # use emulator::general;
/// #
/// let previous = 0x8010;
/// let result = general::sign_extend_u16(previous);
/// assert_eq!(0xFFFF8010, result);
/// ```
pub fn sign_extend_u16(raw_value: u16) -> u32 {
    if (raw_value & 0x8000) == 0 {
        0x0000FFFF & (raw_value as u32)
    } else {
        0xFFFF0000 | (raw_value as u32)
    }
}
