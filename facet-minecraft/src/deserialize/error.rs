use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An error that occurred during deserialization.
#[derive(Debug)]
#[repr(transparent)]
pub struct DeserializeError {
    kind: DeserializeErrorKind,
}

impl DeserializeError {
    /// Create a new [`DeserializeError`].
    #[inline]
    #[must_use]
    pub const fn new(kind: DeserializeErrorKind) -> Self { Self { kind } }

    /// Get the kind of deserialization error.
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &DeserializeErrorKind { &self.kind }
}

/// The type of deserialization error.
#[derive(Debug)]
pub enum DeserializeErrorKind {
    /// An invalid boolean value was encountered.
    InvalidBool(u8),
    /// An invalid enum variant was encountered.
    InvalidVariant(usize),
    /// An invalid UTF-8 sequence was encountered.
    InvalidUtf8,

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { core::fmt::Debug::fmt(self, f) }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for DeserializeError {
    fn from(err: std::io::Error) -> Self { Self { kind: DeserializeErrorKind::Io(err) } }
}
