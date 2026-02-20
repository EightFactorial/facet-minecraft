//! TODO
#![allow(unpredictable_function_pointer_comparisons, reason = "Shouldn't be compared like that")]

use facet::Facet;
use facet_reflect::Peek;

use crate::serialize::{
    Serialize,
    buffer::{SerializeBuffer, SerializeWriter},
    error::SerializeError,
};

type PtrType = for<'mem, 'facet, 'writer> fn(
    Peek<'mem, 'facet>,
    &'writer mut (dyn SerializeWriter + 'writer),
) -> Result<(), SerializeError<'mem, 'facet>>;

/// A custom serializer function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
#[facet(opaque)]
pub struct SerializeFn {
    ptr: PtrType,
}

impl SerializeFn {
    /// Creates a new [`SerializeFn`].
    #[inline]
    #[must_use]
    pub const fn new(ptr: PtrType) -> Self { Self { ptr } }

    /// Call the serializer function.
    ///
    /// # Errors
    ///
    /// Returns a [`SerializeError`] if serialization fails or
    /// if writing to the buffer fails.
    #[inline]
    pub fn call<'mem, 'facet, 'writer>(
        &self,
        peek: Peek<'mem, 'facet>,
        buffer: &'writer mut (dyn SerializeWriter + 'writer),
    ) -> Result<(), SerializeError<'mem, 'facet>> {
        (self.ptr)(peek, buffer)
    }
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value into a [`Vec`].
///
/// # Errors
///
/// Returns a [`SerializeError`] if serialization fails.
#[inline]
pub fn to_vec<'mem, 'facet, T: Serialize<'facet>>(
    value: &'mem T,
) -> Result<alloc::vec::Vec<u8>, SerializeError<'mem, 'facet>> {
    Serialize::to_vec(value)
}

/// Serialize a value into a buffer.
///
/// # Errors
///
/// Returns a [`SerializeError`] if serialization fails or
/// if writing to the buffer fails.
#[inline]
pub fn to_buffer<'mem, 'facet, T: Serialize<'facet>, B: SerializeBuffer>(
    value: &'mem T,
    buffer: &mut B,
) -> Result<(), SerializeError<'mem, 'facet>> {
    Serialize::to_buffer(value, buffer)
}

/// Serialize a value into a writer.
///
/// # Errors
///
/// Returns a [`SerializeError`] if serialization fails or if writing fails.
#[inline]
#[cfg(feature = "std")]
pub fn to_writer<'mem, 'facet, T: Serialize<'facet>, W: std::io::Write>(
    value: &'mem T,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    Serialize::to_writer(value, writer)
}

/// Serialize a value into an [`futures_lite`] writer.
///
/// # Errors
///
/// Returns a [`SerializeError`] if serialization fails or if writing fails.
#[inline]
#[cfg(feature = "futures-lite")]
pub async fn to_async_writer<
    'mem,
    'facet,
    T: Serialize<'facet>,
    W: futures_lite::AsyncWrite + Unpin,
>(
    value: &'mem T,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    Serialize::to_async_writer(value, writer).await
}

/// Serialize a value into a [`tokio`] writer.
///
/// # Errors
///
/// Returns a [`SerializeError`] if serialization fails or if writing fails.
#[inline]
#[cfg(feature = "tokio")]
pub async fn to_tokio_writer<
    'mem,
    'facet,
    T: Serialize<'facet>,
    W: tokio::io::AsyncWrite + Unpin,
>(
    value: &'mem T,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    Serialize::to_tokio_writer(value, writer).await
}
