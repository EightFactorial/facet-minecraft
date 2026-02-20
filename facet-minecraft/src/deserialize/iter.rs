//! [`DeserializeIter`] and related types.
#![allow(dead_code, reason = "WIP")]

use facet::{Facet, HeapValue, Partial, ReflectError, Shape};

use crate::deserialize::error::DeserializeIterError;

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

    /// Returns the final [`Partial`] after deserialization is complete.
    ///
    /// # Errors
    ///
    /// Returns an error if the processor fails to process a [`Partial`].
    pub fn complete<
        F: FnMut(Partial<'facet, true>) -> Result<Partial<'facet, true>, DeserializeIterError<'facet>>,
    >(
        self,
        _processor: F,
    ) -> Result<HeapValue<'facet, true>, DeserializeIterError<'facet>> {
        todo!()
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

    /// Returns the final [`Partial`] after deserialization is complete.
    ///
    /// # Errors
    ///
    /// Returns an error if the processor fails to process a [`Partial`].
    pub fn complete<
        F: FnMut(
            Partial<'static, false>,
        ) -> Result<Partial<'static, false>, DeserializeIterError<'static>>,
    >(
        self,
        _processor: F,
    ) -> Result<HeapValue<'static, false>, DeserializeIterError<'static>> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------
