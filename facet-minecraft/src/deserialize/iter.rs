//! [`DeserializeIter`] and related types.
#![allow(dead_code, reason = "WIP")]

use alloc::{string::String, vec::Vec};
use core::marker::PhantomData;

use facet::{Facet, HeapValue, Partial, ReflectError, Shape};

use crate::deserialize::error::{DeserializeError, DeserializeIterError};

/// An iterator over the fields of a type.
///
/// Uses [`Partial`]s to provide locations for field data.
pub struct DeserializeIter<'facet, const BORROW: bool> {
    input: &'static Shape,
    partial: Partial<'facet, BORROW>,
}

impl<'facet> DeserializeIter<'facet, true> {
    /// Creates a new [`DeserializeIter`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is unsized.
    pub fn new<T: Facet<'facet>>() -> Result<Self, ReflectError> {
        Ok(Self { input: T::SHAPE, partial: Partial::alloc::<T>()? })
    }
}

impl DeserializeIter<'static, false> {
    /// Creates a new [`DeserializeIter`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is unsized.
    pub fn new<T: Facet<'static>>() -> Result<Self, ReflectError> {
        Ok(Self { input: T::SHAPE, partial: Partial::alloc_owned::<T>()? })
    }
}

/// A [`Partial`] value that must be filled in by a deserializer.
pub enum PartialValue<'mem, 'facet, const BORROW: bool> {
    /// A [`bool`] value.
    Bool(PartialLense<'mem, 'facet, BORROW, bool>),
    /// A [`u8`] value.
    U8(PartialLense<'mem, 'facet, BORROW, u8>),
    /// A [`u16`] value, and whether it is variable-length encoded.
    U16(PartialLense<'mem, 'facet, BORROW, u16>, bool),
    /// A [`u32`] value, and whether it is variable-length encoded.
    U32(PartialLense<'mem, 'facet, BORROW, u32>, bool),
    /// A [`u64`] value, and whether it is variable-length encoded.
    U64(PartialLense<'mem, 'facet, BORROW, u64>, bool),
    /// A [`u128`] value, and whether it is variable-length encoded.
    U128(PartialLense<'mem, 'facet, BORROW, u128>, bool),
    /// A [`usize`] value, and whether it is variable-length encoded.
    Usize(PartialLense<'mem, 'facet, BORROW, usize>, bool),
    /// A [`i8`] value.
    I8(PartialLense<'mem, 'facet, BORROW, i8>),
    /// A [`i16`] value, and whether it is variable-length encoded.
    I16(PartialLense<'mem, 'facet, BORROW, i16>, bool),
    /// A [`i32`] value, and whether it is variable-length encoded.
    I32(PartialLense<'mem, 'facet, BORROW, i32>, bool),
    /// A [`i64`] value, and whether it is variable-length encoded.
    I64(PartialLense<'mem, 'facet, BORROW, i64>, bool),
    /// A [`i128`] value, and whether it is variable-length encoded.
    I128(PartialLense<'mem, 'facet, BORROW, i128>, bool),
    /// A [`isize`] value, and whether it is variable-length encoded.
    Isize(PartialLense<'mem, 'facet, BORROW, isize>, bool),
    /// A [`f32`] value.
    F32(PartialLense<'mem, 'facet, BORROW, f32>),
    /// A [`f64`] value.
    F64(PartialLense<'mem, 'facet, BORROW, f64>),
    /// A [`str`] value.
    Str(PartialLense<'mem, 'facet, BORROW, &'facet str>),
    /// A [`String`] value.
    String(PartialLense<'mem, 'facet, BORROW, String>),
    /// A [`&[u8]`](::core::primitive::slice) value.
    Bytes(PartialLense<'mem, 'facet, BORROW, &'facet [u8]>),
    /// A [`Vec<u8>`] value.
    VecBytes(PartialLense<'mem, 'facet, BORROW, Vec<u8>>),
}

/// A lense for a [`Partial`] that allows setting it's value.
pub struct PartialLense<'mem, 'facet, const BORROW: bool, T: Facet<'facet>> {
    partial: &'mem mut Partial<'facet, BORROW>,
    _phantom: PhantomData<T>,
}

impl<'mem, 'facet, const BORROW: bool, T: Facet<'facet>> PartialLense<'mem, 'facet, BORROW, T> {
    /// Creates a new [`PartialLense`] for the given [`Partial`].
    ///
    /// # Panics
    ///
    /// Panics if the [`Partial`] is not for the same type as the lense.
    pub fn new(partial: &'mem mut Partial<'facet, BORROW>) -> Self {
        partial.shape().assert_shape(T::SHAPE);
        Self { partial, _phantom: PhantomData }
    }

    /// Sets the value of the [`Partial`] this lense points to.
    ///
    /// # Panics
    ///
    /// If the shape of the [`Partial`] is not correct, this will panic.
    ///
    /// To prevent undefined behavior, the process will be aborted if this
    /// panics.
    ///
    /// TODO: Check if this can use `unwrap_unchecked` instead of `unwrap` to
    /// avoid the extra check, since we should already know that the shape
    /// is correct.
    pub fn set_value(self, value: T) {
        replace_with::replace_with_or_abort(self.partial, |partial| partial.set(value).unwrap());
    }
}

// -------------------------------------------------------------------------------------------------

impl<'facet, const BORROW: bool> DeserializeIter<'facet, BORROW> {
    /// Advances the iterator to the next field.
    ///
    /// Returns `None` if there are no more fields to process.
    ///
    /// # Errors
    ///
    /// Returns an error if the processor fails to process a [`Partial`].
    ///
    /// Can be resumed after an error, allowing the processor to fix the error
    /// and try again.
    pub fn next<
        F: FnMut(PartialValue<'_, 'facet, BORROW>) -> Result<(), DeserializeIterError<'facet>>,
    >(
        &mut self,
        _processor: F,
    ) -> Option<Result<(), DeserializeIterError<'facet>>> {
        todo!()
    }

    /// Returns the final [`Partial`] after deserialization is complete.
    ///
    /// # Errors
    ///
    /// Returns an error if the processor fails to process a [`Partial`].
    pub fn complete<
        F: FnMut(PartialValue<'_, 'facet, BORROW>) -> Result<(), DeserializeIterError<'facet>>,
    >(
        mut self,
        mut processor: F,
    ) -> Result<HeapValue<'facet, BORROW>, DeserializeError<'facet>> {
        loop {
            match self.next(&mut processor) {
                Some(Ok(())) => {}
                Some(Err(err)) => return Err(err.into()),
                None => return self.partial.build().map_err(Into::into),
            }
        }
    }

    /// Consumes the iterator and returns current [`Partial`].
    ///
    /// This should only be used after the iterator has been fully processed
    /// and the final [`Partial`] is ready to be built.
    #[must_use]
    pub fn into_partial(self) -> Partial<'facet, BORROW> { self.partial }
}
