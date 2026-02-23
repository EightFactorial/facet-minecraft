//! TODO
#![no_std]

extern crate alloc;

use alloc::string::String;
use core::fmt::Display;

use facet::{Facet, Partial, Peek};
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, error::DeserializeValueError},
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};

#[derive(Debug, Facet)]
struct Var<T>(#[facet(mc::variable)] pub T);

impl<T: Display> Display for Var<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl<T: PartialEq> PartialEq for Var<T> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

#[cfg(feature = "tracing")]
fn trace() -> tracing::subscriber::DefaultGuard {
    use tracing_subscriber::prelude::*;
    let subscriber =
        tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().with_test_writer());
    tracing::subscriber::set_default(subscriber)
}

macro_rules! test {
    ($ty:ty: $val:expr => $data:expr) => {
        let val = $val;
        let expected = $data;
        let serialized = facet_minecraft::to_vec(&val).unwrap();
        assert_eq!(
            serialized, expected,
            "From {val:?} expected {expected:02x?}, got {serialized:02x?}"
        );
        assert_eq!(facet_minecraft::from_slice::<$ty>(&serialized).unwrap(), val);
    };
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Facet)]
struct UnitStruct;

#[derive(Debug, PartialEq, Facet)]
struct TupleStruct(bool, Option<u8>, u8);

#[derive(Debug, PartialEq, Facet)]
struct Struct {
    a: bool,
    b: u32,
    #[facet(mc::variable)]
    c: usize,
}
#[derive(Debug, PartialEq, Facet)]
struct VariableOption {
    a: String,
    #[facet(mc::variable)]
    b: Option<u32>,
    c: bool,
}

#[derive(Debug, PartialEq, Facet)]
struct NestedCustomSerialize {
    a: Option<CustomSerialize>,
}

#[derive(Debug, PartialEq, Facet)]
struct CustomSerialize {
    a: u8,
    #[facet(mc::serialize = CustomSerialize::SERIALIZE_B)]
    #[facet(mc::deserialize = CustomSerialize::DESERIALIZE_B)]
    b: u64,
    c: bool,
}

impl CustomSerialize {
    const DESERIALIZE_B: DeserializeFn = DeserializeFn::new(Self::deserialize, Self::deserialize);
    const SERIALIZE_B: SerializeFn = SerializeFn::new(Self::serialize);

    fn serialize<'mem, 'facet>(
        value: Peek<'mem, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        let bytes = value.get::<u64>()?.to_le_bytes();
        if writer.write_data(&[bytes[0]]) { Ok(()) } else { Err(SerializeIterError::new()) }
    }

    fn deserialize<'facet, const BORROW: bool>(
        value: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let byte = cursor.take(1)?[0];
        replace_with::replace_with_or_abort(value, |value| {
            value.set(u64::from_le_bytes([byte, 0, 0, 0, 0, 0, 0, 0])).unwrap()
        });
        Ok(())
    }
}

#[test]
fn r#struct() {
    #[cfg(feature = "tracing")]
    let _guard = trace();

    test!(UnitStruct: UnitStruct => []);
    test!(TupleStruct: TupleStruct(false, None, 0xff) => [0x00, 0x00, 0xff]);
    test!(TupleStruct: TupleStruct(false, Some(0x00), 0xff) => [0x00, 0x01, 0x00, 0xff]);
    test!(TupleStruct: TupleStruct(true, Some(0x2a), 0xff) => [0x01, 0x01, 0x2a, 0xff]);
    test!(Struct: Struct { a: false, b: 0x00, c: 0x00 } => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    test!(Struct: Struct { a: true, b: 0x12345678, c: 0x00 } => [0x01, 0x12, 0x34, 0x56, 0x78, 0x00]);
    test!(Struct: Struct { a: true, b: 0x12345678, c: 0x2a } => [0x01, 0x12, 0x34, 0x56, 0x78, 0x2a]);
    test!(Struct: Struct { a: true, b: 0x12345678, c: 0xff } => [0x01, 0x12, 0x34, 0x56, 0x78, 0xff, 0x01]);
    test!(VariableOption: VariableOption { a: String::new(), b: None, c: false } => [0x00, 0x00, 0x00]);
    test!(VariableOption: VariableOption { a: String::new(), b: Some(0x00), c: false } => [0x00, 0x01, 0x00, 0x00]);
    test!(VariableOption: VariableOption { a: String::from("aa"), b: Some(0xff), c: false } => [0x02, b'a', b'a', 0x01, 0xff, 0x01, 0x00]);
    test!(CustomSerialize: CustomSerialize { a: 0x00, b: 0x34, c: true } => [0x00, 0x34, 0x01]);
    test!(CustomSerialize: CustomSerialize { a: 0x2a, b: 0x34, c: false } => [0x2a, 0x34, 0x00]);
    test!(NestedCustomSerialize: NestedCustomSerialize { a: None } => [0x00]);
    test!(NestedCustomSerialize: NestedCustomSerialize { a: Some(CustomSerialize { a: 0x2a, b: 0x34, c: false }) } => [0x01, 0x2a, 0x34, 0x00]);
}

// -------------------------------------------------------------------------------------------------

#[test]
fn r#enum() {}
