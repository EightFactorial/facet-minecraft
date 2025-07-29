use alloc::vec::Vec;

use super::BorrowedCompound;
use crate::{
    borrowed::BorrowedRef,
    format::{
        owned::{NbtListTag, NbtTag},
        raw::RawTagType,
    },
    mutf8::Mutf8Str,
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowedTag<'a> {
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
    ByteArray(BorrowedRef<'a, [i8]>) = RawTagType::BYTE_ARRAY,
    /// A [`Mutf8Str`].
    String(&'a Mutf8Str) = RawTagType::STRING,
    /// An [`BorrowedListTag`].
    List(BorrowedListTag<'a>) = RawTagType::LIST,
    /// An [`BorrowedCompound`].
    Compound(BorrowedCompound<'a>) = RawTagType::COMPOUND,
    /// An array of signed 32-bit integers.
    IntArray(BorrowedRef<'a, [i32]>) = RawTagType::INT_ARRAY,
    /// An array of signed 64-bit integers.
    LongArray(BorrowedRef<'a, [i64]>) = RawTagType::LONG_ARRAY,
}

impl BorrowedTag<'_> {
    /// Convert the [`BorrowedTag`] into an owned [`NbtTag`].
    #[must_use]
    pub fn to_owned(self) -> NbtTag {
        match self {
            BorrowedTag::Byte(val) => NbtTag::Byte(val),
            BorrowedTag::Short(val) => NbtTag::Short(val),
            BorrowedTag::Int(val) => NbtTag::Int(val),
            BorrowedTag::Long(val) => NbtTag::Long(val),
            BorrowedTag::Float(val) => NbtTag::Float(val),
            BorrowedTag::Double(val) => NbtTag::Double(val),
            BorrowedTag::ByteArray(val) => NbtTag::ByteArray(val.collect()),
            BorrowedTag::String(val) => NbtTag::String(val.to_mutf8_string()),
            BorrowedTag::List(val) => NbtTag::List(val.to_owned()),
            BorrowedTag::Compound(val) => NbtTag::Compound(val.to_owned()),
            BorrowedTag::IntArray(val) => NbtTag::IntArray(val.collect()),
            BorrowedTag::LongArray(val) => NbtTag::LongArray(val.collect()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowedListTag<'a> {
    /// An empty, untyped list.
    Empty = RawTagType::END,
    /// A list of signed 8-bit integers.
    Byte(BorrowedRef<'a, [i8]>) = RawTagType::BYTE,
    /// A list of signed 16-bit integers.
    Short(BorrowedRef<'a, [i16]>) = RawTagType::SHORT,
    /// A list of signed 32-bit integers.
    Int(BorrowedRef<'a, [i32]>) = RawTagType::INT,
    /// A list of signed 64-bit integers.
    Long(BorrowedRef<'a, [i64]>) = RawTagType::LONG,
    /// A list of 32-bit floating point numbers.
    Float(BorrowedRef<'a, [f32]>) = RawTagType::FLOAT,
    /// A list of 64-bit floating point numbers.
    Double(BorrowedRef<'a, [f64]>) = RawTagType::DOUBLE,
    /// A list of arrays of signed 8-bit integers.
    ByteArray(Vec<BorrowedRef<'a, [i8]>>) = RawTagType::BYTE_ARRAY,
    /// A list of [`Mutf8Str`]s.
    String(Vec<&'a Mutf8Str>) = RawTagType::STRING,
    /// A list of [`BorrowedListTag`]s.
    List(Vec<BorrowedListTag<'a>>) = RawTagType::LIST,
    /// A list of [`BorrowedCompound`]s.
    Compound(Vec<BorrowedCompound<'a>>) = RawTagType::COMPOUND,
    /// A list of arrays of signed 32-bit integers.
    IntArray(Vec<BorrowedRef<'a, [i32]>>) = RawTagType::INT_ARRAY,
    /// A list of arrays of signed 64-bit integers.
    LongArray(Vec<BorrowedRef<'a, [i64]>>) = RawTagType::LONG_ARRAY,
}

impl BorrowedListTag<'_> {
    /// Convert the [`BorrowedListTag`] into an owned [`NbtListTag`].
    #[must_use]
    pub fn to_owned(self) -> NbtListTag {
        match self {
            BorrowedListTag::Empty => NbtListTag::Empty,
            BorrowedListTag::Byte(val) => NbtListTag::Byte(val.collect()),
            BorrowedListTag::Short(val) => NbtListTag::Short(val.collect()),
            BorrowedListTag::Int(val) => NbtListTag::Int(val.collect()),
            BorrowedListTag::Long(val) => NbtListTag::Long(val.collect()),
            BorrowedListTag::Float(val) => NbtListTag::Float(val.collect()),
            BorrowedListTag::Double(val) => NbtListTag::Double(val.collect()),
            BorrowedListTag::ByteArray(val) => {
                NbtListTag::ByteArray(val.into_iter().map(Iterator::collect).collect())
            }
            BorrowedListTag::String(val) => {
                NbtListTag::String(val.into_iter().map(Mutf8Str::to_mutf8_string).collect())
            }
            BorrowedListTag::List(val) => {
                NbtListTag::List(val.into_iter().map(Self::to_owned).collect())
            }
            BorrowedListTag::Compound(val) => {
                NbtListTag::Compound(val.into_iter().map(BorrowedCompound::to_owned).collect())
            }
            BorrowedListTag::IntArray(val) => {
                NbtListTag::IntArray(val.into_iter().map(Iterator::collect).collect())
            }
            BorrowedListTag::LongArray(val) => {
                NbtListTag::LongArray(val.into_iter().map(Iterator::collect).collect())
            }
        }
    }
}

impl From<BorrowedListTag<'_>> for NbtListTag {
    #[inline]
    fn from(tag: BorrowedListTag<'_>) -> Self { tag.to_owned() }
}
