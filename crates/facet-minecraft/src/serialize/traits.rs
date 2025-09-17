#![allow(clippy::result_large_err, reason = "Error is large if rich diagnostics are enabled")]

use core::fmt::{Debug, Display};

use facet_core::Facet;
use facet_reflect::Peek;

use super::SerError;

/// A trait for types that can produce a [`Peek`].
///
/// A workaround for [`Facet`] not being `dyn-compatible`.
pub trait Peekable<'facet> {
    /// Create a [`Peek`] for this type.
    fn peek<'input>(&'input self) -> Peek<'input, 'facet>;
}

impl<'facet, T: Facet<'facet> + ?Sized> Peekable<'facet> for T {
    fn peek<'input>(&'input self) -> Peek<'input, 'facet> { Peek::new(self) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for serializing primitive protocol types.
pub trait Serializer {
    /// Serialize a unit from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid unit.
    #[inline]
    fn serialize_unit<W: Writer>(
        &mut self,
        input: (),
        _writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `()`");

        Ok(input)
    }

    /// Serialize a [`bool`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`bool`].
    #[inline]
    fn serialize_bool<W: Writer>(
        &mut self,
        input: bool,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `bool`");

        self.serialize_u8(u8::from(input), writer)
    }

    /// Serialize a [`u8`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u8`].
    fn serialize_u8<W: Writer>(
        &mut self,
        input: u8,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `u8`");

        writer.write(core::slice::from_ref(&input)).unwrap(); // TODO: Handle error
        Ok(())
    }

    /// Serialize a [`i8`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i8`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_i8<W: Writer>(
        &mut self,
        input: i8,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `i8`");

        self.serialize_u8(input as u8, writer)
    }

    /// Serialize a [`u16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u16`].
    fn serialize_u16<W: Writer>(
        &mut self,
        input: u16,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `u16`");

        writer.write(&input.to_be_bytes()).unwrap(); // TODO: Handle error
        Ok(())
    }

    /// Serialize a variable-length [`u16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u16`].
    #[expect(unused_assignments, reason = "Only unused when the input is `0`")]
    fn serialize_var_u16<W: Writer>(
        &mut self,
        mut input: u16,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_u16`");

        let mut byte = 0u8;
        let mut count = 0u8;
        while (input != 0 || count == 0) && count < 3 {
            byte = (input & 0b0111_1111) as u8;
            input = (input >> 7) & (u16::MAX >> 6);
            if input != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte, writer)?;
        }
        Ok(())
    }

    /// Serialize a [`i16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i16`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_i16<W: Writer>(
        &mut self,
        input: i16,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `i16`");

        self.serialize_u16(input as u16, writer)
    }

    /// Serialize a variable-length [`i16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i16`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_var_i16<W: Writer>(
        &mut self,
        input: i16,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_i16`");

        self.serialize_var_u16(input as u16, writer)
    }

    /// Serialize a [`u32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u32`].
    fn serialize_u32<W: Writer>(
        &mut self,
        input: u32,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `u32`");

        writer.write(&input.to_be_bytes()).unwrap(); // TODO: Handle error
        Ok(())
    }

    /// Serialize a variable-length [`u32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u32`].
    #[expect(unused_assignments, reason = "Only unused when the input is `0`")]
    fn serialize_var_u32<W: Writer>(
        &mut self,
        mut input: u32,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_u32`");

        let mut byte = 0u8;
        let mut count = 0u8;
        while (input != 0 || count == 0) && count < 5 {
            byte = (input & 0b0111_1111) as u8;
            input = (input >> 7) & (u32::MAX >> 6);
            if input != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte, writer)?;
        }
        Ok(())
    }

    /// Serialize a [`i32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i32`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_i32<W: Writer>(
        &mut self,
        input: i32,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `i32`");

        self.serialize_u32(input as u32, writer)
    }

    /// Serialize a variable-length [`i32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i32`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_var_i32<W: Writer>(
        &mut self,
        input: i32,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_i32`");

        self.serialize_var_u32(input as u32, writer)
    }

    /// Serialize a [`u64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u64`].
    fn serialize_u64<W: Writer>(
        &mut self,
        input: u64,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `u64`");

        writer.write(&input.to_be_bytes()).unwrap(); // TODO: Handle error
        Ok(())
    }

    /// Serialize a variable-length [`u64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u64`].
    #[expect(unused_assignments, reason = "Only unused when the input is `0`")]
    fn serialize_var_u64<W: Writer>(
        &mut self,
        mut input: u64,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_u64`");

        let mut byte = 0u8;
        let mut count = 0u8;
        while (input != 0 || count == 0) && count < 10 {
            byte = (input & 0b0111_1111) as u8;
            input = (input >> 7) & (u64::MAX >> 6);
            if input != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte, writer)?;
        }
        Ok(())
    }

    /// Serialize a [`i64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i64`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_i64<W: Writer>(
        &mut self,
        input: i64,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `i64`");

        self.serialize_u64(input as u64, writer)
    }

    /// Serialize a variable-length [`i64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i64`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_var_i64<W: Writer>(
        &mut self,
        input: i64,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_i64`");

        self.serialize_var_u64(input as u64, writer)
    }

    /// Serialize a [`u128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u128`].
    fn serialize_u128<W: Writer>(
        &mut self,
        input: u128,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `u128`");

        writer.write(&input.to_be_bytes()).unwrap(); // TODO: Handle error
        Ok(())
    }

    /// Serialize a variable-length [`u128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u128`].
    #[expect(unused_assignments, reason = "Only unused when the input is `0`")]
    fn serialize_var_u128<W: Writer>(
        &mut self,
        mut input: u128,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_u128`");

        let mut byte = 0u8;
        let mut count = 0u8;
        while (input != 0 || count == 0) && count < 19 {
            byte = (input & 0b0111_1111) as u8;
            input = (input >> 7) & (u128::MAX >> 6);
            if input != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte, writer)?;
        }
        Ok(())
    }

    /// Serialize a [`i128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i128`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_i128<W: Writer>(
        &mut self,
        input: i128,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `i128`");

        self.serialize_u128(input as u128, writer)
    }

    /// Serialize a variable-length [`i128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i128`].
    #[inline]
    #[expect(clippy::cast_sign_loss, reason = "Wrapping is desired here")]
    fn serialize_var_i128<W: Writer>(
        &mut self,
        input: i128,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_i128`");

        self.serialize_var_u128(input as u128, writer)
    }

    /// Serialize a [`usize`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`usize`].
    #[inline]
    fn serialize_usize<W: Writer>(
        &mut self,
        input: usize,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `usize`");

        self.serialize_u64(input as u64, writer)
    }

    /// Serialize a variable-length [`usize`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`usize`].
    #[inline]
    fn serialize_var_usize<W: Writer>(
        &mut self,
        input: usize,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_usize`");

        self.serialize_var_u64(input as u64, writer)
    }

    /// Serialize a [`isize`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`isize`].
    #[inline]
    fn serialize_isize<W: Writer>(
        &mut self,
        input: isize,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `isize`");

        self.serialize_i64(input as i64, writer)
    }

    /// Serialize a variable-length [`isize`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`isize`].
    #[inline]
    fn serialize_var_isize<W: Writer>(
        &mut self,
        input: isize,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `var_isize`");

        self.serialize_var_i64(input as i64, writer)
    }

    /// Serialize a [`f32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`f32`].
    #[inline]
    fn serialize_f32<W: Writer>(
        &mut self,
        input: f32,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `f32`");

        self.serialize_u32(input.to_bits(), writer)
    }

    /// Serialize a [`f64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`f64`].
    #[inline]
    fn serialize_f64<W: Writer>(
        &mut self,
        input: f64,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `f64`");

        self.serialize_u64(input.to_bits(), writer)
    }

    /// Serialize a length-prefixed byte slice from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// length-prefix, or if there are not enough bytes in the input to
    /// satisfy the length.
    fn serialize_bytes<W: Writer>(
        &mut self,
        input: &[u8],
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `&[u8]`");

        self.serialize_var_usize(input.len(), writer)?;
        writer.write(input).unwrap(); // TODO: Handle error
        Ok(())
    }

    /// Serialize a length-prefixed UTF-8 string from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// length-prefix, if there are not enough bytes in the input to
    /// satisfy the length, or if the bytes are not valid UTF-8.
    #[inline]
    fn serialize_str<W: Writer>(
        &mut self,
        input: &str,
        writer: &mut W,
    ) -> Result<(), SerError<'static, W>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Serializing `&str`");

        self.serialize_bytes(input.as_bytes(), writer)
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for writing bytes to a sink.
pub trait Writer {
    /// The error type returned when writing fails.
    type Error: core::error::Error + Send + Sync + 'static;

    /// Write the contents of `buf` to the sink.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(not(feature = "std"))]
impl Writer for alloc::vec::Vec<u8> {
    type Error = core::convert::Infallible;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: std::io::Write> Writer for T {
    type Error = std::io::Error;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        <T as std::io::Write>::write_all(self, buf)
    }
}

// -------------------------------------------------------------------------------------------------

/// A cursor for writing to a mutable slice of bytes.
///
/// Comparable to `std::io::Cursor<&mut [u8]>`.
#[derive(Debug, PartialEq, Eq)]
pub struct SliceCursor<'a>(usize, &'a mut [u8]);

impl<'a> SliceCursor<'a> {
    /// Create a new [`SliceCursor`] that writes to the given mutable slice.
    #[inline]
    #[must_use]
    pub const fn new(slice: &'a mut [u8]) -> Self { Self(0, slice) }

    /// Create a new [`SliceCursor`] that writes to the given mutable slice.
    #[inline]
    #[must_use]
    pub fn new_from<T: AsMut<[u8]> + ?Sized>(slice: &'a mut T) -> Self { Self::new(slice.as_mut()) }

    /// Get the current position of the cursor.
    #[inline]
    #[must_use]
    pub const fn position(&self) -> usize { self.0 }

    /// Set the current position of the cursor.
    ///
    /// # Panics
    ///
    /// Panics if the position is out of bounds.
    #[inline]
    pub const fn set_position(&mut self, pos: usize) {
        assert!(pos <= self.1.len(), "Position out of bounds");
        self.0 = pos;
    }

    /// Get a reference to the bytes that have been written so far.
    #[must_use]
    pub const fn written_ref(&self) -> &[u8] { self.1.split_at(self.0).0 }

    /// Get a mutable reference to the bytes that have been written so far.
    #[must_use]
    pub const fn written_mut(&mut self) -> &mut [u8] { self.1.split_at_mut(self.0).0 }

    /// Get the remaining capacity of the cursor.
    #[must_use]
    pub const fn remaining(&self) -> usize { self.1.len() - self.0 }

    /// Get a reference to the remaining bytes that have not been written to.
    #[must_use]
    pub const fn remaining_ref(&self) -> &[u8] { self.1.split_at(self.0).1 }

    /// Get a mutable reference to the remaining bytes that have not been
    /// written to.
    #[must_use]
    pub const fn remaining_mut(&mut self) -> &mut [u8] { self.1.split_at_mut(self.0).1 }

    /// Return the inner mutable slice of bytes.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> &'a mut [u8] { self.1 }

    /// Split the slice into two at the current position,
    /// returning references to both parts.
    ///
    /// The first containing the bytes that have been written so far,
    /// and the second containing the remaining bytes not yet written to.
    #[must_use]
    pub const fn as_split(&self) -> (&[u8], &[u8]) { self.1.split_at(self.0) }

    /// Split the slice into two at the current position,
    /// returning mutable references to both parts.
    ///
    /// The first containing the bytes that have been written so far,
    /// and the second containing the remaining bytes not yet written to.
    #[must_use]
    pub const fn as_split_mut(&mut self) -> (&mut [u8], &mut [u8]) { self.1.split_at_mut(self.0) }

    /// Split the slice into two at the current position.
    ///
    /// The first containing the bytes that have been written so far,
    /// and the second containing the remaining bytes not yet written to.
    #[must_use]
    pub const fn into_split(self) -> (&'a mut [u8], &'a mut [u8]) { self.1.split_at_mut(self.0) }
}

/// An error indicating that a slice is full and cannot accept more data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SliceFullError;

impl core::error::Error for SliceFullError {}
impl Display for SliceFullError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Slice is full")
    }
}

impl Writer for SliceCursor<'_> {
    type Error = SliceFullError;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        if self.0 + buf.len() <= self.1.len() {
            self.1[self.0..self.0 + buf.len()].copy_from_slice(buf);
            self.0 += buf.len();
            Ok(())
        } else {
            Err(SliceFullError)
        }
    }
}
