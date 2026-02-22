//! TODO
#![allow(clippy::elidable_lifetime_names, reason = "WIP")]
#![expect(clippy::result_large_err, reason = "An error variant contains the iterator")]

#[cfg(feature = "std")]
use alloc::vec::Vec;
use core::marker::PhantomData;

use facet::Facet;
#[cfg(feature = "std")]
use facet::HeapValue;

use crate::{
    deserialize::{
        error::{DeserializeError, DeserializeValueError, EndOfInput},
        iter::{DeserializeIter, PartialValue},
    },
    hint::TypeSizeHint,
};

pub mod error;
pub mod fns;
pub mod iter;

/// A trait for types that can be deserialized.
pub trait Deserialize<'facet>: Sized {
    /// The [`TypeSizeHint`] for this type.
    const SIZE_HINT: TypeSizeHint;

    /// Deserialize a value from a [`slice`](::core::primitive::slice),
    /// borrowing data where possible.
    ///
    /// If the type will outlive the input data, use
    /// [`Deserialize::from_slice_owned`] instead.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails.
    fn from_slice(slice: &'facet [u8]) -> Result<Self, DeserializeError<'facet>>
    where
        Self: Facet<'facet>,
    {
        DeserializeIter::<true>::new::<Self>()?
            .complete(borrowed_processor(InputCursor::new(slice)))?
            .materialize()
            .map_err(Into::into)
    }

    /// Deserialize a value from a [`slice`](::core::primitive::slice).
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails.
    #[inline]
    fn from_slice_owned(slice: &[u8]) -> Result<Self, DeserializeError<'static>>
    where
        Self: Facet<'static>,
    {
        Self::from_slice_remainder(slice).map(|(value, _)| value)
    }

    /// Deserialize a value from a [`slice`](::core::primitive::slice),
    /// returning any remaining data.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails.
    fn from_slice_remainder(slice: &[u8]) -> Result<(Self, &[u8]), DeserializeError<'static>>
    where
        Self: Facet<'static>,
    {
        let _value = DeserializeIter::<false>::new::<Self>()?
            .complete(owned_processor(InputCursor::new(slice)))?
            .materialize::<Self>()?;

        todo!()
    }

    /// Deserialize a value from a reader.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails or if reading
    /// fails.
    #[cfg(feature = "std")]
    fn from_reader<R: std::io::Read>(mut reader: R) -> Result<Self, DeserializeError<'static>>
    where
        Self: Facet<'static>,
    {
        from_coroutine(
            DeserializeIter::<false>::new::<Self>()?,
            move |buf: &mut [u8]| reader.read_exact(buf).map_err(Into::into),
            Self::SIZE_HINT,
        )?
        .materialize::<Self>()
        .map_err(Into::into)
    }

    /// Deserialize a value from a [`futures_lite`] reader.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails or if reading
    /// fails.
    #[cfg(feature = "futures-lite")]
    fn from_async_reader<R: futures_lite::AsyncRead + Unpin>(
        mut reader: R,
    ) -> impl Future<Output = Result<Self, DeserializeError<'static>>>
    where
        Self: Facet<'static>,
    {
        use futures_lite::AsyncReadExt;

        async move {
            from_async_coroutine(
                DeserializeIter::<false>::new::<Self>()?,
                async move |buf: &mut [u8]| reader.read_exact(buf).await.map_err(Into::into),
                Self::SIZE_HINT,
            )
            .await?
            .materialize::<Self>()
            .map_err(Into::into)
        }
    }

    /// Deserialize a value from a [`tokio`] reader.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails or if reading
    /// fails.
    #[cfg(feature = "tokio")]
    fn from_tokio_reader<R: tokio::io::AsyncRead + Unpin>(
        mut reader: R,
    ) -> impl Future<Output = Result<Self, DeserializeError<'static>>>
    where
        Self: Facet<'static>,
    {
        use tokio::io::AsyncReadExt;

        async move {
            from_async_coroutine(
                DeserializeIter::<false>::new::<Self>()?,
                async move |buf: &mut [u8]| {
                    reader.read_exact(buf).await.map_or_else(|err| Err(err.into()), |_| Ok(()))
                },
                Self::SIZE_HINT,
            )
            .await?
            .materialize::<Self>()
            .map_err(Into::into)
        }
    }
}

impl<'facet, T: Facet<'facet>> Deserialize<'facet> for T {
    const SIZE_HINT: TypeSizeHint = crate::hint::calculate_shape_hint(Self::SHAPE, None);
}

// -------------------------------------------------------------------------------------------------

/// A `no_std`-compatible cursor for deserialization.
pub struct InputCursor<'input, 'facet> {
    slice: &'input [u8],
    offset: usize,
    _invariant: core::marker::PhantomData<fn(&'facet ()) -> &'facet ()>,
}

impl<'input, 'facet> InputCursor<'input, 'facet> {
    /// Create a new [`InputCursor`] from a byte slice.
    #[must_use]
    pub const fn new(slice: &'input [u8]) -> Self {
        Self { slice, offset: 0, _invariant: PhantomData }
    }

    /// Get the remaining bytes in the cursor as a slice.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &'input [u8] { self.slice }

    /// Read bytes into the given buffer, advancing the cursor.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeIterError`] if there are not enough bytes
    /// remaining in the cursor.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), EndOfInput> {
        if self.offset + buf.len() > self.slice.len() {
            return Err(EndOfInput { had: self.slice.len() - self.offset, expected: buf.len() });
        }
        buf.copy_from_slice(&self.slice[self.offset..self.offset + buf.len()]);
        self.offset += buf.len();
        Ok(())
    }

    /// Take the next N bytes from the cursor, advancing the cursor.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeIterError`] if there are not enough bytes
    /// remaining in the cursor.
    pub fn take(&mut self, n: usize) -> Result<&'input [u8], EndOfInput> {
        if self.offset + n > self.slice.len() {
            return Err(EndOfInput { had: self.slice.len() - self.offset, expected: n });
        }
        let result = &self.slice[self.offset..self.offset + n];
        self.offset += n;
        Ok(result)
    }

    /// Take the next N bytes from the cursor, advancing the cursor.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeIterError`] if there are not enough bytes
    /// remaining in the cursor.
    pub fn take_array<const N: usize>(&mut self) -> Result<&'input [u8; N], EndOfInput> {
        if let Some((start, end)) = self.slice.split_first_chunk::<N>() {
            self.slice = end;
            Ok(start)
        } else {
            Err(EndOfInput { had: self.slice.len(), expected: N })
        }
    }

    /// Advance the cursor by N bytes.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeIterError`] if there are not enough bytes
    /// remaining in the cursor.
    pub fn consume(&mut self, n: usize) -> Result<(), EndOfInput> {
        if self.offset + n > self.slice.len() {
            Err(EndOfInput { had: self.slice.len() - self.offset, expected: n })
        } else {
            self.offset += n;
            Ok(())
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Read a variable-length integer from a buffer.
///
/// Returns the number of bytes read and the integer value.
///
/// # Errors
///
/// Returns a [`DeserializeIterError`] if there isn't enough data.
#[inline]
fn bytes_to_variable(_bytes: &[u8]) -> Result<(usize, u128), EndOfInput> { todo!() }

#[allow(clippy::cast_possible_truncation, reason = "Macro generated code")]
#[allow(clippy::cast_possible_wrap, reason = "Macro generated code")]
#[allow(trivial_numeric_casts, reason = "Macro generated code")]
fn borrowed_processor<'facet>(
    mut cursor: InputCursor<'facet, 'facet>,
) -> impl FnMut(PartialValue<'_, 'facet, true>) -> Result<(), DeserializeValueError> {
    macro_rules! take {
        ($val:expr, $ty:ty) => {{
            $val.set_value(<$ty>::from_le_bytes(
                *cursor.take_array::<{ core::mem::size_of::<$ty>() }>()?,
            ));
            Ok(())
        }};
        ($val:expr, $var:expr, $ty:ty) => {{
            if $var {
                take!($val, $ty)
            } else {
                let (consumed, value) = bytes_to_variable(cursor.as_slice())?;
                cursor.consume(consumed)?;
                $val.set_value(value as _);
                Ok(())
            }
        }};
    }

    move |partial| match partial {
        // TODO: Add a dedicated boolean error
        PartialValue::Bool(val) => {
            val.set_value(cursor.take_array::<1>()?[0].try_into().unwrap());
            Ok(())
        }
        PartialValue::U8(val) => take!(val, u8),
        PartialValue::U16(val, var) => take!(val, var, u16),
        PartialValue::U32(val, var) => take!(val, var, u32),
        PartialValue::U64(val, var) => take!(val, var, u64),
        PartialValue::U128(val, var) => take!(val, var, u128),
        PartialValue::I8(val) => take!(val, i8),
        PartialValue::I16(val, var) => take!(val, var, i16),
        PartialValue::I32(val, var) => take!(val, var, i32),
        PartialValue::I64(val, var) => take!(val, var, i64),
        PartialValue::I128(val, var) => take!(val, var, i128),
        PartialValue::F32(val) => take!(val, f32),
        PartialValue::F64(val) => take!(val, f64),
        PartialValue::Usize(val, var) => {
            if var {
                #[expect(clippy::cast_possible_truncation, reason = "Acceptable")]
                val.set_value(<u64>::from_le_bytes(
                    *cursor.take_array::<{ core::mem::size_of::<u64>() }>()?,
                ) as usize);
                Ok(())
            } else {
                let (consumed, value) = bytes_to_variable(cursor.as_slice())?;
                cursor.consume(consumed)?;
                val.set_value(value as _);
                Ok(())
            }
        }
        PartialValue::Isize(val, var) => {
            if var {
                #[expect(clippy::cast_possible_truncation, reason = "Acceptable")]
                val.set_value(<i64>::from_le_bytes(
                    *cursor.take_array::<{ core::mem::size_of::<i64>() }>()?,
                ) as isize);
                Ok(())
            } else {
                let (consumed, value) = bytes_to_variable(cursor.as_slice())?;
                cursor.consume(consumed)?;
                val.set_value(value as _);
                Ok(())
            }
        }
        PartialValue::Str(val) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            val.set_value(str::from_utf8(cursor.take(len as usize)?)?);
            Ok(())
        }
        PartialValue::String(val) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            val.set_value(str::from_utf8(cursor.take(len as usize)?)?.into());
            Ok(())
        }
        PartialValue::Bytes(val) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            val.set_value(cursor.take(len as usize)?);
            Ok(())
        }
        PartialValue::VecBytes(val) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            val.set_value(cursor.take(len as usize)?.into());
            Ok(())
        }
        PartialValue::Length(length) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            *length = Some(len as usize);
            Ok(())
        }
    }
}

#[allow(clippy::cast_possible_truncation, reason = "Macro generated code")]
#[allow(clippy::cast_possible_wrap, reason = "Macro generated code")]
#[allow(trivial_numeric_casts, reason = "Macro generated code")]
fn owned_processor(
    mut cursor: InputCursor<'_, 'static>,
) -> impl FnMut(PartialValue<'_, 'static, false>) -> Result<(), DeserializeValueError> {
    macro_rules! take {
        ($val:expr, $ty:ty) => {{
            $val.set_value(<$ty>::from_le_bytes(
                *cursor.take_array::<{ core::mem::size_of::<$ty>() }>()?,
            ));
            Ok(())
        }};
        ($val:expr, $var:expr, $ty:ty) => {
            if $var {
                take!($val, $ty)
            } else {
                let (consumed, value) = bytes_to_variable(cursor.as_slice())?;
                cursor.consume(consumed)?;
                $val.set_value(value as _);
                Ok(())
            }
        };
    }

    move |partial| match partial {
        PartialValue::Bool(val) => {
            // TODO: Add a dedicated boolean error
            val.set_value(cursor.take_array::<1>()?[0].try_into().unwrap());
            Ok(())
        }
        PartialValue::U8(val) => take!(val, u8),
        PartialValue::U16(val, var) => take!(val, var, u16),
        PartialValue::U32(val, var) => take!(val, var, u32),
        PartialValue::U64(val, var) => take!(val, var, u64),
        PartialValue::U128(val, var) => take!(val, var, u128),
        PartialValue::I8(val) => take!(val, i8),
        PartialValue::I16(val, var) => take!(val, var, i16),
        PartialValue::I32(val, var) => take!(val, var, i32),
        PartialValue::I64(val, var) => take!(val, var, i64),
        PartialValue::I128(val, var) => take!(val, var, i128),
        PartialValue::F32(val) => take!(val, f32),
        PartialValue::F64(val) => take!(val, f64),
        PartialValue::Usize(val, var) => {
            if var {
                #[expect(clippy::cast_possible_truncation, reason = "Acceptable")]
                val.set_value(<u64>::from_le_bytes(
                    *cursor.take_array::<{ core::mem::size_of::<u64>() }>()?,
                ) as usize);
                Ok(())
            } else {
                let (consumed, value) = bytes_to_variable(cursor.as_slice())?;
                cursor.consume(consumed)?;
                val.set_value(value as _);
                Ok(())
            }
        }
        PartialValue::Isize(val, var) => {
            if var {
                #[expect(clippy::cast_possible_truncation, reason = "Acceptable")]
                val.set_value(<i64>::from_le_bytes(
                    *cursor.take_array::<{ core::mem::size_of::<i64>() }>()?,
                ) as isize);
                Ok(())
            } else {
                let (consumed, value) = bytes_to_variable(cursor.as_slice())?;
                cursor.consume(consumed)?;
                val.set_value(value as _);
                Ok(())
            }
        }
        PartialValue::String(val) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            val.set_value(str::from_utf8(cursor.take(len as usize)?)?.into());
            Ok(())
        }
        PartialValue::VecBytes(val) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            val.set_value(cursor.take(len as usize)?.into());
            Ok(())
        }
        PartialValue::Length(length) => {
            let (consumed, len) = bytes_to_variable(cursor.as_slice())?;
            cursor.consume(consumed)?;
            *length = Some(len as usize);
            Ok(())
        }
        // Cannot borrow strings or bytes when deserializing owned values
        PartialValue::Str(_) | PartialValue::Bytes(_) => Err(DeserializeValueError::StaticBorrow),
    }
}

/// A helper function to drive the deserialization process
/// using a synchronous reader.
#[cfg(feature = "std")]
fn from_coroutine<F: FnMut(&mut [u8]) -> Result<(), DeserializeError<'static>>>(
    mut iter: DeserializeIter<'static, false>,
    mut reader: F,
    hint: TypeSizeHint,
) -> Result<HeapValue<'static, false>, DeserializeError<'static>> {
    let mut buffer = Vec::<u8>::with_capacity(hint.minimum().unwrap_or_default());
    (reader)(&mut buffer)?;

    let mut processor = owned_processor(InputCursor::new(buffer.as_slice()));
    loop {
        match iter.next(&mut processor) {
            Ok((iterator, false)) => iter = iterator,
            Ok((iterator, true)) => return Ok(iterator.into_partial().build()?),
            Err(err) => {
                match err {
                    crate::deserialize::error::DeserializeIterError::EndOfInput {
                        error: crate::deserialize::error::EndOfInput { had, expected },
                        iterator,
                    } => {
                        // Drop the old processor to release the buffer
                        drop(processor);
                        // Move the remaining data to the start of the buffer and resize it
                        let src = buffer.len() - had;
                        buffer.copy_within(src.., 0);
                        buffer.resize(had + expected, 0);
                        // Read new data into the buffer
                        (reader)(&mut buffer[had..])?;
                        // Create a new processor using the refilled buffer
                        processor = owned_processor(InputCursor::new(buffer.as_slice()));
                        // Replace the old iterator and resume deserialization
                        iter = iterator;
                    }
                    other => return Err(other.into()),
                }
            }
        }
    }
}

/// A helper function to drive the deserialization process
/// using an asynchronous reader.
#[cfg(any(feature = "futures-lite", feature = "tokio"))]
async fn from_async_coroutine<F: AsyncFnMut(&mut [u8]) -> Result<(), DeserializeError<'static>>>(
    mut iter: DeserializeIter<'static, false>,
    mut reader: F,
    hint: TypeSizeHint,
) -> Result<HeapValue<'static, false>, DeserializeError<'static>> {
    let mut buffer = Vec::<u8>::with_capacity(hint.minimum().unwrap_or_default());
    (reader)(&mut buffer).await?;

    let mut processor = owned_processor(InputCursor::new(buffer.as_slice()));
    loop {
        match iter.next(&mut processor) {
            Ok((iterator, false)) => iter = iterator,
            Ok((iterator, true)) => return Ok(iterator.into_partial().build()?),
            Err(err) => {
                match err {
                    crate::deserialize::error::DeserializeIterError::EndOfInput {
                        error: crate::deserialize::error::EndOfInput { had, expected },
                        iterator,
                    } => {
                        // Drop the old processor to release the buffer
                        drop(processor);
                        // Move the remaining data to the start of the buffer and resize it
                        let src = buffer.len() - had;
                        buffer.copy_within(src.., 0);
                        buffer.resize(had + expected, 0);
                        // Read new data into the buffer
                        (reader)(&mut buffer[had..]).await?;
                        // Create a new processor using the refilled buffer
                        processor = owned_processor(InputCursor::new(buffer.as_slice()));
                        // Replace the old iterator and resume deserialization
                        iter = iterator;
                    }
                    other => return Err(other.into()),
                }
            }
        }
    }
}
