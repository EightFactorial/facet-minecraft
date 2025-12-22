//! TODO: Make `Serializable::SERIALIZABLE.possible()` a trait bound.
//!
//! This would force all types to support serialization at compile time,
//! preventing usage of non-serializable types.

use facet::{Facet, Shape};
use facet_format::SerializeError as FSError;

use crate::{
    common::{TypeSerializeHint, TypeSerializeResult, calculate_shape_hint},
    serialize::{self, SerializeBuffer, SerializeError},
};

/// A trait for types that can be serialized.
pub trait Serializable<'facet>: Facet<'facet> {
    /// The [`TypeSerializeResult`] for this type.
    const SERIALIZABLE: &'static TypeSerializeResult = &calculate_shape_serialize(Self::SHAPE);
    /// A hint for the size of this type after serialization.
    const SERIALIZE_HINT: &'static TypeSerializeHint = &calculate_shape_hint(Self::SHAPE, None);

    /// Serialize a value into a buffer,
    /// returning a slice containing the serialized data.
    ///
    /// # Errors
    ///
    /// This function will return an error if serialization fails,
    /// or if the buffer cannot be written to.
    #[inline]
    fn to_buffer<'output: 'facet, B: SerializeBuffer>(
        &'facet self,
        buffer: &'output mut B,
    ) -> Result<&'output [u8], FSError<SerializeError>> {
        serialize::to_buffer::<Self, B>(self, buffer)
    }

    /// Serialize a value of type `T` into a [`Writer`](std::io::Write).
    ///
    /// # Errors
    ///
    /// This function will return an error if serialization fails,
    /// or the writer encounters an I/O error.
    #[inline]
    #[cfg(feature = "streaming")]
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), FSError<SerializeError>> {
        serialize::to_writer::<Self, W>(self, writer)
    }

    /// Serialize a value of type `T` into an asynchronous
    /// [`AsyncWrite`](futures_io::AsyncWrite).
    ///
    /// # Errors
    ///
    /// This function will return an error if serialization fails,
    /// or the writer encounters an I/O error.
    #[inline]
    #[cfg(feature = "futures-io")]
    fn to_async_writer<W: futures_io::AsyncWrite>(
        &self,
        writer: &mut W,
    ) -> impl Future<Output = Result<(), FSError<SerializeError>>> {
        serialize::to_async_writer::<Self, W>(self, writer)
    }

    /// Serialize a value of type `T` into an asynchronous
    /// [`AsyncWrite`](tokio::io::AsyncWrite).
    ///
    /// # Errors
    ///
    /// This function will return an error if serialization fails,
    /// or the writer encounters an I/O error.
    #[inline]
    #[cfg(feature = "tokio")]
    fn to_tokio_writer<W: tokio::io::AsyncWrite>(
        &self,
        writer: &mut W,
    ) -> impl Future<Output = Result<(), FSError<SerializeError>>> {
        serialize::to_tokio_writer::<Self, W>(self, writer)
    }
}

impl<'facet, T: Facet<'facet>> Serializable<'facet> for T {}

// -------------------------------------------------------------------------------------------------

/// A helper function to calculate the [`TypeSerializeResult`] for a [`Shape`].
const fn calculate_shape_serialize(_shape: &'static Shape) -> TypeSerializeResult { todo!() }
