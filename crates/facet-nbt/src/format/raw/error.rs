use core::{
    error::Error,
    fmt::{self, Debug, Display},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RawError<'a> {
    kind: RawErrorKind<'a>,
    source: (&'a [u8], &'a [u8]), // (input, remaining)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawErrorKind<'a> {
    /// An invalid tag type was encountered
    InvalidTagType(u8),
    /// A string was invalid MUTF-8
    InvalidString(&'a [u8]),
    /// The data input ended unexpectedly
    EndOfInput,
}

impl<'a> RawError<'a> {
    /// Create a [`RawError`] with the given error kind and remaining data.
    ///
    /// # Note
    /// Without calling [`RawError::with_input`],
    /// the input will be an empty slice.
    #[must_use]
    pub const fn new(kind: RawErrorKind<'a>, remaining: &'a [u8]) -> Self {
        Self { kind, source: (&[], remaining) }
    }

    /// Set the input data for this error.
    #[must_use]
    pub const fn with_input(mut self, input: &'a [u8]) -> Self {
        self.source.0 = input;
        self
    }

    /// Get the [`RawErrorKind`] of this error.
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> RawErrorKind<'a> { self.kind }
}

impl Display for RawError<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Debug::fmt(self, f) }
}

impl Error for RawError<'_> {}

// -------------------------------------------------------------------------------------------------

impl RawErrorKind<'_> {
    /// Get a static message for the error kind.
    #[must_use]
    pub const fn static_message(&self) -> &'static str {
        match self {
            RawErrorKind::InvalidTagType(..) => "Invalid tag type",
            RawErrorKind::InvalidString(..) => "Invalid MUTF-8 string",
            RawErrorKind::EndOfInput => "Unexpected end of input",
        }
    }
}

#[cfg(not(feature = "rich-diagnostics"))]
impl Debug for RawError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self.kind.static_message(), f)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "rich-diagnostics")]
impl RawError<'_> {}

#[cfg(feature = "rich-diagnostics")]
impl Debug for RawError<'_> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}
