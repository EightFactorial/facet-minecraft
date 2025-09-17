//! Custom serializer and deserializer functions

use alloc::vec::Vec;
use core::{any::TypeId, fmt::Debug};

use facet_core::ConstTypeId;
use facet_reflect::{Partial, Peek};
pub use linkme;

use crate::{
    deserialize::{DeserError, DeserStep},
    serialize::SerStep,
};

/// A list of all [`DeserializerFn`]s defined globally
#[linkme::distributed_slice]
pub static FACET_DESERIALIZERS: [DeserializerFn];
/// A [`DeserializerFn`] function pointer type
pub type DeserFunc = for<'partial, 'input> fn(
    &'partial mut Partial<'input>,
    &'input [u8],
    &mut Vec<DeserStep>,
) -> Result<
    (&'partial mut Partial<'input>, &'input [u8]),
    DeserError<'input>,
>;

/// A deserializer function
#[derive(Clone, Copy, Eq)]
pub struct DeserializerFn {
    /// The [`ConstTypeId`] of the type this function deserializes
    type_id: ConstTypeId,
    /// A function pointer to the deserializer function
    func: DeserFunc,
}

impl DeserializerFn {
    /// Get the [`DeserializerFn`] for the given type, if one exists.
    #[inline]
    #[must_use]
    pub fn for_type<T: ?Sized>() -> Option<Self> { Self::for_type_id(ConstTypeId::of::<T>()) }

    /// Get the [`DeserializerFn`] for the given [`ConstTypeId`], if one exists.
    #[must_use]
    pub fn for_type_id(type_id: ConstTypeId) -> Option<Self> {
        FACET_DESERIALIZERS.into_iter().find(|s| s.type_id == type_id).copied()
    }

    /// Create a new [`DeserializerFn`].
    #[must_use]
    pub const fn new<T: ?Sized>(func: DeserFunc) -> Self {
        Self { type_id: ConstTypeId::of::<T>(), func }
    }

    /// Get the [`ConstTypeId`] of the deserializer type.
    #[must_use]
    pub const fn const_type_id(&self) -> ConstTypeId { self.type_id }

    /// Get the [`TypeId`] of the deserializer type.
    #[must_use]
    pub fn type_id(&self) -> TypeId { self.type_id.get() }

    /// Run the deserializer function.
    ///
    /// # Errors
    ///
    /// Returns an error if the user's deserializer function fails.
    #[allow(clippy::result_large_err, reason = "Error is large if rich diagnostics are enabled")]
    pub fn run<'input, 'partial>(
        &self,
        partial: &'partial mut Partial<'input>,
        input: &'input [u8],
        instructions: &mut Vec<DeserStep>,
    ) -> Result<(&'partial mut Partial<'input>, &'input [u8]), DeserError<'input>> {
        (self.func)(partial, input, instructions)
    }
}

impl Debug for DeserializerFn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeserializerFn").field(&self.type_id).finish()
    }
}

impl PartialEq for DeserializerFn {
    fn eq(&self, other: &Self) -> bool { self.type_id == other.type_id }
}

// -------------------------------------------------------------------------------------------------

/// A list of all defined [`SerializerFn`]s defined globally
#[linkme::distributed_slice]
pub static FACET_SERIALIZERS: [SerializerFn];
/// A [`SerializerFn`] function pointer type
pub type SerFunc =
    for<'input, 'facet> fn(Peek<'input, 'facet>, bool, &mut Vec<SerStep<'input, 'facet>>);

/// A serializer function
#[derive(Clone, Copy, Eq)]
pub struct SerializerFn {
    /// The [`ConstTypeId`] of the type this function serializes
    type_id: ConstTypeId,
    /// A function pointer to the serializer function
    func: SerFunc,
}

impl SerializerFn {
    /// Get the [`SerializerFn`] for the given type, if one exists.
    #[inline]
    #[must_use]
    pub fn for_type<T: ?Sized>() -> Option<Self> { Self::for_type_id(ConstTypeId::of::<T>()) }

    /// Get the [`SerializerFn`] for the given [`ConstTypeId`], if one exists.
    #[must_use]
    pub fn for_type_id(type_id: ConstTypeId) -> Option<Self> {
        FACET_SERIALIZERS.into_iter().find(|s| s.type_id == type_id).copied()
    }

    /// Create a new [`SerializerFn`].
    #[must_use]
    pub const fn new<T: ?Sized>(func: SerFunc) -> Self {
        Self { type_id: ConstTypeId::of::<T>(), func }
    }

    /// Get the [`ConstTypeId`] of the serializer type.
    #[must_use]
    pub const fn const_type_id(&self) -> ConstTypeId { self.type_id }

    /// Get the [`TypeId`] of the serializer type.
    #[must_use]
    pub fn type_id(&self) -> TypeId { self.type_id.get() }

    /// Run the deserializer function.
    ///
    /// # Errors
    ///
    /// Returns an error if the user's deserializer function fails.
    #[allow(clippy::result_large_err, reason = "Error is large if rich diagnostics are enabled")]
    pub fn run<'input, 'facet>(
        &self,
        peek: Peek<'input, 'facet>,
        is_variable: bool,
        instructions: &mut Vec<SerStep<'input, 'facet>>,
    ) {
        (self.func)(peek, is_variable, instructions);
    }
}

impl Debug for SerializerFn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SerializerFn").field(&self.type_id).finish()
    }
}

impl PartialEq for SerializerFn {
    fn eq(&self, other: &Self) -> bool { self.type_id == other.type_id }
}
