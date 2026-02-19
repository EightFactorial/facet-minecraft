//! TODO
#![allow(unpredictable_function_pointer_comparisons, reason = "Shouldn't be compared like that")]
#![expect(clippy::result_unit_err, reason = "Temporary")]

use facet::Facet;

type PtrType = fn() -> Result<(), ()>;

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
    pub fn call(&self) -> Result<(), ()> { (self.ptr)() }
}

// -------------------------------------------------------------------------------------------------
