//! TODO

use facet::Facet;

use crate::{
    deserialize::{error::DeserializeError, iter::DeserializeIter},
    hint::TypeSerializeHint,
};

pub mod error;
pub mod fns;
pub mod iter;

/// A trait for types that can be deserialized.
pub trait Deserialize<'facet>: Sized {
    /// The [`TypeSerializeHint`] for this type.
    const SIZE_HINT: TypeSerializeHint;

    /// Deserialize a value from a [`slice`](::core::primitive::slice),
    /// borrowing data where possible.
    ///
    /// If the type will outlive the input data, use
    /// [`Deserialize::from_slice_owned`] instead.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails.
    fn from_slice(_slice: &'facet [u8]) -> Result<Self, DeserializeError<'facet>>
    where
        Self: Facet<'facet>,
    {
        let _iter = DeserializeIter::<true>::new::<Self>();
        todo!()
    }

    /// Deserialize a value from a [`slice`](::core::primitive::slice).
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails.
    fn from_slice_owned(_slice: &[u8]) -> Result<Self, DeserializeError<'static>>
    where
        Self: Facet<'static>,
    {
        let _iter = DeserializeIter::<false>::new::<Self>();
        todo!()
    }

    /// Deserialize a value from a reader.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails or if reading
    /// fails.
    #[cfg(feature = "std")]
    fn from_reader<R: std::io::Read>(_reader: R) -> Result<Self, DeserializeError<'static>>
    where
        Self: Facet<'static>,
    {
        todo!()
    }

    /// Deserialize a value from a [`futures_lite`] reader.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails or if reading
    /// fails.
    #[cfg(feature = "futures-lite")]
    fn from_async_reader<R: futures_lite::AsyncReadExt>(
        _reader: R,
    ) -> impl Future<Output = Result<Self, DeserializeError<'static>>>
    where
        Self: Facet<'static>,
    {
        async move { todo!() }
    }

    /// Deserialize a value from a [`tokio`] reader.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails or if reading
    /// fails.
    #[cfg(feature = "tokio")]
    fn from_tokio_reader<R: tokio::io::AsyncReadExt>(
        _reader: R,
    ) -> impl Future<Output = Result<Self, DeserializeError<'static>>>
    where
        Self: Facet<'static>,
    {
        async move { todo!() }
    }
}

impl<'facet, T: Facet<'facet>> Deserialize<'facet> for T {
    const SIZE_HINT: TypeSerializeHint = crate::hint::calculate_shape_hint(Self::SHAPE, None);
}

// -------------------------------------------------------------------------------------------------
