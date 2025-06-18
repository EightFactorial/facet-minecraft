//! Benchmarks for basic type serialization.

use divan::{bench, black_box};
use facet_minecraft::{McSerializer, Serializer, SerializerExt};

fn main() { divan::main() }

#[bench(args = [u8::MIN, u8::MIN + u8::MAX / 2, u8::MAX])]
fn serialize_u8(val: u8) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(1));
    ser.serialize_u8(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [u16::MIN, u16::MIN + u16::MAX / 2, u16::MAX])]
fn serialize_u16(val: u16) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(2));
    ser.serialize_u16(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [u32::MIN, u32::MIN + u32::MAX / 2, u32::MAX])]
fn serialize_u32(val: u32) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(4));
    ser.serialize_u32(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [u64::MIN, u64::MIN + u64::MAX / 2, u64::MAX])]
fn serialize_u64(val: u64) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(8));
    ser.serialize_u64(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [u128::MIN, u128::MIN + u128::MAX / 2, u128::MAX])]
fn serialize_u128(val: u128) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(16));
    ser.serialize_u128(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [usize::MIN, usize::MIN + usize::MAX / 2, usize::MAX])]
fn serialize_usize(val: usize) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(8));
    ser.serialize_usize(black_box(val)).unwrap();
    ser.0
}

// -------------------------------------------------------------------------------------------------

#[bench(args = [u16::MIN, u16::MIN + u16::MAX / 2, u16::MAX])]
fn serialize_var_u16(val: u16) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(3));
    ser.serialize_var_u16(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [u32::MIN, u32::MIN + u32::MAX / 2, u32::MAX])]
fn serialize_var_u32(val: u32) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(5));
    ser.serialize_var_u32(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [u64::MIN, u64::MIN + u64::MAX / 2, u64::MAX])]
fn serialize_var_u64(val: u64) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(10));
    ser.serialize_var_u64(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [u128::MIN, u128::MIN + u128::MAX / 2, u128::MAX])]
fn serialize_var_u128(val: u128) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(19));
    ser.serialize_var_u128(black_box(val)).unwrap();
    ser.0
}

#[bench(args = [usize::MIN, usize::MIN + usize::MAX / 2, usize::MAX])]
fn serialize_var_usize(val: usize) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(10));
    ser.serialize_var_usize(black_box(val)).unwrap();
    ser.0
}

// -------------------------------------------------------------------------------------------------

#[bench(args = [true, false])]
fn serialize_bool(val: bool) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(1));
    ser.serialize_bool(black_box(val)).unwrap();
    ser.0
}

#[bench(args = ["", "abc", "123", "Hello, world!", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor..."])]
fn serialize_str(val: &str) -> Vec<u8> {
    let mut ser = McSerializer(Vec::with_capacity(val.len() + 2));
    ser.serialize_str(black_box(val)).unwrap();
    ser.0
}
