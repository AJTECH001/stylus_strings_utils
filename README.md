# Strings Utils for Stylus

A Rust library for Stylus (Arbitrum) that replicates OpenZeppelin's `Strings.sol` functionality, providing utility functions to convert `U256` values to decimal and hexadecimal strings.

## Features

- `to_string(value: U256) -> String`: Converts a `U256` to a decimal string.
- `to_hex_string(value: U256) -> String`: Converts a `U256` to a hexadecimal string with `0x` prefix.
- `to_hex_string_fixed(value: U256, len: usize) -> String`: Converts a `U256` to a fixed-length hexadecimal string with `0x` prefix.


# stylus_strings_utils
