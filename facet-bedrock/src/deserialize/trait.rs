use facet::{Facet, Shape};

/// A trait for types that can be deserialized.
pub trait Deserializable<'facet>: Facet<'facet> {
    /// The [`TypeDeserializable`] result for this type.
    const DESERIALIZABLE: TypeDeserializable;
    /// An optional hint for the size of this type when deserialized.
    const DESERIALIZE_HINT: Option<usize> = None;

    /// Returns `true` if the type will always deserialize successfully.
    #[inline]
    #[must_use]
    fn will_deserialize(&self) -> bool { Self::DESERIALIZABLE.will_deserialize() }

    /// Returns `true` if the type may deserialize successfully.
    #[inline]
    #[must_use]
    fn can_deserialize(&self) -> bool { Self::DESERIALIZABLE.can_deserialize() }
}

impl<'facet, T: Facet<'facet>> Deserializable<'facet> for T {
    const DESERIALIZABLE: TypeDeserializable = calculate_shape_serialize(T::SHAPE);
    const DESERIALIZE_HINT: Option<usize> = calculate_shape_hint(T::SHAPE);
}

/// A helper function to calculate the [`TypeDeserializable`] for a [`Shape`].
const fn calculate_shape_serialize(_shape: &'static Shape) -> TypeDeserializable { todo!() }

/// A helper function to calculate a size hint for a [`Shape`].
const fn calculate_shape_hint(_shape: &'static Shape) -> Option<usize> { todo!() }

// -------------------------------------------------------------------------------------------------

/// Indicates whether a type can be deserialized and whether it can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeDeserializable {
    /// The type will always deserialize successfully.
    Infallible,
    /// The type may deserialize successfully or fail.
    Fallible,
    /// The type will always fail to deserialize.
    Never,
}

impl TypeDeserializable {
    /// Returns `true` if the type will always deserialize successfully.
    #[must_use]
    pub const fn will_deserialize(&self) -> bool { matches!(self, TypeDeserializable::Infallible) }

    /// Returns `true` if the type may deserialize successfully.
    #[must_use]
    pub const fn can_deserialize(&self) -> bool {
        matches!(self, TypeDeserializable::Infallible | TypeDeserializable::Fallible)
    }

    /// Combine two [`TypeDeserializable`] values.
    #[must_use]
    #[expect(dead_code, reason = "WIP")]
    const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (TypeDeserializable::Infallible, TypeDeserializable::Infallible) => {
                TypeDeserializable::Infallible
            }
            (TypeDeserializable::Never, _) | (_, TypeDeserializable::Never) => {
                TypeDeserializable::Never
            }
            _ => TypeDeserializable::Fallible,
        }
    }
}
