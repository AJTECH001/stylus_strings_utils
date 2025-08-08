#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::U256;
use stylus_sdk::prelude::*;
use strings_utils::{to_string, to_hex_string, to_hex_string_fixed};

sol_storage! {
    #[entrypoint]
    struct StringsDemo {}
}

#[external]
impl StringsDemo {
    pub fn to_decimal(&self, value: U256) -> String {
        to_string(value)
    }

    pub fn to_hex(&self, value: U256) -> String {
        to_hex_string(value)
    }

    pub fn to_hex_fixed(&self, value: U256, len: u32) -> String {
        to_hex_string_fixed(value, len as usize)
    }
}