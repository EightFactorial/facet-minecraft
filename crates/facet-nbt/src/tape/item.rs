use core::fmt::Debug;

/// A single item in the NBT tape.
///
/// The internal layout is as follows:
/// - Position: `0..64`    (64 bits)
/// - Tag Data: `64..96`   (32 bits)
/// - Tag Type: `96..104`  (8 bits)
/// - Unused:   `104..128` (24 bits)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NbtTapeItem(u128);

impl Debug for NbtTapeItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (tag, position, data) = self.into_parts();
        f.debug_struct("NbtTapeItem")
            .field("tag", &tag)
            .field("position", &position)
            .field("data", &data)
            .finish()
    }
}

impl NbtTapeItem {
    /// Create a new [`TapeItem`] from a tag, position, and associated data.
    #[must_use]
    pub const fn new(tag: NbtTapeTag, position: u64, data: u32) -> Self {
        let mut value = 0u128;
        value |= position as u128;
        value |= (data as u128) << 64;
        value |= (tag as u128) << 96;
        Self(value)
    }

    /// Get the position of this item from the start of the NBT data.
    #[must_use]
    pub const fn position(&self) -> u64 { (self.0 & 0xFFFF_FFFF_FFFF) as u64 }

    /// Get the data associated with this item.
    #[must_use]
    pub const fn data(&self) -> u32 { ((self.0 >> 64) & 0xFFFF_FFFF) as u32 }

    /// Get the [`NbtTapeTag`] of this item.
    #[must_use]
    pub const fn tag(&self) -> NbtTapeTag {
        let tag = ((self.0 >> 96) & 0xFF) as u8;
        // SAFETY: `raw_tag` is guaranteed to be a valid `NbtTapeTag` variant
        unsafe { core::mem::transmute::<u8, NbtTapeTag>(tag) }
    }

    /// Decompose this item into its parts: (tag, offset, data).
    #[must_use]
    pub const fn into_parts(self) -> (NbtTapeTag, u64, u32) {
        let position = (self.0 & 0xFFFF_FFFF_FFFF) as u64;
        let data = ((self.0 >> 64) & 0xFFFF_FFFF) as u32;
        let tag = ((self.0 >> 96) & 0xFF) as u8;

        // SAFETY: `tag` is guaranteed to be a valid `NbtTapeTag` variant
        let tag = unsafe { core::mem::transmute::<u8, NbtTapeTag>(tag) };

        (tag, position, data)
    }

    /// Add an offset in bytes to the item's current offset.
    ///
    /// This is useful when adjusting the start of the input data.
    ///
    /// # Panics
    ///
    /// Panics if the new position would overflow `u64`.
    pub const fn add_offset(&mut self, offset: u64) {
        let (tag, position, data) = self.into_parts();
        if let Some(position) = position.checked_add(offset) {
            *self = Self::new(tag, position, data);
        } else {
            panic!("Tag position overflowed when adding offset!");
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The type of item contained by an [`NbtTapeItem`].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(missing_docs, reason = "WIP")]
pub enum NbtTapeTag {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,

    // List variants
    ListEmpty = 9,
    ListByte = 13,
    ListShort = 14,
    ListInt = 15,
    ListLong = 16,
    ListFloat = 17,
    ListDouble = 18,
    ListByteArray = 19,
    ListString = 20,
    ListCompound = 21,
    ListIntArray = 22,
    ListLongArray = 23,
}

#[rustfmt::skip]
#[expect(missing_docs, reason = "Tag `u8`s")]
impl NbtTapeTag {
    pub const END: u8 = NbtTapeTag::End as u8;
    pub const BYTE: u8 = NbtTapeTag::Byte as u8;
    pub const SHORT: u8 = NbtTapeTag::Short as u8;
    pub const INT: u8 = NbtTapeTag::Int as u8;
    pub const LONG: u8 = NbtTapeTag::Long as u8;
    pub const FLOAT: u8 = NbtTapeTag::Float as u8;
    pub const DOUBLE: u8 = NbtTapeTag::Double as u8;
    pub const BYTE_ARRAY: u8 = NbtTapeTag::ByteArray as u8;
    pub const STRING: u8 = NbtTapeTag::String as u8;
    pub const LIST: u8 = NbtTapeTag::ListEmpty as u8;
    pub const COMPOUND: u8 = NbtTapeTag::Compound as u8;
    pub const INT_ARRAY: u8 = NbtTapeTag::IntArray as u8;
    pub const LONG_ARRAY: u8 = NbtTapeTag::LongArray as u8;
}

impl NbtTapeTag {
    /// Returns `true` if the tag is a primitive type
    ///
    /// Primitive Tags:
    /// - [`NbtTapeTag::Byte`]
    /// - [`NbtTapeTag::Short`]
    /// - [`NbtTapeTag::Int`]
    /// - [`NbtTapeTag::Long`]
    /// - [`NbtTapeTag::Float`]
    /// - [`NbtTapeTag::Double`]
    #[must_use]
    pub const fn is_primitive(&self) -> bool {
        matches!(
            self,
            NbtTapeTag::Byte
                | NbtTapeTag::Short
                | NbtTapeTag::Int
                | NbtTapeTag::Long
                | NbtTapeTag::Float
                | NbtTapeTag::Double
        )
    }

    /// Returns `true` if the tag is a numeric array type
    ///
    /// Numeric Array Tags:
    /// - [`NbtTapeTag::ByteArray`]
    /// - [`NbtTapeTag::IntArray`]
    /// - [`NbtTapeTag::LongArray`]
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, NbtTapeTag::ByteArray | NbtTapeTag::IntArray | NbtTapeTag::LongArray)
    }

    /// Returns `true` if the tag is a list type
    ///
    /// List Tags:
    /// - [`NbtTapeTag::ListEmpty`]
    /// - [`NbtTapeTag::ListByte`]
    /// - [`NbtTapeTag::ListShort`]
    /// - [`NbtTapeTag::ListInt`]
    /// - [`NbtTapeTag::ListLong`]
    /// - [`NbtTapeTag::ListFloat`]
    /// - [`NbtTapeTag::ListDouble`]
    /// - [`NbtTapeTag::ListByteArray`]
    /// - [`NbtTapeTag::ListString`]
    /// - [`NbtTapeTag::ListCompound`]
    /// - [`NbtTapeTag::ListIntArray`]
    /// - [`NbtTapeTag::ListLongArray`]
    #[must_use]
    pub const fn is_list(&self) -> bool {
        matches!(
            self,
            NbtTapeTag::ListEmpty
                | NbtTapeTag::ListByte
                | NbtTapeTag::ListShort
                | NbtTapeTag::ListInt
                | NbtTapeTag::ListLong
                | NbtTapeTag::ListFloat
                | NbtTapeTag::ListDouble
                | NbtTapeTag::ListByteArray
                | NbtTapeTag::ListString
                | NbtTapeTag::ListCompound
                | NbtTapeTag::ListIntArray
                | NbtTapeTag::ListLongArray
        )
    }
}
