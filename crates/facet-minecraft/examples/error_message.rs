//! TODO
#![allow(dead_code)]

use facet_macros::Facet;
use facet_minecraft::McDeserializer;

extern crate facet_core as facet;

fn main() {
    const INVALID_UTF8: &[u8] = &[8, b'a', b'a', b'a', b'a', b'a', b'a', 0xc3, 0x28, 1, 1, 0];

    let failed = McDeserializer::deserialize::<ExampleProperty>(INVALID_UTF8);
    failed.unwrap_err().eprint();

    const INVALID_BOOL: &[u8] = &[2, b'a', b'a', 0, 2];

    let failed = McDeserializer::deserialize::<ExampleProperty>(INVALID_BOOL);
    failed.unwrap_err().eprint();

    const INVALID_VARIANT: &[u8] = &[2, b'a', b'a', 128, 2, 0, 0, 0, 0];

    let failed = McDeserializer::deserialize::<ExampleProperty>(INVALID_VARIANT);
    failed.unwrap_err().eprint();

    const END_OF_STREAM_STR: &[u8] = &[4, b'a'];

    let failed = McDeserializer::deserialize::<ExampleProperty>(END_OF_STREAM_STR);
    failed.unwrap_err().eprint();

    const END_OF_STREAM_VAR_SHORT: &[u8] = &[1, b'a', 2, 128, 128];

    let failed = McDeserializer::deserialize::<ExampleProperty>(END_OF_STREAM_VAR_SHORT);
    failed.unwrap_err().eprint();

    const END_OF_STREAM_LONG: &[u8] = &[0, 4, 0, 0, 0];

    let failed = McDeserializer::deserialize::<ExampleProperty>(END_OF_STREAM_LONG);
    failed.unwrap_err().eprint();
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Facet)]
struct ExampleProperty {
    pub name: String,
    pub value: ExampleValue,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Facet)]
enum ExampleValue {
    Bool(bool) = 0,
    Byte(u8),
    Short(#[facet(var)] u16),
    Int(u32),
    Long(u64),
}
