#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;
    use hex::encode;

    #[test]
    fn test_to_string() {
        assert_eq!(to_string(U256::from(0)), "0");
        assert_eq!(to_string(U256::from(123)), "123");
        assert_eq!(to_string(U256::from(1000)), "1000");
        assert_eq!(
            to_string(U256::MAX),
            "115792089237316195423570985008687907853269984665640564039457584007913129639935"
        );
    }

    #[test]
    fn test_to_hex_string() {
        assert_eq!(to_hex_string(U256::from(0)), "0x0");
        assert_eq!(to_hex_string(U256::from(255)), "0xff");
        assert_eq!(to_hex_string(U256::from(1234)), "0x4d2");
        assert_eq!(
            to_hex_string(U256::MAX),
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
    }

    #[test]
    fn test_to_hex_string_fixed() {
        assert_eq!(to_hex_string_fixed(U256::from(0), 1), "0x00");
        assert_eq!(to_hex_string_fixed(U256::from(255), 2), "0x00ff");
        assert_eq!(to_hex_string_fixed(U256::from(1234), 4), "0x000004d2");
        assert_eq!(
            to_hex_string_fixed(U256::MAX, 32),
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(to_hex_string_fixed(U256::from(255), 1), "0xff");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(to_string(U256::from(1)), "1");
        assert_eq!(to_hex_string(U256::from(1)), "0x1");
        assert_eq!(to_hex_string_fixed(U256::from(1), 2), "0x0001");
        assert_eq!(to_hex_string_fixed(U256::from(0), 3), "0x000000");
    }
}