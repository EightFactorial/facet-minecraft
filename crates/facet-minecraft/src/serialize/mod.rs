//! TODO

use alloc::vec::Vec;

use facet::Shape;
use facet_format::{FieldOrdering, FormatSerializer, ScalarValue, SerializeError as FSError};
use facet_reflect::{FieldItem, Peek};

mod buffer;
pub use buffer::SerializeBuffer;

mod error;
pub use error::{SerializeError, SerializeErrorKind};

pub(crate) mod r#trait;
pub use r#trait::Serializable;

/// A function pointer to a serialization function.
pub type SerializeFn = fn();

/// A serializer that implements [`FormatSerializer`].
pub struct McSerializer<'buffer, B: SerializeBuffer> {
    buffer: &'buffer mut B,
}

impl<'buffer, B: SerializeBuffer> McSerializer<'buffer, B> {
    /// Create a new [`McSerializer`].
    #[inline]
    #[must_use]
    pub const fn new(buffer: &'buffer mut B) -> Self { Self { buffer } }

    /// Consume the serializer and return the buffer reference.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> &'buffer mut B { self.buffer }
}

impl<B: SerializeBuffer> FormatSerializer for McSerializer<'_, B> {
    type Error = SerializeError;

    fn begin_struct(&mut self) -> Result<(), Self::Error> { todo!() }

    fn field_key(&mut self, _key: &str) -> Result<(), Self::Error> { todo!() }

    fn end_struct(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_seq(&mut self) -> Result<(), Self::Error> { todo!() }

    fn end_seq(&mut self) -> Result<(), Self::Error> { todo!() }

    fn scalar(&mut self, _val: ScalarValue<'_>) -> Result<(), Self::Error> { todo!() }

    fn field_metadata(&mut self, _field: &FieldItem) -> Result<(), Self::Error> { todo!() }

    fn struct_metadata(&mut self, _shape: &Shape) -> Result<(), Self::Error> { todo!() }

    fn preferred_field_order(&self) -> FieldOrdering { FieldOrdering::Declaration }
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value of type `T` into a byte vector.
///
/// # Errors
///
/// This function will return an error if serialization fails.
pub fn to_vec<'facet, T: Serializable<'facet> + ?Sized>(
    value: &'facet T,
) -> Result<Vec<u8>, FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    let mut buffer = T::SERIALIZE_HINT.minimum().map_or_else(Vec::new, Vec::with_capacity);
    to_buffer::<T, Vec<u8>>(value, &mut buffer)?;
    Ok(buffer)
}

/// Serialize a value of type `T` into a buffer,
/// returning a slice containing the serialized data.
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or if the buffer cannot be written to.
pub fn to_buffer<'output, 'facet, T: Serializable<'facet> + ?Sized, B: SerializeBuffer>(
    value: &'facet T,
    buffer: &'output mut B,
) -> Result<&'output [u8], FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    let mut format = McSerializer::new(buffer);
    facet_format::serialize_root(&mut format, Peek::new(value))?;
    Ok(buffer.get_content())
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value of type `T` into a [`Writer`](std::io::Write).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "streaming")]
pub fn to_writer<'facet, T: Serializable<'facet> + ?Sized, W: std::io::Write>(
    _value: &T,
    _writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    todo!()
}

/// Serialize a value of type `T` into an asynchronous
/// [`AsyncWrite`](futures_io::AsyncWrite).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "futures-io")]
pub async fn to_async_writer<
    'facet,
    T: Serializable<'facet> + ?Sized,
    W: futures_io::AsyncWrite,
>(
    _value: &T,
    _writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    todo!()
}

/// Serialize a value of type `T` into an asynchronous
/// [`AsyncWrite`](tokio::io::AsyncWrite).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "tokio")]
pub async fn to_tokio_writer<'facet, T: Serializable<'facet> + ?Sized, W: tokio::io::AsyncWrite>(
    _value: &T,
    _writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    todo!()
}
