use super::internal;

pub fn generate(call_id: u16) -> Vec<u8> {
    let mut result = Vec::new();
    result.append(&mut internal::store::store_u16(0, call_id));

    // Store the Jump address for systemcalls into r2
    result.extend_from_slice(&[0xe2, 0x80, 0x42, 0x18, 0x72, 0x02, 0x42, 0x28, 0x72, 0x70]);

    // Jump
    result.push(0x42);
    result.push(0x2b);

    // Noop after jump
    result.push(0x00);
    result.push(0x09);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_get_key() {
        let call_id = 0x0eab;

        let expected = vec![
            0xe0, 0x0e, 0x40, 0x18, 0x70, 0xab, 0xe2, 0x80, 0x42, 0x18, 0x72, 0x02, 0x42, 0x28,
            0x72, 0x70, 0x42, 0x2b, 0x00, 0x09,
        ];

        assert_eq!(expected, generate(call_id));
    }
}
