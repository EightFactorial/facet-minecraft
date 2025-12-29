//! TODO

use alloc::vec::Vec;

use facet::{Facet, Shape};
use facet_format::{FieldOrdering, FormatSerializer, ScalarValue, SerializeError as FSError};
use facet_reflect::{FieldItem, Peek};

mod buffer;
pub use buffer::SerializeBuffer;

mod error;
pub use error::{SerializeError, SerializeErrorKind};

pub(crate) mod r#trait;
pub use r#trait::Serializable;

/// A function pointer to a serialization function.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct SerializeFn {
    ptr: for<'buffer> fn(
        &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
    ) -> Result<(), SerializeError>,
}

impl SerializeFn {
    /// Create a new [`SerializeFn`].
    #[inline]
    #[must_use]
    pub const fn new(
        ptr: for<'buffer> fn(
            &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
        ) -> Result<(), SerializeError>,
    ) -> Self {
        Self { ptr }
    }

    /// Call the serialization function.
    ///
    /// # Errors
    ///
    /// This function will return an error if serialization fails.
    #[inline]
    pub fn call<'buffer>(
        &self,
        serializer: &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
    ) -> Result<(), SerializeError> {
        (self.ptr)(serializer)
    }
}

impl
    From<
        for<'buffer> fn(
            &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
        ) -> Result<(), SerializeError>,
    > for SerializeFn
{
    #[inline]
    fn from(
        ptr: for<'buffer> fn(
            &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
        ) -> Result<(), SerializeError>,
    ) -> Self {
        Self::new(ptr)
    }
}

// -------------------------------------------------------------------------------------------------

/// A serializer that implements [`FormatSerializer`].
pub struct McSerializer<'buffer, B: SerializeBuffer + ?Sized> {
    buffer: &'buffer mut B,
}

impl<'buffer, B: SerializeBuffer + ?Sized> McSerializer<'buffer, B> {
    /// Create a new [`McSerializer`].
    #[inline]
    #[must_use]
    pub const fn new(buffer: &'buffer mut B) -> Self { Self { buffer } }

    /// Consume the serializer and return the buffer reference.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> &'buffer mut B { self.buffer }

    /// Create a [`McSerializer`] over a
    /// [`dyn SerializeBuffer`](SerializeBuffer) from this serializer.
    #[inline]
    #[must_use]
    pub const fn as_dyn<'a: 'buffer>(&'a mut self) -> McSerializer<'a, dyn SerializeBuffer + 'a>
    where
        B: Sized + 'a,
    {
        McSerializer { buffer: self.buffer }
    }
}

impl<B: SerializeBuffer + ?Sized> FormatSerializer for McSerializer<'_, B> {
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
    value: &T,
) -> Result<Vec<u8>, FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    let mut buffer = T::SERIALIZE_HINT
        .maximum()
        .or(T::SERIALIZE_HINT.minimum())
        .map_or_else(Vec::new, Vec::with_capacity);
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
pub fn to_buffer<'output, 'facet, T: Serializable<'facet> + ?Sized, B: SerializeBuffer + ?Sized>(
    value: &T,
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
    value: &T,
    writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not
    // serializable!") };

    std::io::Write::write_all(writer, to_vec::<T>(value)?.as_slice())
        .map_err(|err| FSError::Backend(SerializeError::from(err)))
}

/// Serialize a value of type `T` into an asynchronous
/// [`AsyncWrite`](futures_lite::AsyncWrite).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "futures-lite")]
pub async fn to_async_writer<
    'facet,
    T: Serializable<'facet> + ?Sized,
    W: futures_lite::AsyncWrite + Unpin,
>(
    value: &T,
    writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not
    // serializable!") };

    futures_lite::AsyncWriteExt::write_all(writer, to_vec::<T>(value)?.as_slice())
        .await
        .map_err(|err| FSError::Backend(SerializeError::from(err)))
}

/// Serialize a value of type `T` into an asynchronous
/// [`AsyncWrite`](tokio::io::AsyncWrite).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "tokio")]
pub async fn to_tokio_writer<
    'facet,
    T: Serializable<'facet> + ?Sized,
    W: tokio::io::AsyncWrite + Unpin,
>(
    value: &T,
    writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    tokio::io::AsyncWriteExt::write_all(writer, to_vec::<T>(value)?.as_slice())
        .await
        .map_err(|err| FSError::Backend(SerializeError::from(err)))
}
