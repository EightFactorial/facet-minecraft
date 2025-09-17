//! Tests for deserialization error messages.

use facet_minecraft::{Standard, deserialize::Deserializer};

#[test]
fn test_bool_message() {
    static INPUT: &[u8] = &[0x02];
    let mut err = Standard.deserialize_bool(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_u8_message() {
    static INPUT: &[u8] = &[];
    let mut err = Standard.deserialize_u8(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_u16_message() {
    static INPUT: &[u8] = &[0xFF];
    let mut err = Standard.deserialize_u16(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_u32_message() {
    static INPUT: &[u8] = &[0xFF, 0xFF, 0xFF];
    let mut err = Standard.deserialize_u32(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_u64_message() {
    static INPUT: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let mut err = Standard.deserialize_u64(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_u128_message() {
    static INPUT: &[u8] =
        &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let mut err = Standard.deserialize_u128(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_f32_message() {
    static INPUT: &[u8] = &[0xFF, 0xFF, 0xFF];
    let mut err = Standard.deserialize_f32(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_f64_message() {
    static INPUT: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let mut err = Standard.deserialize_f64(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_bytes_message() {
    static INPUT: &[u8] = &[11, b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r'];
    let mut err = Standard.deserialize_bytes(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}

#[test]
fn test_string_message() {
    static INPUT: &[u8] = &[11, b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', 0xFF, b'l', b'd'];
    let mut err = Standard.deserialize_str(INPUT).unwrap_err();
    err = err.with_location(core::panic::Location::caller());
    println!("{}", err.as_report());
}
