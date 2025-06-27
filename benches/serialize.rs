//! Benchmarks for basic type serialization.

use divan::{bench, black_box};
use facet_minecraft::{McSerializer, Serializer, SerializerExt, SliceCursor};

fn main() { divan::main() }

const SLICE_SIZE: usize = 32;

#[bench(args = [u8::MIN, u8::MIN + u8::MAX / 2, u8::MAX])]
fn serialize_u8(val: u8) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_u8(black_box(val)).unwrap();
}

#[bench(args = [u16::MIN, u16::MIN + u16::MAX / 2, u16::MAX])]
fn serialize_u16(val: u16) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_u16(black_box(val)).unwrap();
}

#[bench(args = [u32::MIN, u32::MIN + u32::MAX / 2, u32::MAX])]
fn serialize_u32(val: u32) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_u32(black_box(val)).unwrap();
}

#[bench(args = [u64::MIN, u64::MIN + u64::MAX / 2, u64::MAX])]
fn serialize_u64(val: u64) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_u64(black_box(val)).unwrap();
}

#[bench(args = [u128::MIN, u128::MIN + u128::MAX / 2, u128::MAX])]
fn serialize_u128(val: u128) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_u128(black_box(val)).unwrap();
}

#[bench(args = [usize::MIN, usize::MIN + usize::MAX / 2, usize::MAX])]
fn serialize_usize(val: usize) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_usize(black_box(val)).unwrap();
}

// -------------------------------------------------------------------------------------------------

#[bench(args = [u16::MIN, u16::MIN + u16::MAX / 2, u16::MAX])]
fn serialize_var_u16(val: u16) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_var_u16(black_box(val)).unwrap();
}

#[bench(args = [u32::MIN, u32::MIN + u32::MAX / 2, u32::MAX])]
fn serialize_var_u32(val: u32) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_var_u32(black_box(val)).unwrap();
}

#[bench(args = [u64::MIN, u64::MIN + u64::MAX / 2, u64::MAX])]
fn serialize_var_u64(val: u64) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_var_u64(black_box(val)).unwrap();
}

#[bench(args = [u128::MIN, u128::MIN + u128::MAX / 2, u128::MAX])]
fn serialize_var_u128(val: u128) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_var_u128(black_box(val)).unwrap();
}

#[bench(args = [usize::MIN, usize::MIN + usize::MAX / 2, usize::MAX])]
fn serialize_var_usize(val: usize) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_var_usize(black_box(val)).unwrap();
}

// -------------------------------------------------------------------------------------------------

#[bench(args = [true, false])]
fn serialize_bool(val: bool) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_bool(black_box(val)).unwrap();
}

#[bench(args = ["", "abc", "123", "Hello, world!"])]
fn serialize_str(val: &str) {
    let mut slice = [0u8; SLICE_SIZE];
    let mut ser = McSerializer(SliceCursor::new(&mut slice));
    ser.serialize_str(black_box(val)).unwrap();
}
