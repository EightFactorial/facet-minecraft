//! TODO
#![allow(unpredictable_function_pointer_comparisons, reason = "Shouldn't be compared like that")]
#![expect(clippy::result_unit_err, reason = "Temporary")]

use facet::{Facet, Partial};

use crate::{Deserialize, deserialize::error::DeserializeError};

type PtrType = for<'facet> fn(Partial<'facet, false>) -> Result<(), DeserializeError<'facet>>;

/// A custom deserializer function.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct DeserializeFn {
    ptr: PtrType,
}

impl DeserializeFn {
    /// Creates a new [`DeserializeFn`].
    #[inline]
    #[must_use]
    pub const fn new(ptr: PtrType) -> Self { Self { ptr } }

    /// Call the deserializer function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    #[inline]
    pub fn call<'facet>(
        &self,
        partial: Partial<'facet, false>,
    ) -> Result<(), DeserializeError<'facet>> {
        (self.ptr)(partial)
    }
}

// -------------------------------------------------------------------------------------------------

/// Deserialize a value from a [`slice`](::core::primitive::slice),
/// borrowing data where possible.
///
/// If the type will outlive the input data, use
/// [`from_slice_owned`] instead.
///
/// # Errors
///
/// Returns a [`DeserializeError`] if deserialization fails.
#[inline]
pub fn from_slice<'facet, T: Facet<'facet>>(
    slice: &'facet [u8],
) -> Result<T, DeserializeError<'facet>> {
    Deserialize::from_slice(slice)
}

/// Deserialize a value from a [`slice`](::core::primitive::slice).
///
/// # Errors
///
/// Returns a [`DeserializeError`] if deserialization fails.
#[inline]
pub fn from_slice_owned<T: Facet<'static>>(slice: &[u8]) -> Result<T, DeserializeError<'static>> {
    Deserialize::from_slice_owned(slice)
}

/// Deserialize a value from a [`slice`](::core::primitive::slice),
/// returning any remaining data.
///
/// # Errors
///
/// Returns a [`DeserializeError`] if deserialization fails.
#[inline]
pub fn from_slice_remainder<'a, T: Facet<'static>>(
    slice: &'a [u8],
) -> Result<(T, &'a [u8]), DeserializeError<'static>> {
    Deserialize::from_slice_remainder(slice)
}

/// Deserialize a value from a reader.
///
/// # Errors
///
/// Returns a [`DeserializeError`] if deserialization fails or if reading
/// fails.
#[inline]
#[cfg(feature = "std")]
pub fn from_reader<T: Facet<'static>, R: std::io::Read>(
    reader: R,
) -> Result<T, DeserializeError<'static>> {
    Deserialize::from_reader(reader)
}

/// Deserialize a value from a [`futures_lite`] reader.
///
/// # Errors
/// Returns a [`DeserializeError`] if deserialization fails or if reading
/// fails.
#[inline]
#[cfg(feature = "futures-lite")]
pub async fn from_async_reader<T: Facet<'static>, R: futures_lite::AsyncRead + Unpin>(
    reader: R,
) -> Result<T, DeserializeError<'static>> {
    <T as Deserialize>::from_async_reader(reader).await
}

/// Deserialize a value from a [`tokio`] reader.
///
/// # Errors
/// Returns a [`DeserializeError`] if deserialization fails or if reading
/// fails.
#[inline]
#[cfg(feature = "tokio")]
pub async fn from_tokio_reader<T: Facet<'static>, R: tokio::io::AsyncRead + Unpin>(
    reader: R,
) -> Result<T, DeserializeError<'static>> {
    <T as Deserialize>::from_tokio_reader(reader).await
}
