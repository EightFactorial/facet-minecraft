//! TODO

use alloc::vec::Vec;

use error::SerializeError;
use facet::Facet;
use facet_reflect::Peek;
use smallvec::SmallVec;

use crate::{
    hint::TypeSizeHint,
    serialize::{
        buffer::SerializeBuffer,
        iter::{PeekValue, SerializeIter},
    },
};

pub mod buffer;
pub mod error;
pub mod fns;
pub mod iter;

/// A trait for types that can be serialized.
pub trait Serialize<'facet>: Facet<'facet> {
    /// The [`TypeSizeHint`] for this type.
    const SIZE_HINT: &'static TypeSizeHint = &crate::hint::calculate_shape_hint(Self::SHAPE, None);

    /// Serialize this value into a [`Vec<u8>`].
    ///
    /// # Errors
    ///
    /// Returns a [`SerializeError`] if serialization fails.
    #[inline]
    fn to_vec(&self) -> Result<Vec<u8>, SerializeError<'_, 'facet>> {
        let capacity = Self::SIZE_HINT.maximum().or(Self::SIZE_HINT.minimum()).unwrap_or_default();
        let mut buffer = Vec::with_capacity(capacity);
        to_buffer_inner(Peek::new(self), &mut buffer)?;
        Ok(buffer)
    }

    /// Serialize this value into a buffer.
    ///
    /// # Errors
    ///
    /// Returns a [`SerializeError`] if serialization fails or
    /// if writing to the buffer fails.
    #[inline]
    fn to_buffer<B: SerializeBuffer>(
        &self,
        buffer: &mut B,
    ) -> Result<(), SerializeError<'_, 'facet>> {
        to_buffer_inner(Peek::new(self), buffer)
    }

    /// Serialize this value into a writer.
    ///
    /// # Errors
    ///
    /// Returns a [`SerializeError`] if serialization fails or if writing fails.
    #[inline]
    #[cfg(feature = "std")]
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        to_writer_inner(Peek::new(self), writer)
    }

    /// Serialize this value into an [`futures_lite`] writer.
    ///
    /// # Errors
    ///
    /// Returns a [`SerializeError`] if serialization fails or if writing fails.
    #[inline]
    #[cfg(feature = "futures-lite")]
    fn to_async_writer<'a, W: futures_lite::AsyncWrite + Unpin>(
        &'a self,
        writer: &'a mut W,
    ) -> impl Future<Output = Result<(), std::io::Error>> + 'a
    where
        'facet: 'a,
    {
        to_async_writer_inner(Peek::new(self), writer)
    }

    /// Serialize this value into a [`tokio`] writer.
    ///
    /// # Errors
    ///
    /// Returns a [`SerializeError`] if serialization fails or if writing fails.
    #[inline]
    #[cfg(feature = "tokio")]
    fn to_tokio_writer<'a, W: tokio::io::AsyncWrite + Unpin>(
        &'a self,
        writer: &'a mut W,
    ) -> impl Future<Output = Result<(), std::io::Error>> + 'a
    where
        'facet: 'a,
    {
        to_tokio_writer_inner(Peek::new(self), writer)
    }
}

impl<'facet, T: Facet<'facet> + ?Sized> Serialize<'facet> for T {}

// -------------------------------------------------------------------------------------------------

/// Collect the values from a [`Peek`] into a [`SmallVec`].
#[inline]
fn parse_peek<'mem, 'facet>(
    peek: Peek<'mem, 'facet>,
) -> Result<SmallVec<[PeekValue<'mem, 'facet>; 8]>, SerializeError<'mem, 'facet>> {
    let mut values = SmallVec::<[_; 8]>::new_const();
    let mut iter = SerializeIter::new_from_peek(peek)?;

    loop {
        match iter.next() {
            Some(Ok(instruction)) => values.push(instruction),
            Some(Err(err)) => return Err(SerializeError::from(err)),
            None => break,
        }
    }

    Ok(values)
}

/// Write a variable-length integer to a buffer.
///
/// Returns the length of the written data.
#[inline]
fn variable_to_bytes(mut val: u128, buffer: &mut [u8; 19]) -> usize {
    let mut byte;
    let mut index = 0;
    while (val != 0 || index == 0) && index < 19 {
        byte = (val & 0b0111_1111) as u8;
        val = (val >> 7) & (u128::MAX >> 6);
        if val != 0 {
            byte |= 0b1000_0000;
        }
        buffer[index] = byte;
        index += 1;
    }
    index
}

/// To prevent code duplication,
/// this function actually performs the serialization.
fn to_buffer_inner<'mem, 'facet, B: SerializeBuffer>(
    peek: Peek<'mem, 'facet>,
    buffer: &mut B,
) -> Result<(), SerializeError<'mem, 'facet>> {
    macro_rules! wrap {
        ($expr:expr) => {
            $expr.then_some(()).ok_or_else(SerializeError::new)?
        };
    }

    let mut variable = [0u8; _];
    let values = parse_peek(peek)?;
    for val in values {
        match val {
            PeekValue::Unit(()) => {}
            PeekValue::Bool(val) => wrap!(buffer.write_data(&[u8::from(val)])),
            PeekValue::U8(val) => wrap!(buffer.write_data(&[val])),
            PeekValue::U16(val) => wrap!(buffer.write_data(&val.to_be_bytes())),
            PeekValue::U32(val) => wrap!(buffer.write_data(&val.to_be_bytes())),
            PeekValue::U64(val) => wrap!(buffer.write_data(&val.to_be_bytes())),
            PeekValue::U128(val) => wrap!(buffer.write_data(&val.to_be_bytes())),
            PeekValue::F32(val) => wrap!(buffer.write_data(&val.to_be_bytes())),
            PeekValue::F64(val) => wrap!(buffer.write_data(&val.to_be_bytes())),
            PeekValue::Bytes(items) => wrap!(buffer.write_data(items.as_ref())),
            PeekValue::Variable(val) => {
                let length = variable_to_bytes(val, &mut variable);
                wrap!(buffer.write_data(&variable[..length]));
            }
            PeekValue::Custom(peek, serialize) => serialize.call(peek, buffer)?,
        }
    }

    Ok(())
}

/// To prevent code duplication,
/// this function actually performs the serialization.
#[cfg(feature = "std")]
fn to_writer_inner<W: std::io::Write>(
    peek: Peek<'_, '_>,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    #[allow(unused_imports, reason = "Required")]
    use std::io::Write;

    let mut variable = [0u8; _];
    let values = parse_peek(peek).map_err(std::io::Error::other)?;
    for val in values {
        match val {
            PeekValue::Unit(()) => {}
            PeekValue::Bool(val) => writer.write_all(&[u8::from(val)])?,
            PeekValue::U8(val) => writer.write_all(&[val])?,
            PeekValue::U16(val) => writer.write_all(&val.to_be_bytes())?,
            PeekValue::U32(val) => writer.write_all(&val.to_be_bytes())?,
            PeekValue::U64(val) => writer.write_all(&val.to_be_bytes())?,
            PeekValue::U128(val) => writer.write_all(&val.to_be_bytes())?,
            PeekValue::F32(val) => writer.write_all(&val.to_be_bytes())?,
            PeekValue::F64(val) => writer.write_all(&val.to_be_bytes())?,
            PeekValue::Bytes(items) => writer.write_all(items.as_ref())?,
            PeekValue::Variable(val) => {
                let length = variable_to_bytes(val, &mut variable);
                writer.write_all(&variable[..length])?;
            }
            PeekValue::Custom(peek, serialize) => {
                serialize.call(peek, writer).map_err(std::io::Error::other)?;
            }
        }
    }

    Ok(())
}

/// To prevent code duplication,
/// this function actually performs the serialization.
#[cfg(feature = "futures-lite")]
async fn to_async_writer_inner<W: futures_lite::AsyncWrite + Unpin>(
    peek: Peek<'_, '_>,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    use futures_lite::AsyncWriteExt;

    let mut variable = [0u8; _];
    let values = parse_peek(peek).map_err(std::io::Error::other)?;
    for val in values {
        match val {
            PeekValue::Unit(()) => {}
            PeekValue::Bool(val) => writer.write_all(&[u8::from(val)]).await?,
            PeekValue::U8(val) => writer.write_all(&[val]).await?,
            PeekValue::U16(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::U32(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::U64(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::U128(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::F32(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::F64(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::Bytes(items) => writer.write_all(items.as_ref()).await?,
            PeekValue::Variable(val) => {
                let length = variable_to_bytes(val, &mut variable);
                writer.write_all(&variable[..length]).await?;
            }
            PeekValue::Custom(peek, serialize) => serialize
                .call(peek, &mut crate::serialize::buffer::FuturesLite(&mut *writer))
                .map_err(std::io::Error::other)?,
        }
    }

    Ok(())
}

/// To prevent code duplication,
/// this function actually performs the serialization.
#[cfg(feature = "tokio")]
async fn to_tokio_writer_inner<W: tokio::io::AsyncWrite + Unpin>(
    peek: Peek<'_, '_>,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    use tokio::io::AsyncWriteExt;

    let mut variable = [0u8; _];
    let values = parse_peek(peek).map_err(std::io::Error::other)?;
    for val in values {
        match val {
            PeekValue::Unit(()) => {}
            PeekValue::Bool(val) => writer.write_all(&[u8::from(val)]).await?,
            PeekValue::U8(val) => writer.write_all(&[val]).await?,
            PeekValue::U16(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::U32(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::U64(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::U128(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::F32(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::F64(val) => writer.write_all(&val.to_be_bytes()).await?,
            PeekValue::Bytes(items) => writer.write_all(items.as_ref()).await?,
            PeekValue::Variable(val) => {
                let length = variable_to_bytes(val, &mut variable);
                writer.write_all(&variable[..length]).await?;
            }
            PeekValue::Custom(peek, serialize) => serialize
                .call(peek, &mut crate::serialize::buffer::Tokio(&mut *writer))
                .map_err(std::io::Error::other)?,
        }
    }

    Ok(())
}
