use super::RawTag;
use crate::{
    format::raw::{RawError, RawTagType, error::RawErrorKind},
    mutf8::Mutf8Str,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawNbt<'a>(Option<&'a Mutf8Str>, RawCompound<'a>);

impl<'a> RawNbt<'a> {
    /// Create a new named [`RawNbt`] from a byte slice.
    ///
    /// # Panics
    /// Panics if the byte slice is not a valid named [`RawNbt`].
    #[must_use]
    pub const fn new_named(input: &'a [u8]) -> Self {
        match Self::try_new_named(input) {
            Ok(nbt) => nbt,
            Err(err) => {
                const_panic::concat_panic!("", err.kind().static_message())
            }
        }
    }

    /// Try to create a new named [`RawNbt`] from a byte slice.
    ///
    /// # Errors
    /// Returns an error if the byte slice is not a valid named [`RawNbt`].
    #[must_use]
    pub const fn try_new_named(input: &'a [u8]) -> Result<Self, RawError<'a>> {
        match input.split_first() {
            Some((&RawTagType::COMPOUND, data)) => {
                let (name, data) = Mutf8Str::new_raw_prefixed(data);
                Ok(Self(Some(name), RawCompound::new_unchecked(data)))
            }
            Some((&tag, data)) => {
                Err(RawError::new(RawErrorKind::InvalidTagType(tag), data).with_input(input))
            }
            None => Err(RawError::new(RawErrorKind::EndOfInput, input).with_input(input)),
        }
    }

    /// Create a new unnamed [`RawNbt`] from a byte slice.
    ///
    /// # Panics
    /// Panics if the byte slice is not a valid unnamed [`RawNbt`].
    #[must_use]
    pub const fn new_unnamed(input: &'a [u8]) -> Self {
        match Self::try_new_unnamed(input) {
            Ok(nbt) => nbt,
            Err(err) => {
                const_panic::concat_panic!("", err.kind().static_message())
            }
        }
    }

    /// Try to create a new unnamed [`RawNbt`] from a byte slice.
    ///
    /// # Errors
    /// Returns an error if the byte slice is not a valid unnamed [`RawNbt`].
    #[must_use]
    pub const fn try_new_unnamed(input: &'a [u8]) -> Result<Self, RawError<'a>> {
        match input.split_first() {
            Some((&RawTagType::COMPOUND, data)) => Ok(Self(None, RawCompound::new_unchecked(data))),
            Some((&tag, data)) => {
                Err(RawError::new(RawErrorKind::InvalidTagType(tag), data).with_input(input))
            }
            None => Err(RawError::new(RawErrorKind::EndOfInput, input).with_input(input)),
        }
    }

    /// Get the name of the [`RawNbt`], if it has one.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> Option<&'a Mutf8Str> { self.0 }

    /// Get the inner [`RawCompound`] of the [`RawNbt`].
    #[inline]
    #[must_use]
    pub const fn compound(&self) -> &RawCompound<'a> { &self.1 }

    /// Create a new [`BorrowedNbt`] from this [`RawNbt`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_borrowed(&self) -> crate::format::borrowed::BorrowedNbt<'a> {
        crate::format::borrowed::BorrowedNbt::from_parts(self.0, self.1.to_borrowed())
    }
}

impl<'a> core::ops::Deref for RawNbt<'a> {
    type Target = RawCompound<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.1 }
}
impl<'a> core::ops::DerefMut for RawNbt<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.1 }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawCompound<'a>(&'a [u8]);

impl<'a> RawCompound<'a> {
    /// Create a new [`RawCompound`] from a byte slice.
    ///
    /// # Warning
    /// This requires that the byte slice is a valid NBT compound.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(bytes: &'a [u8]) -> Self { Self(bytes) }

    /// Get the raw inner byte slice of the [`RawCompound`].
    #[inline]
    #[must_use]
    pub const fn as_raw_bytes(&self) -> &'a [u8] { self.0 }

    /// Get the next string-tag pair of the [`RawCompound`].
    ///
    /// # Warning
    /// Once this function returns `None`, it should never be called again.
    ///
    /// Calling it again may start indexing into trailing data,
    /// returning garbage results or panicking.
    #[must_use]
    pub const fn next_entry(&mut self) -> Option<(&'a Mutf8Str, RawTag<'a>)> {
        match self.0.split_first() {
            Some((&RawTagType::END, remaining)) => {
                self.0 = remaining;
                None
            }
            Some((&tag, data)) => {
                let Some(tag) = RawTagType::from_byte(tag) else { return None };
                let (name, data) = Mutf8Str::new_raw_prefixed(data);
                let Some((tag, remaining)) = RawTag::parse_data(tag, data) else { return None };
                self.0 = remaining;
                Some((name, tag))
            }
            None => None,
        }
    }

    /// Create a new [`BorrowedCompound`] from this [`RawCompound`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_borrowed(&self) -> crate::format::borrowed::BorrowedCompound<'a> {
        self.clone().map(|(k, v)| (k, v.to_borrowed())).collect()
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Iterator for RawCompound<'a> {
    type Item = (&'a Mutf8Str, RawTag<'a>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.next_entry() }
}

// -------------------------------------------------------------------------------------------------

impl core::convert::AsRef<[u8]> for RawCompound<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_raw_bytes() }
}

impl core::borrow::Borrow<[u8]> for RawCompound<'_> {
    #[inline]
    fn borrow(&self) -> &[u8] { self.as_raw_bytes() }
}
