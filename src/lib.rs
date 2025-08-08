use alloy_primitives::U256;

pub fn to_string(value: U256) -> String {
    if value.is_zero() {
        return "0".to_string();
    }

    let mut buf = Vec::new();
    let mut v = value;
    while v > U256::zero() {
        let digit = (v % U256::from(10)).as_u64() as u8;
        buf.push(b'0' + digit);
        v /= U256::from(10);
    }
    buf.reverse();
    String::from_utf8(buf).expect("Invalid UTF-8 sequence")
}

pub fn to_hex_string(value: U256) -> String {
    if value.is_zero() {
        return "0x0".to_string();
    }

    let bytes = value.to_be_bytes::<32>();
    let mut trimmed = bytes.iter().skip_while(|&&b| b == 0).collect::<Vec<_>>();
    if trimmed.is_empty() {
        trimmed.push(&0); // Ensure at least one zero for non-zero input edge cases
    }

    let mut result = String::with_capacity(2 + trimmed.len() * 2);
    result.push_str("0x");
    for &b in trimmed {
        result.push_str(&format!("{:02x}", b));
    }
    result
}

pub fn to_hex_string_fixed(value: U256, len: usize) -> String {
    let bytes = value.to_be_bytes::<32>();
    let mut result = String::with_capacity(2 + len * 2);
    result.push_str("0x");

    // Start from the end of the bytes array, taking the least significant len*2 nibbles
    let start = bytes.len().saturating_sub(len);
    for i in start..bytes.len() {
        result.push_str(&format!("{:02x}", bytes[i]));
    }
    // Pad with leading zeros if needed
    for _ in 0..(len.saturating_sub(bytes.len() - start)) {
        result.push_str("00");
    }
    result
}