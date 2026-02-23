//! TODO
#![allow(unpredictable_function_pointer_comparisons, reason = "Shouldn't be compared like that")]

use facet::{Facet, Partial};

use crate::{
    Deserialize,
    deserialize::{
        InputCursor,
        error::{DeserializeError, DeserializeValueError},
    },
};

type PtrTypeBorrowed = for<'facet> fn(
    &mut Partial<'facet, true>,
    &mut InputCursor<'facet, 'facet>,
) -> Result<(), DeserializeValueError>;
type PtrTypeOwned = for<'facet> fn(
    &mut Partial<'facet, false>,
    &mut InputCursor<'_, 'facet>,
) -> Result<(), DeserializeValueError>;

/// A custom deserializer function.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct DeserializeFn {
    ptr_borrowed: PtrTypeBorrowed,
    ptr_owned: PtrTypeOwned,
}

impl DeserializeFn {
    /// Creates a new [`DeserializeFn`].
    #[inline]
    #[must_use]
    pub const fn new(borrowed: PtrTypeBorrowed, owned: PtrTypeOwned) -> Self {
        Self { ptr_borrowed: borrowed, ptr_owned: owned }
    }

    /// Call the deserializer function.
    ///
    /// Automatically selects the borrowed or owned deserializer based on the
    /// `BORROW` const generic parameter.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    #[inline]
    pub fn call<'input, 'facet, const BORROW: bool>(
        &self,
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'input, 'facet>,
    ) -> Result<(), DeserializeValueError>
    where
        Self: sealed::UnifyDeserFn<'input, 'facet, BORROW>,
    {
        sealed::UnifyDeserFn::call(self, partial, cursor)
    }

    /// Call the borrowed deserializer function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    #[inline]
    pub fn call_borrowed<'facet>(
        &self,
        partial: &mut Partial<'facet, true>,
        cursor: &mut InputCursor<'facet, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        (self.ptr_borrowed)(partial, cursor)
    }

    /// Call the owned deserializer function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    #[inline]
    pub fn call_owned<'facet>(
        &self,
        partial: &mut Partial<'facet, false>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        (self.ptr_owned)(partial, cursor)
    }
}

mod sealed {
    use super::{DeserializeFn, DeserializeValueError, InputCursor, Partial};
    pub trait UnifyDeserFn<'input, 'facet, const BORROW: bool> {
        fn call(
            &self,
            partial: &mut Partial<'facet, BORROW>,
            cursor: &mut InputCursor<'input, 'facet>,
        ) -> Result<(), DeserializeValueError>;
    }

    impl<'input: 'facet, 'facet: 'input> UnifyDeserFn<'input, 'facet, true> for DeserializeFn {
        #[inline]
        fn call(
            &self,
            partial: &mut Partial<'facet, true>,
            cursor: &mut InputCursor<'input, 'facet>,
        ) -> Result<(), DeserializeValueError> {
            self.call_borrowed(partial, cursor)
        }
    }
    impl<'facet> UnifyDeserFn<'_, 'facet, false> for DeserializeFn {
        #[inline]
        fn call(
            &self,
            partial: &mut Partial<'facet, false>,
            cursor: &mut InputCursor<'_, 'facet>,
        ) -> Result<(), DeserializeValueError> {
            self.call_owned(partial, cursor)
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Deserialize a value from a `&[u8]` slice,
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

/// Deserialize a value from a `&[u8]` slice.
///
/// # Errors
///
/// Returns a [`DeserializeError`] if deserialization fails.
#[inline]
pub fn from_slice_owned<T: Facet<'static>>(slice: &[u8]) -> Result<T, DeserializeError<'static>> {
    Deserialize::from_slice_owned(slice)
}

/// Deserialize a value from a `&[u8]` slice,
/// returning any remaining data.
///
/// # Errors
///
/// Returns a [`DeserializeError`] if deserialization fails.
#[inline]
pub fn from_slice_remainder<T: Facet<'static>>(
    slice: &[u8],
) -> Result<(T, &[u8]), DeserializeError<'static>> {
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
