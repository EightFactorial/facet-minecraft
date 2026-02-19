//! [`DeserializeIter`] and related types.
#![allow(dead_code, reason = "WIP")]

use facet::{Facet, Partial, ReflectError, Shape};

use crate::deserialize::error::DeserializeIterError;

/// An iterator over the fields of a type.
pub struct DeserializeIter<'facet, const BORROW: bool> {
    input: &'static Shape,
    last: Partial<'facet, BORROW>,
}

impl<'facet> DeserializeIter<'facet, true> {
    /// Creates a new [`DeserializeIter`] for the given type.
    ///
    /// # Error
    ///
    /// Returns an error if the type is unsized.
    pub fn new<T: Facet<'facet>>() -> Result<Self, ReflectError> {
        Ok(Self { input: T::SHAPE, last: Partial::alloc::<T>()? })
    }

    /// Returns the final [`Partial`] after deserialization is complete.
    #[inline]
    #[must_use]
    pub fn into_partial(self) -> Partial<'facet, true> { self.last }
}

impl DeserializeIter<'static, false> {
    /// Creates a new [`DeserializeIter`] for the given type.
    ///
    /// # Error
    ///
    /// Returns an error if the type is unsized.
    pub fn new<T: Facet<'static>>() -> Result<Self, ReflectError> {
        Ok(Self { input: T::SHAPE, last: Partial::alloc_owned::<T>()? })
    }

    /// Returns the final [`Partial`] after deserialization is complete.
    #[inline]
    #[must_use]
    pub fn into_partial(self) -> Partial<'static, false> { self.last }
}

// -------------------------------------------------------------------------------------------------

impl<'facet, const BORROW: bool> Iterator for DeserializeIter<'facet, BORROW> {
    type Item = Result<Partial<'facet>, DeserializeIterError<'facet>>;

    fn next(&mut self) -> Option<Self::Item> { todo!() }
}
