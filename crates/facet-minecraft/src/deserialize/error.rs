use alloc::{borrow::Cow, boxed::Box, format, string::ToString, vec::Vec};
use core::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Range,
};

use facet_reflect::ReflectError;

#[cfg(feature = "rich-diagnostics")]
use crate::report::LabeledRange;

/// An error that occurred during deserialization.
pub struct DeserError<'a> {
    /// The source data that was being deserialized.
    source: Cow<'a, [u8]>,
    /// The name of the type that failed to deserialize.
    type_name: &'static str,

    /// The reason for the error.
    reason: DeserErrorKind,
    /// The span of the error in the source data.
    span: Range<usize>,

    /// Additional help messages and notes to include in the error report.
    #[cfg(feature = "rich-diagnostics")]
    help_notes: Vec<(Cow<'a, str>, bool)>,
    /// Additional labels to include in the error report.
    #[cfg(feature = "rich-diagnostics")]
    labels: Vec<LabeledRange<'a>>,
    /// The location where the function was called.
    #[cfg(feature = "rich-diagnostics")]
    location: Option<&'static core::panic::Location<'static>>,
}

impl<'a> DeserError<'a> {
    /// Create a new [`DeserError`].
    #[must_use]
    pub fn new<T: ?Sized>(source: &'a [u8], reason: DeserErrorKind, span: Range<usize>) -> Self {
        Self::new_using(source, core::any::type_name::<T>(), reason, span)
    }

    /// Create a new [`DeserError`] with a specific type name.
    #[must_use]
    pub const fn new_using(
        source: &'a [u8],
        type_name: &'static str,
        reason: DeserErrorKind,
        span: Range<usize>,
    ) -> Self {
        Self {
            source: Cow::Borrowed(source),
            type_name,
            reason,
            span,
            #[cfg(feature = "rich-diagnostics")]
            help_notes: Vec::new(),
            #[cfg(feature = "rich-diagnostics")]
            labels: Vec::new(),
            #[cfg(feature = "rich-diagnostics")]
            location: None,
        }
    }

    /// Get the source data that was being deserialized.
    #[must_use]
    pub const fn source(&self) -> &[u8] {
        match self.source {
            Cow::Borrowed(b) => b,
            Cow::Owned(ref b) => b.as_slice(),
        }
    }

    /// Get the name of the type that failed to deserialize.
    #[inline]
    #[must_use]
    pub const fn type_name(&self) -> &'static str { self.type_name }

    /// Get the inner [`DeserErrorKind`].
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &DeserErrorKind { &self.reason }

    /// Get the span of the error in the source data.
    #[inline]
    #[must_use]
    pub const fn span(&self) -> &Range<usize> { &self.span }

    /// Get the list of help messages notes associated with the error.
    ///
    /// # Note
    ///
    /// `true` indicates a help message, while `false` indicates a note.
    #[inline]
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub const fn messages(&self) -> &[(Cow<'a, str>, bool)] { self.help_notes.as_slice() }

    /// Get the list of labels associated with the error.
    #[inline]
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub const fn labels(&self) -> &[LabeledRange<'a>] { self.labels.as_slice() }

    /// Get the location where the function was called, if available.
    #[inline]
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub const fn location(&self) -> Option<&'static core::panic::Location<'static>> {
        self.location
    }

    /// Set the source data that was used during deserialization.
    ///
    /// Adjusts all spans by sliding them down to match their original
    /// positions.
    #[must_use]
    pub fn with_source(mut self, source: &'a [u8]) -> Self {
        let new_len = source.len();
        let old_len = self.source.len();
        let len_diff = new_len.saturating_sub(old_len);

        self.source = Cow::Borrowed(source);

        // Adjust the error span range
        self.span.start += len_diff;
        self.span.end += len_diff;

        // Adjust the span range of all labels
        #[cfg(feature = "rich-diagnostics")]
        for label in &mut self.labels {
            label.range.start += len_diff;
            label.range.end += len_diff;
        }

        self
    }

    /// Set the name of the type that failed to deserialize.
    #[must_use]
    pub fn with_type_name<T: ?Sized>(mut self) -> Self {
        self.type_name = core::any::type_name::<T>();
        self
    }

    /// Add a help message to the error.
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub fn with_help<T: Into<Cow<'a, str>>>(mut self, help: T) -> Self {
        self.help_notes.push((help.into(), true));
        self
    }

    /// Add a note to the error.
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub fn with_note<T: Into<Cow<'a, str>>>(mut self, help: T) -> Self {
        self.help_notes.push((help.into(), false));
        self
    }

    /// Add an additional label to the error.
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub fn with_label<T: Into<Cow<'a, str>>>(mut self, label: T, span: Range<usize>) -> Self {
        self.labels.push(LabeledRange::new(label, span));
        self
    }

    /// Add the location where the function was called.
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub fn with_location(mut self, location: &'static core::panic::Location<'static>) -> Self {
        self.location = Some(location);
        self
    }

    /// Take ownership of the error, copying the source data if necessary.
    #[must_use]
    pub fn into_owned(self) -> DeserError<'static> {
        DeserError {
            source: Cow::Owned(self.source.into_owned()),
            type_name: self.type_name,
            reason: self.reason,
            span: self.span,
            #[cfg(feature = "rich-diagnostics")]
            help_notes: self
                .help_notes
                .into_iter()
                .map(|(h, b)| (h.into_owned().into(), b))
                .collect(),
            #[cfg(feature = "rich-diagnostics")]
            labels: self.labels.into_iter().map(LabeledRange::into_owned).collect(),
            #[cfg(feature = "rich-diagnostics")]
            location: self.location,
        }
    }

    /// Get the [`ariadne::Report`] for this error.
    #[must_use]
    #[cfg(feature = "rich-diagnostics")]
    pub fn as_report(&self) -> crate::report::ReportDisplay<'a, (&'static str, Range<usize>)> {
        From::from(self)
    }
}

impl Error for DeserError<'_> {}
impl Debug for DeserError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("DeserError").field("reason", &self.reason).finish_non_exhaustive()
    }
}
impl Display for DeserError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to deserialize `{}`, {}", self.type_name, self.reason.description())
    }
}

#[cfg(feature = "std")]
impl From<DeserError<'_>> for std::io::Error {
    fn from(value: DeserError<'_>) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, value.into_owned())
    }
}

// -------------------------------------------------------------------------------------------------

/// A type of error that can occur during deserialization.
#[derive(Debug)]
pub enum DeserErrorKind {
    /// An invalid byte was encountered when deserializing a boolean.
    InvalidBool(u8),
    /// The input ended unexpectedly, and optionally,
    /// the number of bytes expected.
    EndOfInput(Option<usize>),
    /// The input was not valid UTF-8.
    InvalidUtf8(core::str::Utf8Error),
    /// An error occurred during reflection.
    Reflect(ReflectError),
    /// Some other error occurred.
    Other(Box<dyn Error + Send + Sync>),
}

impl DeserErrorKind {
    /// Get a [`str`] with a brief description of this error.
    ///
    /// Because it does not allocate, this is suitable for use in
    /// `const` contexts.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            DeserErrorKind::InvalidBool(_) => "Invalid boolean value",
            DeserErrorKind::EndOfInput(_) => "Unexpected end of input",
            DeserErrorKind::InvalidUtf8(_) => "Invalid UTF-8",
            DeserErrorKind::Reflect(_) => "Reflection error",
            DeserErrorKind::Other(_) => "Other",
        }
    }

    /// Create an error message for this error.
    #[must_use]
    pub fn as_message(&self) -> Cow<'static, str> {
        match self {
            DeserErrorKind::InvalidBool(b) => Cow::Owned(format!("Invalid boolean value: `{b}`")),

            DeserErrorKind::EndOfInput(None) => {
                Cow::Borrowed("Reached the end, but expected more data")
            }
            DeserErrorKind::EndOfInput(Some(1)) => {
                Cow::Borrowed("Reached the end, but expected 1 more byte")
            }
            DeserErrorKind::EndOfInput(Some(n)) => {
                Cow::Owned(format!("Reached the end, but expected {n} more bytes"))
            }

            DeserErrorKind::InvalidUtf8(_) => Cow::Borrowed("Invalid UTF-8 sequence"),

            DeserErrorKind::Reflect(reflect) => match reflect {
                ReflectError::NoSuchVariant { enum_type } => Cow::Owned(format!(
                    "Unexpected enum type, expected one of: {:?}",
                    enum_type.variants.iter().map(|v| v.name.to_string()).collect::<Vec<_>>()
                )),
                ReflectError::WrongShape { .. } => todo!(),
                ReflectError::WasNotA { .. } => todo!(),
                ReflectError::UninitializedField { .. } => todo!(),
                ReflectError::UninitializedEnumField { .. } => todo!(),
                ReflectError::UninitializedValue { shape } => {
                    Cow::Owned(format!("Uninitialized `{}` value", shape.type_identifier))
                }
                ReflectError::InvariantViolation { invariant } => {
                    Cow::Owned(format!("Invariant violation, {invariant}"))
                }
                ReflectError::MissingCharacteristic { .. } => todo!(),
                ReflectError::OperationFailed { operation, .. } => {
                    Cow::Owned(format!("Operation failed, {operation}"))
                }
                ReflectError::FieldError { .. } => todo!(),
                ReflectError::MissingPushPointee { .. } => todo!(),
                ReflectError::Unknown => todo!(),
                ReflectError::TryFromError { .. } => todo!(),
                ReflectError::DefaultAttrButNoDefaultImpl { .. } => todo!(),
                ReflectError::Unsized { .. } => todo!(),
                ReflectError::ArrayNotFullyInitialized { .. } => todo!(),
                ReflectError::ArrayIndexOutOfBounds { .. } => todo!(),
                ReflectError::InvalidOperation { .. } => todo!(),
                ReflectError::UnexpectedTracker { .. } => todo!(),
                ReflectError::NoActiveFrame => todo!(),
                ReflectError::HeistCancelledDifferentShapes { .. } => todo!(),
            },

            DeserErrorKind::Other(err) => Cow::Owned(err.to_string()),
        }
    }
}

impl Error for DeserErrorKind {}
impl Display for DeserErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.write_str(&self.as_message()) }
}

impl From<core::str::Utf8Error> for DeserErrorKind {
    fn from(err: core::str::Utf8Error) -> Self { DeserErrorKind::InvalidUtf8(err) }
}
impl From<ReflectError> for DeserErrorKind {
    fn from(err: ReflectError) -> Self { DeserErrorKind::Reflect(err) }
}
impl<T: Error + Send + Sync + 'static> From<Box<T>> for DeserErrorKind {
    fn from(err: Box<T>) -> Self { DeserErrorKind::Other(err) }
}
