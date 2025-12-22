//! TODO: Make `Deserializable::DESERIALIZABLE.possible()` a trait bound.
//!
//! This would force all types to support deserialization at compile time,
//! preventing usage of non-deserializable types.

use facet::{Facet, Shape};
use facet_format::DeserializeError as FDError;

use crate::{
    common::{TypeSerializeHint, TypeSerializeResult, calculate_shape_hint},
    deserialize::{self, DeserializeError},
};

/// A trait for types that can be deserialized.
pub trait Deserializable<'facet>: Facet<'facet> + Sized {
    /// The [`TypeSerializeResult`] result for this type.
    const DESERIALIZABLE: TypeSerializeResult = calculate_shape_serialize(Self::SHAPE);
    /// A hint for the size of this type before deserialization.
    const DESERIALIZE_HINT: TypeSerializeHint = calculate_shape_hint(Self::SHAPE, None);

    /// Deserialize a value from a byte slice and returning any
    /// remaining bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if deserialization fails.
    #[inline]
    fn from_slice(input: &[u8]) -> Result<(Self, &[u8]), FDError<DeserializeError>>
    where
        'facet: 'static,
    {
        deserialize::from_slice::<Self>(input)
    }

    /// Deserialize a value from a byte slice and returning any
    /// remaining bytes, allowing zero-copy borrowing.
    ///
    /// This variant requires the input to outlive the result (`'input:
    /// 'facet`), enabling zero-copy deserialization of string fields as
    /// `&str` or `Cow<str>`.
    ///
    /// Use this when you need maximum performance and can guarantee the input
    /// buffer outlives the deserialized value. For most use cases, prefer
    /// [`Deserializable::from_slice`] which doesn't have lifetime requirements.
    ///
    /// # Errors
    ///
    /// This function will return an error if deserialization fails.
    #[inline]
    fn from_slice_borrowed<'input: 'facet>(
        input: &'input [u8],
    ) -> Result<(Self, &'input [u8]), FDError<DeserializeError>> {
        deserialize::from_slice_borrowed::<Self>(input)
    }

    /// Deserialize a value of type `T` from a [`Reader`](std::io::Read).
    ///
    /// # Errors
    ///
    /// This function will return an error if deserialization fails,
    /// or the reader encounters an I/O error.
    #[inline]
    #[cfg(feature = "streaming")]
    fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self, FDError<DeserializeError>> {
        deserialize::from_reader::<Self, R>(reader)
    }

    /// Deserialize a value of type `T` from an asynchronous
    /// [`AsyncRead`](futures_io::AsyncRead).
    ///
    /// # Errors
    ///
    /// This function will return an error if deserialization fails,
    /// or the reader encounters an I/O error.
    #[inline]
    #[cfg(feature = "futures-io")]
    fn from_async_reader<R: futures_io::AsyncRead>(
        reader: &mut R,
    ) -> impl Future<Output = Result<Self, FDError<DeserializeError>>> {
        deserialize::from_async_reader::<Self, R>(reader)
    }

    /// Deserialize a value of type `T` from an asynchronous
    /// [`AsyncRead`](tokio::io::AsyncRead).
    ///
    /// # Errors
    ///
    /// This function will return an error if deserialization fails,
    /// or the reader encounters an I/O error.
    #[inline]
    #[cfg(feature = "tokio")]
    fn from_tokio_reader<R: tokio::io::AsyncRead>(
        reader: &mut R,
    ) -> impl Future<Output = Result<Self, FDError<DeserializeError>>> {
        deserialize::from_tokio_reader::<Self, R>(reader)
    }
}

impl<'facet, T: Facet<'facet>> Deserializable<'facet> for T {}

// -------------------------------------------------------------------------------------------------

/// A helper function to calculate the [`TypeSerializeResult`] for a [`Shape`].
const fn calculate_shape_serialize(_shape: &'static Shape) -> TypeSerializeResult { todo!() }
