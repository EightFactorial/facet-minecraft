use core::{
    convert::{AsMut, AsRef, From},
    error::Error,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::WriteAdapter;

/// A `no_std`-compatible adapter for writing to a mutable slice of bytes.
///
/// Comparable to `std::io::Cursor<&mut [u8]>`.
#[derive(Debug)]
pub struct SliceCursor<'a>(usize, &'a mut [u8]);

impl<'a> SliceCursor<'a> {
    /// Create a new [`SliceWriter`] from a mutable slice of bytes.
    #[must_use]
    pub const fn new(slice: &'a mut [u8]) -> Self { SliceCursor(0, slice) }

    /// Get the current position in the slice.
    #[must_use]
    pub const fn position(&self) -> usize { self.0 }

    /// Return the inner mutable slice of bytes.
    #[must_use]
    pub const fn into_inner(self) -> &'a mut [u8] { self.1 }

    /// Split the slice into two at the current position.
    ///
    /// The first containing the bytes that have been written so far,
    /// and the second containing the remaining bytes.
    #[must_use]
    pub const fn into_split(self) -> (&'a mut [u8], &'a mut [u8]) { self.1.split_at_mut(self.0) }

    /// Get the slice of bytes that have been written so far.
    #[must_use]
    pub const fn written_slice(&self) -> &[u8] { self.1.split_at(self.0).0 }

    /// Get the remaining bytes that have not been written to.
    #[must_use]
    pub const fn remaining_slice(&self) -> &[u8] { self.1.split_at(self.0).1 }
}

impl WriteAdapter for SliceCursor<'_> {
    type Error = SliceError;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        if self.0 + buf.len() <= self.1.len() {
            self.1[self.0..self.0 + buf.len()].copy_from_slice(buf);
            self.0 += buf.len();
            Ok(())
        } else {
            Err(SliceError)
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SliceError;

impl Error for SliceError {}

impl Debug for SliceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SliceWriter error: not enough space in slice")
    }
}

impl Display for SliceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SliceWriter error: not enough space in slice")
    }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<[u8]> for SliceCursor<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.1 }
}

impl AsMut<[u8]> for SliceCursor<'_> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] { self.1 }
}

impl Deref for SliceCursor<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target { self.1 }
}

impl DerefMut for SliceCursor<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { self.1 }
}

impl<'a, T: AsMut<[u8]> + AsRef<[u8]>> From<&'a mut T> for SliceCursor<'a> {
    fn from(slice: &'a mut T) -> Self { SliceCursor::new(slice.as_mut()) }
}

impl<'a> From<SliceCursor<'a>> for &'a mut [u8] {
    fn from(writer: SliceCursor<'a>) -> Self { writer.into_inner() }
}
