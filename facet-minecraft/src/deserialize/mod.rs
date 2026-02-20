//! TODO

#[cfg(feature = "std")]
use facet::HeapValue;
use facet::{Facet, Partial};

use crate::{
    deserialize::{
        error::{DeserializeError, DeserializeIterError},
        iter::DeserializeIter,
    },
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
        DeserializeIter::<true>::new::<Self>()?
            .complete(borrowed_processor)?
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
    fn from_slice_remainder(_slice: &[u8]) -> Result<(Self, &[u8]), DeserializeError<'static>>
    where
        Self: Facet<'static>,
    {
        let _value = DeserializeIter::<false>::new::<Self>()?
            .complete(owned_processor)?
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
        from_coroutine(DeserializeIter::<false>::new::<Self>()?, move |buf: &mut [u8]| {
            reader.read_exact(buf).map_err(Into::into)
        })?
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
            )
            .await?
            .materialize::<Self>()
            .map_err(Into::into)
        }
    }
}

impl<'facet, T: Facet<'facet>> Deserialize<'facet> for T {
    const SIZE_HINT: TypeSerializeHint = crate::hint::calculate_shape_hint(Self::SHAPE, None);
}

// -------------------------------------------------------------------------------------------------

fn borrowed_processor(
    _partial: Partial<'_, true>,
) -> Result<Partial<'_, true>, DeserializeIterError<'_>> {
    todo!()
}

fn owned_processor(
    _partial: Partial<'static, false>,
) -> Result<Partial<'static, false>, DeserializeIterError<'static>> {
    todo!()
}

#[cfg(feature = "std")]
fn from_coroutine<F: FnMut(&mut [u8]) -> Result<(), DeserializeError<'static>>>(
    _iter: DeserializeIter<'static, false>,
    _reader: F,
) -> Result<HeapValue<'static, false>, DeserializeError<'static>> {
    todo!()
}

#[expect(clippy::unused_async, reason = "WIP")]
#[cfg(any(feature = "futures-lite", feature = "tokio"))]
async fn from_async_coroutine<F: AsyncFnMut(&mut [u8]) -> Result<(), DeserializeError<'static>>>(
    _iter: DeserializeIter<'static, false>,
    _reader: F,
) -> Result<HeapValue<'static, false>, DeserializeError<'static>> {
    todo!()
}
