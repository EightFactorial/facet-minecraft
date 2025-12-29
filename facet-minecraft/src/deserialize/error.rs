use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An error that occurred during deserialization.
#[derive(Debug)]
pub struct DeserializeError {
    kind: DeserializeErrorKind,
}

impl DeserializeError {
    /// Get the kind of deserialization error.
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &DeserializeErrorKind { &self.kind }

    /// Creates a new [`DeserializeErrorKind::UnexpectedEndOfInput`]
    /// [`DeserializeError`].
    #[must_use]
    pub const fn new_eof(expected: usize, found: usize) -> Self {
        Self { kind: DeserializeErrorKind::UnexpectedEndOfInput { expected, found } }
    }
}

/// The type of deserialization error.
#[derive(Debug)]
pub enum DeserializeErrorKind {
    /// The input ended unexpectedly.
    UnexpectedEndOfInput {
        /// The number of additional bytes expected.
        expected: usize,
        /// The number of bytes actually found.
        found: usize,
    },
    /// An I/O error occurred.
    #[cfg(feature = "std")]
    Io(std::io::Error),
}

// -------------------------------------------------------------------------------------------------

impl Error for DeserializeError {}
impl Display for DeserializeError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result { todo!() }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for DeserializeError {
    fn from(err: std::io::Error) -> Self { Self { kind: DeserializeErrorKind::Io(err) } }
}
