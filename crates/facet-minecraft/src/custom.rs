//! Custom facet serialization overrides
//!
//! Uses [`inventory`] to register implementations
//! and [`once_cell`] to provide a static slice.

use alloc::vec::Vec;
#[cfg(all(not(feature = "once_cell"), feature = "std"))]
use std::sync::LazyLock;

use facet::{ConstTypeId, Facet};
#[cfg(feature = "deserialize")]
pub use facet_reflect::Partial;
#[cfg(feature = "serialize")]
pub use facet_reflect::Peek;
pub use inventory::submit;
#[cfg(feature = "once_cell")]
use once_cell::sync::OnceCell;

#[cfg(feature = "deserialize")]
use crate::DeserializeError;
#[cfg(feature = "serialize")]
use crate::serialize::SerializationTask;

/// A custom serialization or deserialization override for a [`Facet`].
pub struct FacetOverride {
    /// The [`ConstTypeId`] of the [`Facet`] this override applies to.
    pub id: ConstTypeId,
    /// A custom serialization function for this [`FacetOverride`].
    #[cfg(feature = "serialize")]
    pub serialize: Option<SerializeFn>,
    /// A custom deserialization function for this [`FacetOverride`].
    #[cfg(feature = "deserialize")]
    pub deserialize: Option<DeserializeFn>,
}

#[cfg(feature = "serialize")]
type SerializeFn =
    for<'mem, 'facet> fn(Peek<'_, 'facet>, &mut Vec<SerializationTask<'mem, 'facet>>);
#[cfg(feature = "deserialize")]
type DeserializeFn = for<'input, 'partial, 'facet> fn(
    &'partial mut Partial<'facet>,
    &'input [u8],
) -> Result<
    (&'partial mut Partial<'facet>, &'input [u8]),
    DeserializeError<'input>,
>;

impl FacetOverride {
    /// Returns a static slice of all registered [`FacetOverride`]s.
    #[must_use]
    #[cfg(feature = "once_cell")]
    pub fn global() -> &'static [&'static FacetOverride] {
        static GLOBAL: OnceCell<Vec<&'static FacetOverride>> = OnceCell::new();
        GLOBAL.get_or_init(|| inventory::iter::<FacetOverride>().collect())
    }

    /// Returns a static slice of all registered [`FacetOverride`]s.
    #[must_use]
    #[cfg(all(not(feature = "once_cell"), feature = "std"))]
    pub fn global() -> &'static [&'static FacetOverride] {
        static GLOBAL: LazyLock<Vec<&'static FacetOverride>> =
            LazyLock::new(|| inventory::iter::<FacetOverride>().collect());
        &GLOBAL
    }

    /// Create a new [`FacetOverride`].
    #[must_use]
    pub const fn new<'a, T: Facet<'a>>() -> Self {
        FacetOverride {
            id: T::SHAPE.id,
            #[cfg(feature = "serialize")]
            serialize: None,
            #[cfg(feature = "deserialize")]
            deserialize: None,
        }
    }

    /// Add a custom serialization function for this [`FacetOverride`].
    #[must_use]
    #[cfg(feature = "serialize")]
    pub const fn with_ser(self, serialize: SerializeFn) -> Self {
        FacetOverride {
            id: self.id,
            serialize: Some(serialize),
            #[cfg(feature = "deserialize")]
            deserialize: self.deserialize,
        }
    }

    /// Add a custom deserialization function for this [`FacetOverride`].
    #[must_use]
    #[cfg(feature = "deserialize")]
    pub const fn with_de(self, deserialize: DeserializeFn) -> Self {
        FacetOverride {
            id: self.id,
            #[cfg(feature = "serialize")]
            serialize: self.serialize,
            deserialize: Some(deserialize),
        }
    }
}

inventory::collect!(FacetOverride);
