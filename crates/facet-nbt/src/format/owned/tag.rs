use alloc::vec::Vec;

use super::NbtCompound;
use crate::{format::raw::RawTagType, mutf8::Mutf8String};

#[repr(u8)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    /// A signed 8-bit integer.
    Byte(i8) = RawTagType::BYTE,
    /// A signed 16-bit integer.
    Short(i16) = RawTagType::SHORT,
    /// A signed 32-bit integer.
    Int(i32) = RawTagType::INT,
    /// A signed 64-bit integer.
    Long(i64) = RawTagType::LONG,
    /// A 32-bit floating point number.
    Float(f32) = RawTagType::FLOAT,
    /// A 64-bit floating point number.
    Double(f64) = RawTagType::DOUBLE,
    /// An array of signed 8-bit integers.
    ByteArray(Vec<i8>) = RawTagType::BYTE_ARRAY,
    /// A [`Mutf8String`].
    String(Mutf8String) = RawTagType::STRING,
    /// An [`NbtListTag`].
    List(NbtListTag) = RawTagType::LIST,
    /// An [`NbtCompound`].
    Compound(NbtCompound) = RawTagType::COMPOUND,
    /// An array of signed 32-bit integers.
    IntArray(Vec<i32>) = RawTagType::INT_ARRAY,
    /// An array of signed 64-bit integers.
    LongArray(Vec<i64>) = RawTagType::LONG_ARRAY,
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
impl From<Vec<i8>> for NbtTag {
    fn from(value: Vec<i8>) -> Self { NbtTag::ByteArray(value) }
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

#[repr(u8)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, PartialEq)]
pub enum NbtListTag {
    /// An empty, untyped list.
    Empty = RawTagType::END,
    /// A list of signed 8-bit integers.
    Byte(Vec<i8>) = RawTagType::BYTE,
    /// A list of signed 16-bit integers.
    Short(Vec<i16>) = RawTagType::SHORT,
    /// A list of signed 32-bit integers.
    Int(Vec<i32>) = RawTagType::INT,
    /// A list of signed 64-bit integers.
    Long(Vec<i64>) = RawTagType::LONG,
    /// A list of 32-bit floating point numbers.
    Float(Vec<f32>) = RawTagType::FLOAT,
    /// A list of 64-bit floating point numbers.
    Double(Vec<f64>) = RawTagType::DOUBLE,
    /// A list of arrays of signed 8-bit integers.
    ByteArray(Vec<Vec<i8>>) = RawTagType::BYTE_ARRAY,
    /// A list of [`Mutf8String`]s.
    String(Vec<Mutf8String>) = RawTagType::STRING,
    /// A list of [`NbtListTag`]s.
    List(Vec<NbtListTag>) = RawTagType::LIST,
    /// A list of [`NbtCompound`]s.
    Compound(Vec<NbtCompound>) = RawTagType::COMPOUND,
    /// A list of arrays of signed 32-bit integers.
    IntArray(Vec<Vec<i32>>) = RawTagType::INT_ARRAY,
    /// A list of arrays of signed 64-bit integers.
    LongArray(Vec<Vec<i64>>) = RawTagType::LONG_ARRAY,
}

impl From<Vec<i8>> for NbtListTag {
    fn from(value: Vec<i8>) -> Self { NbtListTag::Byte(value) }
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
impl From<Vec<Vec<i8>>> for NbtListTag {
    fn from(value: Vec<Vec<i8>>) -> Self { NbtListTag::ByteArray(value) }
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
