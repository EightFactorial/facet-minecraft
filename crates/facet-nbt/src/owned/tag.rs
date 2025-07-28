use alloc::vec::Vec;

use crate::{mutf8::Mutf8String, owned::NbtCompound};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, facet_macros::Facet)]
pub enum NbtTag {
    /// A signed 8-bit integer.
    Byte(i8) = NbtTag::BYTE,
    /// A signed 16-bit integer.
    Short(i16) = NbtTag::SHORT,
    /// A signed 32-bit integer.
    Int(i32) = NbtTag::INT,
    /// A signed 64-bit integer.
    Long(i64) = NbtTag::LONG,
    /// A 32-bit floating point number.
    Float(f32) = NbtTag::FLOAT,
    /// A 64-bit floating point number.
    Double(f64) = NbtTag::DOUBLE,
    /// An array of signed 8-bit integers.
    ByteArray(Vec<u8>) = NbtTag::BYTE_ARRAY,
    /// A [`Mutf8String`].
    String(Mutf8String) = NbtTag::STRING,
    /// An [`NbtListTag`].
    List(NbtListTag) = NbtTag::LIST,
    /// An [`NbtCompound`].
    Compound(NbtCompound) = NbtTag::COMPOUND,
    /// An array of signed 32-bit integers.
    IntArray(Vec<i32>) = NbtTag::INT_ARRAY,
    /// An array of signed 64-bit integers.
    LongArray(Vec<i64>) = NbtTag::LONG_ARRAY,
}

#[rustfmt::skip]
impl NbtTag {
    /// The end of a [`NbtTag::Compound`] or [`NbtTag::List`].
    pub const END: u8 = 0;
    /// The tag of a [`NbtTag::Byte`].
    pub const BYTE: u8 = 1;
    /// The tag of a [`NbtTag::Short`].
    pub const SHORT: u8 = 2;
    /// The tag of a [`NbtTag::Int`].
    pub const INT: u8 = 3;
    /// The tag of a [`NbtTag::Long`].
    pub const LONG: u8 = 4;
    /// The tag of a [`NbtTag::Float`].
    pub const FLOAT: u8 = 5;
    /// The tag of a [`NbtTag::Double`].
    pub const DOUBLE: u8 = 6;
    /// The tag of a [`NbtTag::ByteArray`].
    pub const BYTE_ARRAY: u8 = 7;
    /// The tag of a [`NbtTag::String`].
    pub const STRING: u8 = 8;
    /// The tag of a [`NbtTag::List`].
    pub const LIST: u8 = 9;
    /// The tag of a [`NbtTag::Compound`].
    pub const COMPOUND: u8 = 10;
    /// The tag of a [`NbtTag::IntArray`].
    pub const INT_ARRAY: u8 = 11;
    /// The tag of a [`NbtTag::LongArray`].
    pub const LONG_ARRAY: u8 = 12;
}

impl From<i8> for NbtTag {
    fn from(value: i8) -> Self { NbtTag::Byte(value) }
}
impl From<i16> for NbtTag {
    fn from(value: i16) -> Self { NbtTag::Short(value) }
}
impl From<i32> for NbtTag {
    fn from(value: i32) -> Self { NbtTag::Int(value) }
}
impl From<i64> for NbtTag {
    fn from(value: i64) -> Self { NbtTag::Long(value) }
}
impl From<f32> for NbtTag {
    fn from(value: f32) -> Self { NbtTag::Float(value) }
}
impl From<f64> for NbtTag {
    fn from(value: f64) -> Self { NbtTag::Double(value) }
}
impl From<Vec<u8>> for NbtTag {
    fn from(value: Vec<u8>) -> Self { NbtTag::ByteArray(value) }
}
impl From<Mutf8String> for NbtTag {
    fn from(value: Mutf8String) -> Self { NbtTag::String(value) }
}
impl From<NbtListTag> for NbtTag {
    fn from(value: NbtListTag) -> Self { NbtTag::List(value) }
}
impl From<NbtCompound> for NbtTag {
    fn from(value: NbtCompound) -> Self { NbtTag::Compound(value) }
}
impl From<Vec<i32>> for NbtTag {
    fn from(value: Vec<i32>) -> Self { NbtTag::IntArray(value) }
}
impl From<Vec<i64>> for NbtTag {
    fn from(value: Vec<i64>) -> Self { NbtTag::LongArray(value) }
}

// -------------------------------------------------------------------------------------------------

/// A list of NBT tag values.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, facet_macros::Facet)]
pub enum NbtListTag {
    /// An empty, untyped list.
    Empty = NbtTag::END,
    /// A list of signed 8-bit integers.
    Byte(Vec<u8>) = NbtTag::BYTE,
    /// A list of signed 16-bit integers.
    Short(Vec<i16>) = NbtTag::SHORT,
    /// A list of signed 32-bit integers.
    Int(Vec<i32>) = NbtTag::INT,
    /// A list of signed 64-bit integers.
    Long(Vec<i64>) = NbtTag::LONG,
    /// A list of 32-bit floating point numbers.
    Float(Vec<f32>) = NbtTag::FLOAT,
    /// A list of 64-bit floating point numbers.
    Double(Vec<f64>) = NbtTag::DOUBLE,
    /// A list of arrays of signed 8-bit integers.
    ByteArray(Vec<Vec<u8>>) = NbtTag::BYTE_ARRAY,
    /// A list of [`Mutf8String`]s.
    String(Vec<Mutf8String>) = NbtTag::STRING,
    /// A list of [`NbtListTag`]s.
    List(Vec<NbtListTag>) = NbtTag::LIST,
    /// A list of [`NbtCompound`]s.
    Compound(Vec<NbtCompound>) = NbtTag::COMPOUND,
    /// A list of arrays of signed 32-bit integers.
    IntArray(Vec<Vec<i32>>) = NbtTag::INT_ARRAY,
    /// A list of arrays of signed 64-bit integers.
    LongArray(Vec<Vec<i64>>) = NbtTag::LONG_ARRAY,
}

impl From<Vec<u8>> for NbtListTag {
    fn from(value: Vec<u8>) -> Self { NbtListTag::Byte(value) }
}
impl From<Vec<i16>> for NbtListTag {
    fn from(value: Vec<i16>) -> Self { NbtListTag::Short(value) }
}
impl From<Vec<i32>> for NbtListTag {
    fn from(value: Vec<i32>) -> Self { NbtListTag::Int(value) }
}
impl From<Vec<i64>> for NbtListTag {
    fn from(value: Vec<i64>) -> Self { NbtListTag::Long(value) }
}
impl From<Vec<f32>> for NbtListTag {
    fn from(value: Vec<f32>) -> Self { NbtListTag::Float(value) }
}
impl From<Vec<f64>> for NbtListTag {
    fn from(value: Vec<f64>) -> Self { NbtListTag::Double(value) }
}
impl From<Vec<Vec<u8>>> for NbtListTag {
    fn from(value: Vec<Vec<u8>>) -> Self { NbtListTag::ByteArray(value) }
}
impl From<Vec<Mutf8String>> for NbtListTag {
    fn from(value: Vec<Mutf8String>) -> Self { NbtListTag::String(value) }
}
impl From<Vec<NbtListTag>> for NbtListTag {
    fn from(value: Vec<NbtListTag>) -> Self { NbtListTag::List(value) }
}
impl From<Vec<NbtCompound>> for NbtListTag {
    fn from(value: Vec<NbtCompound>) -> Self { NbtListTag::Compound(value) }
}
impl From<Vec<Vec<i32>>> for NbtListTag {
    fn from(value: Vec<Vec<i32>>) -> Self { NbtListTag::IntArray(value) }
}
impl From<Vec<Vec<i64>>> for NbtListTag {
    fn from(value: Vec<Vec<i64>>) -> Self { NbtListTag::LongArray(value) }
}
