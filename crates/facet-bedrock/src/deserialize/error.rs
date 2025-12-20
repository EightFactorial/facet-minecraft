use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An error that occurred during deserialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeserializeError {
    kind: DeserializeErrorKind,
}

impl DeserializeError {
    /// Creates a new [`DeserializeErrorKind::UnexpectedEndOfInput`]
    /// [`DeserializeError`].
    #[must_use]
    pub const fn new_eof(expected: usize, found: usize) -> Self {
        Self { kind: DeserializeErrorKind::UnexpectedEndOfInput { expected, found } }
    }
}

/// The type of deserialization error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeserializeErrorKind {
    /// The input ended unexpectedly.
    UnexpectedEndOfInput {
        /// The number of additional bytes expected.
        expected: usize,
        /// The number of bytes actually found.
        found: usize,
    },
}

// -------------------------------------------------------------------------------------------------

impl Error for DeserializeError {}
impl Display for DeserializeError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result { todo!() }
}
