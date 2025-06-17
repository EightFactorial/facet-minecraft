//! A `no_std`-compatible adapter for writing bytes.

/// A trait used for writing bytes to a destination.
pub trait WriteAdapter {
    /// A type of error that can occur when writing.
    ///
    /// If the adapter does not have or support errors,
    /// this can be [`Infallible`](core::convert::Infallible).
    type Error;

    /// Writes a slice of bytes to the destination.
    ///
    /// # Errors
    /// If the write operation fails, it returns an error of type `Self::Error`.
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;

    /// Reserve capacity for `len` additional bytes.
    ///
    /// Optional, may or may not be implemented.
    fn reserve(&mut self, _len: usize) {}
}

// -------------------------------------------------------------------------------------------------

#[cfg(not(feature = "std"))]
impl WriteAdapter for alloc::vec::Vec<u8> {
    type Error = core::convert::Infallible;

    #[inline]
    #[expect(clippy::unit_arg)]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        Ok(alloc::vec::Vec::extend(self, buf))
    }

    #[inline]
    fn reserve(&mut self, len: usize) { alloc::vec::Vec::reserve(self, len); }
}

#[cfg(not(feature = "std"))]
impl WriteAdapter for &mut alloc::vec::Vec<u8> {
    type Error = core::convert::Infallible;

    #[inline]
    #[expect(clippy::unit_arg)]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        Ok(alloc::vec::Vec::extend(self, buf))
    }

    #[inline]
    fn reserve(&mut self, len: usize) { alloc::vec::Vec::reserve(self, len); }
}

// -------------------------------------------------------------------------------------------------

#[cfg(all(feature = "nightly", not(feature = "std")))]
impl WriteAdapter for core::io::BorrowedCursor<'_> {
    type Error = core::convert::Infallible;

    #[inline]
    #[expect(clippy::unit_arg)]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        Ok(core::io::BorrowedCursor::append(self, buf))
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
impl<T: std::io::Write> WriteAdapter for T {
    type Error = std::io::Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> { self.write_all(buf) }
}
