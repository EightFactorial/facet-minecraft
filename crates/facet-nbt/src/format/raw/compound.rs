use super::RawTag;
use crate::{format::raw::RawTagType, mutf8::Mutf8Str};

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
