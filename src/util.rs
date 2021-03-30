pub fn write_string(target: &mut [u8], content: &str) {
    let content_length = content.len();
    if content_length > target.len() {
        return;
    }

    target[0..content_length].copy_from_slice(content.as_bytes());
}

// TODO
pub fn checksum(_data: &[u8]) -> u32 {
    0
}
