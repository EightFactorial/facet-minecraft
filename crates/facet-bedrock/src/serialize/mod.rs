//! TODO

use alloc::vec::Vec;

use facet::Facet;
use facet_format::{FormatSerializer, ScalarValue, SerializeError as FSError};
use facet_reflect::Peek;

mod error;
pub use error::{SerializeError, SerializeErrorKind};

mod r#trait;
pub use r#trait::{Serializable, TypeSerializable};

/// A function pointer to a serialization function.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct SerializeFn {
    ptr: fn(),
}

impl SerializeFn {
    /// Create a new [`SerializeFn`].
    #[inline]
    #[must_use]
    pub const fn new(ptr: fn()) -> Self { Self { ptr } }

    /// Call the serialization function.
    #[inline]
    pub fn call(&self) { (self.ptr)() }
}

// -------------------------------------------------------------------------------------------------

/// A serializer that implements [`FormatSerializer`].
#[derive(Default)]
pub struct McSerializer {}

impl McSerializer {
    /// Create a new [`McSerializer`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self {} }
}

impl FormatSerializer for McSerializer {
    type Error = SerializeError;

    fn begin_struct(&mut self) -> Result<(), Self::Error> { todo!() }

    fn field_key(&mut self, _key: &str) -> Result<(), Self::Error> { todo!() }

    fn end_struct(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_seq(&mut self) -> Result<(), Self::Error> { todo!() }

    fn end_seq(&mut self) -> Result<(), Self::Error> { todo!() }

    fn scalar(&mut self, _val: ScalarValue<'_>) -> Result<(), Self::Error> { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value of type `T` into a byte vector.
///
/// # Errors
///
/// This function will return an error if serialization fails.
pub fn to_vec<'facet, T: Serializable<'facet>>(
    value: &'facet T,
) -> Result<Vec<u8>, FSError<SerializeError>> {
    let mut buffer = Vec::new();
    to_buffer(value, &mut buffer)?;
    Ok(buffer)
}

/// Serialize a value of type `T` into a buffer,
/// returning a slice containing the serialized data.
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or if the buffer is too small and cannot allocate.
pub fn to_buffer<'output, 'facet, T: Serializable<'facet>, B>(
    value: &'facet T,
    _buffer: &'output mut B,
) -> Result<&'output [u8], FSError<SerializeError>> {
    let mut format = McSerializer::new();
    facet_format::serialize_root(&mut format, Peek::new(value))?;
    todo!()
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value of type `T` into a [`Writer`](std::io::Write).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "streaming")]
pub fn to_writer<W: std::io::Write, T: Serializable<'static>>(
    _value: &T,
    _writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    todo!()
}

/// Serialize a value of type `T` into an asynchronous
/// [`AsyncWrite`](futures_io::AsyncWrite).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "futures-lite")]
pub async fn to_async_writer<W: futures_lite::AsyncWrite, T: Serializable<'static>>(
    _value: &T,
    _writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
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
pub async fn to_tokio_writer<W: tokio::io::AsyncWrite, T: Serializable<'static>>(
    _value: &T,
    _writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    todo!()
}
