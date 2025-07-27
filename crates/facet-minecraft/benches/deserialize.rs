//! Benchmarks for basic type deserialization.

use divan::{bench, black_box};
use facet_minecraft::{Deserializer, McDeserializer};

fn main() { divan::main() }

#[bench(args = [
    [0u8].as_slice(),
    [127u8].as_slice(),
    [255u8].as_slice()
])]
fn deserialize_u8(input: &[u8]) {
    let (de, rem) = McDeserializer.deserialize_u8(black_box(input)).unwrap();
    debug_assert!(matches!(de, 0 | 127 | 255), "Expected u8 values 0, 127, or 255, got: {de}");
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

#[bench(args = [
    [0u8, 0u8].as_slice(),
    [255u8, 0u8].as_slice(),
    [255u8, 255u8].as_slice()
])]
fn deserialize_u16(input: &[u8]) {
    let (de, rem) = McDeserializer.deserialize_u16(black_box(input)).unwrap();
    debug_assert!(
        matches!(de, 0 | 255 | 65535),
        "Expected u16 values 0, 32767, or 65534, got: {de}"
    );
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

#[bench(args = [
    [0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8].as_slice()
])]
fn deserialize_u32(input: &[u8]) {
    let (de, rem) = McDeserializer.deserialize_u32(black_box(input)).unwrap();
    debug_assert!(
        matches!(de, 0 | 65535 | 4294967295),
        "Expected u32 values 0, 65535, or 4294967295, got: {de}"
    );
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

#[bench(args = [
    [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8].as_slice()
])]
fn deserialize_u64(input: &[u8]) {
    let (de, rem) = McDeserializer.deserialize_u64(black_box(input)).unwrap();
    debug_assert!(
        matches!(de, 0 | 4294967295 | 18446744073709551615),
        "Expected u64 values 0, 4294967295, or 18446744073709551615, got: {de}"
    );
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

#[bench(args = [
    [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8].as_slice()
])]
fn deserialize_u128(input: &[u8]) {
    let (de, rem) = McDeserializer.deserialize_u128(black_box(input)).unwrap();
    debug_assert!(
        matches!(de, 0 | 18446744073709551615 | 340282366920938463463374607431768211455),
        "Expected u128 values 0, 18446744073709551615, or 340282366920938463463374607431768211455, got: {de}"
    );
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

#[bench(args = [
    [0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8].as_slice()
])]
#[cfg(target_pointer_width = "32")]
fn deserialize_usize(input: &[u8]) {
    let (de, rem) = McDeserializer.deserialize_usize(black_box(input)).unwrap();
    debug_assert!(
        matches!(de, 0 | 65535 | 4294967295),
        "Expected u32 values 0, 65535, or 4294967295, got: {de}"
    );
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

#[bench(args = [
    [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8].as_slice(),
    [255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8].as_slice()
])]
#[cfg(target_pointer_width = "64")]
fn deserialize_usize(input: &[u8]) {
    let (de, rem) = McDeserializer.deserialize_usize(black_box(input)).unwrap();
    debug_assert!(
        matches!(de, 0 | 4294967295 | 18446744073709551615),
        "Expected u64 values 0, 4294967295, or 18446744073709551615, got: {de}"
    );
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

// -------------------------------------------------------------------------------------------------

// #[bench(args = [])]
// fn deserialize_var_u16(input: &[u8]) {}

// #[bench(args = [])]
// fn deserialize_var_u32(input: &[u8]) {}

// #[bench(args = [])]
// fn deserialize_var_u64(input: &[u8]) {}

// #[bench(args = [])]
// fn deserialize_var_u128(input: &[u8]) {}

// #[bench(args = [])]
// fn deserialize_var_usize(input: &[u8]) {}

// -------------------------------------------------------------------------------------------------

#[bench(args = [
    [0u8].as_slice(),
    [1u8].as_slice()
])]
fn deserialize_bool(input: &[u8]) {
    let (_de, rem) = McDeserializer.deserialize_bool(black_box(input)).unwrap();
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}

#[bench(args = [
    [0u8].as_slice(),
    [3u8, b'a', b'b', b'c'].as_slice(),
    [3u8, b'1', b'2', b'3'].as_slice(),
    [13u8, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!'].as_slice()
])]
fn deserialize_str(input: &[u8]) {
    let (_de, rem) = McDeserializer.deserialize_str(black_box(input)).unwrap();
    debug_assert!(rem.is_empty(), "Expected no remaining bytes, got: {rem:?}");
}
