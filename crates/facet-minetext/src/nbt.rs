//! TODO

use facet_nbt::prelude::*;

use crate::prelude::*;

impl<'a> BorrowedText<'a> {
    /// Converts a [`BorrowedText`] into a [`BorrowedNbt`].
    ///
    /// # Errors
    /// Returns an error if TODO
    pub fn try_as_nbt(&self) -> Result<BorrowedNbt<'a>, NbtTextError> {
        let compound = BorrowedCompound::with_capacity(1);

        Ok(BorrowedNbt::from_parts(None, compound))
    }

    /// Converts a [`BorrowedNbt`] into a [`BorrowedText`].
    ///
    /// # Errors
    /// Returns an error if the NBT does not contain formatted text.
    pub fn try_from_nbt(_nbt: &BorrowedNbt<'a>) -> Result<Self, NbtTextError> {
        let text = BorrowedText::new(TextContent::from(""));

        Ok(text)
    }
}

impl<'a> TryFrom<BorrowedNbt<'a>> for BorrowedText<'a> {
    type Error = NbtTextError;

    #[inline]
    fn try_from(nbt: BorrowedNbt<'a>) -> Result<Self, Self::Error> {
        BorrowedText::try_from_nbt(&nbt)
    }
}
impl<'a> TryFrom<&'a BorrowedNbt<'a>> for BorrowedText<'a> {
    type Error = NbtTextError;

    #[inline]
    fn try_from(nbt: &'a BorrowedNbt<'a>) -> Result<Self, Self::Error> {
        BorrowedText::try_from_nbt(nbt)
    }
}

impl<'a> TryFrom<BorrowedText<'a>> for BorrowedNbt<'a> {
    type Error = NbtTextError;

    #[inline]
    fn try_from(text: BorrowedText<'a>) -> Result<Self, Self::Error> {
        BorrowedText::try_as_nbt(&text)
    }
}
impl<'a> TryFrom<&'a BorrowedText<'a>> for BorrowedNbt<'a> {
    type Error = NbtTextError;

    #[inline]
    fn try_from(text: &'a BorrowedText<'a>) -> Result<Self, Self::Error> {
        BorrowedText::try_as_nbt(text)
    }
}

// -------------------------------------------------------------------------------------------------

/// An error that occurs when converting between
/// [`BorrowedText`] and [`BorrowedNbt`].
pub enum NbtTextError {
    #[expect(missing_docs)]
    Placeholder,
}
