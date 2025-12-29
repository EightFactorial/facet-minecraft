use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An error that occurred during serialization.
#[derive(Debug)]
pub struct SerializeError {
    kind: SerializeErrorKind,
}

impl SerializeError {
    /// Get the kind of serialization error.
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &SerializeErrorKind { &self.kind }
}

/// The type of serialization error.
#[derive(Debug)]
pub enum SerializeErrorKind {
    /// An I/O error occurred.
    #[cfg(feature = "std")]
    Io(std::io::Error),
}

// -------------------------------------------------------------------------------------------------

impl Error for SerializeError {}
impl Display for SerializeError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result { todo!() }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for SerializeError {
    #[inline]
    fn from(err: std::io::Error) -> Self { Self { kind: SerializeErrorKind::Io(err) } }
}
