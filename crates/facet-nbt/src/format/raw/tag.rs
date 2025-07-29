#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawTag<'a>(RawTagType, &'a [u8]);

impl<'a> RawTag<'a> {
    /// Create a new [`RawTag`] from a tag type and byte slice.
    ///
    /// # Warning
    /// This requires that the tag type matches the byte slice.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(ty: RawTagType, bytes: &'a [u8]) -> Self { Self(ty, bytes) }

    /// Create a new [`RawTag`] from a byte slice.
    ///
    /// # Warning
    /// This requires that the byte slice is a valid NBT tag.
    #[must_use]
    pub const fn new_untyped(bytes: &'a [u8]) -> Option<Self> {
        match bytes.split_first() {
            Some((&first, bytes)) => match RawTagType::from_byte(first) {
                Some(RawTagType::End) | None => None,
                Some(ty) => Some(Self::new_unchecked(ty, bytes)),
            },
            None => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawListTag<'a>(RawTagType, &'a [u8]);

impl<'a> RawListTag<'a> {
    /// Create a new [`RawListTag`] from a list type and byte slice.
    ///
    /// # Warning
    /// This requires that the list type matches the byte slice.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(ty: RawTagType, bytes: &'a [u8]) -> Self { Self(ty, bytes) }

    /// Create a new [`RawListTag`] from a byte slice.
    ///
    /// # Warning
    /// This requires that the byte slice is a valid NBT list tag.
    #[must_use]
    pub const fn new_untyped(bytes: &'a [u8]) -> Option<Self> {
        match bytes.split_first() {
            Some((&first, bytes)) => match RawTagType::from_byte(first) {
                Some(ty) => Some(Self::new_unchecked(ty, bytes)),
                None => None,
            },
            None => None,
        }
    }
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
