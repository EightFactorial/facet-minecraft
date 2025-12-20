use facet::{Facet, Shape};

/// A trait for types that can be serialized.
pub trait Serializable<'facet>: Facet<'facet> {
    /// The [`TypeSerializable`] result for this type.
    const SERIALIZABLE: TypeSerializable;
    /// An optional hint for the size of this type when serialized.
    const SERIALIZE_HINT: Option<usize>;

    /// Returns `true` if the type will always serialize successfully.
    #[inline]
    #[must_use]
    fn will_serialize(&self) -> bool { Self::SERIALIZABLE.will_serialize() }

    /// Returns `true` if the type may serialize successfully.
    #[inline]
    #[must_use]
    fn can_serialize(&self) -> bool { Self::SERIALIZABLE.can_serialize() }
}

impl<'facet, T: Facet<'facet>> Serializable<'facet> for T {
    const SERIALIZABLE: TypeSerializable = calculate_shape_serialize(T::SHAPE);
    const SERIALIZE_HINT: Option<usize> = calculate_shape_hint(T::SHAPE);
}

/// A helper function to calculate the [`TypeSerializable`] for a [`Shape`].
const fn calculate_shape_serialize(_shape: &'static Shape) -> TypeSerializable { todo!() }

/// A helper function to calculate a size hint for a [`Shape`].
const fn calculate_shape_hint(_shape: &'static Shape) -> Option<usize> { todo!() }

// -------------------------------------------------------------------------------------------------

/// Indicates whether a type can be serialized and whether it can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSerializable {
    /// The type will always serialize successfully.
    Infallible,
    /// The type may serialize successfully or fail.
    Fallible,
    /// The type will always fail to serialize.
    Never,
}

impl TypeSerializable {
    /// Returns `true` if the type will always serialize successfully.
    #[must_use]
    pub const fn will_serialize(self) -> bool { matches!(self, TypeSerializable::Infallible) }

    /// Returns `true` if the type may serialize successfully.
    #[must_use]
    pub const fn can_serialize(self) -> bool {
        matches!(self, TypeSerializable::Infallible | TypeSerializable::Fallible)
    }

    /// Combine two [`TypeSerializable`] values.
    #[must_use]
    #[expect(dead_code, reason = "WIP")]
    const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (TypeSerializable::Infallible, TypeSerializable::Infallible) => {
                TypeSerializable::Infallible
            }
            (TypeSerializable::Never, _) | (_, TypeSerializable::Never) => TypeSerializable::Never,
            _ => TypeSerializable::Fallible,
        }
    }
}
