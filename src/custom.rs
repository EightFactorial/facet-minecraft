//! Custom facet ser/de overrides
//!
//! Hopefully temporary...

use alloc::vec::Vec;

use facet::{ConstTypeId, Facet};
use facet_reflect::Peek;
use once_cell::sync::OnceCell;

use crate::serialize::SerializationTask;

/// A custom serialization or deserialization override for a [`Facet`].
pub struct FacetOverride {
    /// The [`ConstTypeId`] of the [`Facet`] this override applies to.
    pub id: ConstTypeId,
    /// A custom serialization function for this [`FacetOverride`].
    pub serialize: Option<SerializeFn>,
    /// A custom deserialization function for this [`FacetOverride`].
    pub deserialize: Option<DeserializeFn>,
}

type SerializeFn =
    for<'mem, 'shape> fn(Peek<'mem, '_, 'shape>, &mut Vec<SerializationTask<'mem, '_, 'shape>>);
type DeserializeFn = for<'mem, 'shape> fn(Peek<'mem, '_, 'shape>);

impl FacetOverride {
    /// Returns a static slice of all registered [`FacetOverride`]s.
    #[must_use]
    pub fn global() -> &'static [(ConstTypeId, &'static FacetOverride)] {
        static GLOBAL: OnceCell<Vec<(ConstTypeId, &'static FacetOverride)>> = OnceCell::new();
        GLOBAL.get_or_init(|| inventory::iter::<Self>().map(|ty| (ty.id, ty)).collect())
    }

    /// Create a new [`FacetOverride`].
    #[must_use]
    pub const fn new<'a, T: Facet<'a>>() -> Self {
        FacetOverride { id: T::SHAPE.id, serialize: None, deserialize: None }
    }

    /// Add a custom serialization function for this [`FacetOverride`].
    #[must_use]
    pub const fn with_ser(self, serialize: SerializeFn) -> Self {
        FacetOverride { id: self.id, serialize: Some(serialize), deserialize: self.deserialize }
    }

    /// Add a custom deserialization function for this [`FacetOverride`].
    #[must_use]
    pub const fn with_de(self, deserialize: DeserializeFn) -> Self {
        FacetOverride { id: self.id, serialize: self.serialize, deserialize: Some(deserialize) }
    }
}

inventory::collect!(FacetOverride);
