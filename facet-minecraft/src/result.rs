//! Common types used for analyzing type properties at compile time.
#![allow(clippy::too_many_lines, reason = "Recursive type analysis")]

/// Indicates whether a type can be serialized and whether it can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSerializeResult {
    /// The type will always serialize successfully.
    Infallible,
    /// The type may serialize successfully or fail.
    Fallible,
    /// The type will always fail to serialize.
    Never,
}

impl TypeSerializeResult {
    /// Returns `true` if the type will always serialize successfully.
    ///
    /// ### Note
    ///
    /// When serializing this type,
    /// the only possible errors come from I/O operations.
    #[must_use]
    pub const fn guaranteed(self) -> bool { matches!(self, TypeSerializeResult::Infallible) }

    /// Returns `true` if the type may serialize successfully.
    ///
    /// ### Note
    ///
    /// If this returns `false`,
    /// the type does not support serialization at all.
    #[must_use]
    pub const fn possible(self) -> bool { !matches!(self, TypeSerializeResult::Never) }

    /// Combines two [`TypeSerializeResult`]s into one,
    /// returning the overall result.
    #[must_use]
    pub const fn with(self, other: Self) -> Self {
        match (self, other) {
            (TypeSerializeResult::Infallible, TypeSerializeResult::Infallible) => {
                TypeSerializeResult::Infallible
            }
            (TypeSerializeResult::Never, _) | (_, TypeSerializeResult::Never) => {
                TypeSerializeResult::Never
            }
            _ => TypeSerializeResult::Fallible,
        }
    }
}
