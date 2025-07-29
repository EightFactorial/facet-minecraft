use crate::{borrowed::BorrowedRef, format::raw::RawCompound, mutf8::Mutf8String};

#[repr(u8)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, PartialEq)]
pub enum RawTag<'a> {
    /// A signed 8-bit integer.
    Byte(BorrowedRef<'a, i8>) = RawTagType::BYTE,
    /// A signed 16-bit integer.
    Short(BorrowedRef<'a, i16>) = RawTagType::SHORT,
    /// A signed 32-bit integer.
    Int(BorrowedRef<'a, i32>) = RawTagType::INT,
    /// A signed 64-bit integer.
    Long(BorrowedRef<'a, i64>) = RawTagType::LONG,
    /// A 32-bit floating point number.
    Float(BorrowedRef<'a, f32>) = RawTagType::FLOAT,
    /// A 64-bit floating point number.
    Double(BorrowedRef<'a, f64>) = RawTagType::DOUBLE,
    /// An array of signed 8-bit integers.
    ByteArray(BorrowedRef<'a, [i8]>) = RawTagType::BYTE_ARRAY,
    /// A [`Mutf8Str`].
    String(BorrowedRef<'a, Mutf8String>) = RawTagType::STRING,
    /// An [`BorrowedListTag`].
    List(RawListTag<'a>) = RawTagType::LIST,
    /// An [`BorrowedCompound`].
    Compound(RawCompound<'a>) = RawTagType::COMPOUND,
    /// An array of signed 32-bit integers.
    IntArray(BorrowedRef<'a, [i32]>) = RawTagType::INT_ARRAY,
    /// An array of signed 64-bit integers.
    LongArray(BorrowedRef<'a, [i64]>) = RawTagType::LONG_ARRAY,
}

impl<'a> RawTag<'a> {
    /// Create a new [`RawTag`] from a tag type and byte slice.
    #[must_use]
    pub const fn parse_data(tag: RawTagType, _data: &'a [u8]) -> Option<Self> {
        match tag {
            RawTagType::Byte => todo!(),
            RawTagType::Short => todo!(),
            RawTagType::Int => todo!(),
            RawTagType::Long => todo!(),
            RawTagType::Float => todo!(),
            RawTagType::Double => todo!(),
            RawTagType::ByteArray => todo!(),
            RawTagType::String => todo!(),
            RawTagType::List => todo!(),
            RawTagType::Compound => todo!(),
            RawTagType::IntArray => todo!(),
            RawTagType::LongArray => todo!(),
            RawTagType::End => unreachable!("`RawTagType::End` case should already be handled"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, PartialEq)]
pub enum RawListTag<'a> {
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
    ByteArray(BorrowedRef<'a, [&'a [i8]]>) = RawTagType::BYTE_ARRAY,
    /// A list of [`Mutf8Str`]s.
    String(BorrowedRef<'a, [&'a Mutf8String]>) = RawTagType::STRING,
    /// A list of [`BorrowedListTag`]s.
    List(BorrowedRef<'a, [RawListTag<'a>]>) = RawTagType::LIST,
    /// A list of [`BorrowedCompound`]s.
    Compound(BorrowedRef<'a, [RawCompound<'a>]>) = RawTagType::COMPOUND,
    /// A list of arrays of signed 32-bit integers.
    IntArray(BorrowedRef<'a, [&'a [i32]]>) = RawTagType::INT_ARRAY,
    /// A list of arrays of signed 64-bit integers.
    LongArray(BorrowedRef<'a, [&'a [i64]]>) = RawTagType::LONG_ARRAY,
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawTagType {
    End = RawTagType::END,
    Byte = RawTagType::BYTE,
    Short = RawTagType::SHORT,
    Int = RawTagType::INT,
    Long = RawTagType::LONG,
    Float = RawTagType::FLOAT,
    Double = RawTagType::DOUBLE,
    ByteArray = RawTagType::BYTE_ARRAY,
    String = RawTagType::STRING,
    List = RawTagType::LIST,
    Compound = RawTagType::COMPOUND,
    IntArray = RawTagType::INT_ARRAY,
    LongArray = RawTagType::LONG_ARRAY,
}

#[rustfmt::skip]
impl RawTagType {
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

impl RawTagType {
    /// Get the [`RawTagType`] from it's byte representation.
    ///
    /// Returns `None` if the byte does not correspond to a valid NBT tag type.
    #[must_use]
    pub const fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            RawTagType::END => Some(RawTagType::End),
            RawTagType::BYTE => Some(RawTagType::Byte),
            RawTagType::SHORT => Some(RawTagType::Short),
            RawTagType::INT => Some(RawTagType::Int),
            RawTagType::LONG => Some(RawTagType::Long),
            RawTagType::FLOAT => Some(RawTagType::Float),
            RawTagType::DOUBLE => Some(RawTagType::Double),
            RawTagType::BYTE_ARRAY => Some(RawTagType::ByteArray),
            RawTagType::STRING => Some(RawTagType::String),
            RawTagType::LIST => Some(RawTagType::List),
            RawTagType::COMPOUND => Some(RawTagType::Compound),
            RawTagType::INT_ARRAY => Some(RawTagType::IntArray),
            RawTagType::LONG_ARRAY => Some(RawTagType::LongArray),
            _ => None,
        }
    }

    /// Get the byte representation of a [`RawTagType`].
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        match self {
            RawTagType::End => RawTagType::END,
            RawTagType::Byte => RawTagType::BYTE,
            RawTagType::Short => RawTagType::SHORT,
            RawTagType::Int => RawTagType::INT,
            RawTagType::Long => RawTagType::LONG,
            RawTagType::Float => RawTagType::FLOAT,
            RawTagType::Double => RawTagType::DOUBLE,
            RawTagType::ByteArray => RawTagType::BYTE_ARRAY,
            RawTagType::String => RawTagType::STRING,
            RawTagType::List => RawTagType::LIST,
            RawTagType::Compound => RawTagType::COMPOUND,
            RawTagType::IntArray => RawTagType::INT_ARRAY,
            RawTagType::LongArray => RawTagType::LONG_ARRAY,
        }
    }
}
