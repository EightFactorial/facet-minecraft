//! TODO

use alloc::vec::Vec;

use facet_nbt::prelude::*;

use crate::prelude::*;

impl BorrowedText<'_> {
    /// Converts a [`BorrowedText`] into a [`NbtCompound`].
    #[must_use]
    pub fn as_nbt(&self) -> NbtCompound {
        let mut compound = NbtCompound::with_capacity(1);

        self.content.as_nbt(&mut compound);
        self.style.as_nbt(&mut compound);

        if !self.children.is_empty() {
            let mut extra = Vec::with_capacity(self.children.len());
            for child in &self.children {
                extra.push(child.as_nbt());
            }

            compound
                .insert(Mutf8String::new_str("extra"), NbtTag::List(NbtListTag::Compound(extra)));
        }

        compound
    }

    /// Converts a [`NbtCompound`] into a [`BorrowedText`].
    ///
    /// # Errors
    /// Returns an error if the NBT does not contain formatted text.
    pub fn try_from_nbt(_nbt: &NbtCompound) -> Result<Self, NbtTextError> {
        let text = BorrowedText::new(TextContent::from(""));

        Ok(text)
    }
}

impl TextContent<'_> {
    fn as_nbt(&self, compound: &mut NbtCompound) {
        match &self {
            TextContent::Text(c) => {
                compound.insert(
                    Mutf8String::new_str("type"),
                    NbtTag::String(Mutf8String::new_str("text")),
                );
                compound.insert(
                    Mutf8String::new_str("text"),
                    NbtTag::String(Mutf8String::new_str(&c.text)),
                );
            }
            TextContent::Translation(_c) => todo!(),
            TextContent::Score(_c) => todo!(),
            TextContent::Selector(_c) => todo!(),
            TextContent::Keybind(_c) => todo!(),
            TextContent::Nbt(_c) => todo!(),
        }
    }
}

impl TextStyle<'_> {
    fn as_nbt(&self, _compound: &mut NbtCompound) { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl TryFrom<NbtCompound> for BorrowedText<'_> {
    type Error = NbtTextError;

    #[inline]
    fn try_from(nbt: NbtCompound) -> Result<Self, Self::Error> { BorrowedText::try_from_nbt(&nbt) }
}
impl<'a> TryFrom<&'a NbtCompound> for BorrowedText<'a> {
    type Error = NbtTextError;

    #[inline]
    fn try_from(nbt: &'a NbtCompound) -> Result<Self, Self::Error> {
        BorrowedText::try_from_nbt(nbt)
    }
}

impl<'a> From<BorrowedText<'a>> for NbtCompound {
    #[inline]
    fn from(text: BorrowedText<'a>) -> Self { BorrowedText::as_nbt(&text) }
}
impl<'a> From<&BorrowedText<'a>> for NbtCompound {
    #[inline]
    fn from(text: &BorrowedText<'a>) -> Self { BorrowedText::as_nbt(text) }
}

// -------------------------------------------------------------------------------------------------

/// An error that occurs when converting between
/// [`BorrowedText`] and [`NbtCompound`].
pub enum NbtTextError {
    #[expect(missing_docs)]
    Placeholder,
}
