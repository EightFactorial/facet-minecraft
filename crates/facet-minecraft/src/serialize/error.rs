use alloc::{borrow::Cow, boxed::Box, format, string::ToString, vec::Vec};
use core::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Range,
};

use facet_reflect::ReflectError;

#[cfg(feature = "rich-diagnostics")]
use crate::report::LabeledRange;
use crate::serialize::Writer;

/// An error that occurred during serialization.
pub struct SerError<'a, W: Writer> {
    /// The source data that was being serialized.
    source: Cow<'a, [u8]>,
    /// The name of the type that failed to serialize.
    type_name: &'static str,

    /// The reason for the error.
    reason: SerErrorKind<W>,
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

impl<'a, W: Writer> SerError<'a, W> {
    /// Create a new [`SerError`].
    #[inline]
    #[must_use]
    pub fn new<T: ?Sized>(source: &'a [u8], reason: SerErrorKind<W>, span: Range<usize>) -> Self {
        Self::new_using(source, core::any::type_name::<T>(), reason, span)
    }

    /// Create a new [`SerError`] using the provided type name.
    #[must_use]
    pub fn new_using(
        source: &'a [u8],
        type_name: &'static str,
        reason: SerErrorKind<W>,
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

    /// Get the source data that was being serialized.
    #[must_use]
    pub const fn source(&self) -> &[u8] {
        match self.source {
            Cow::Borrowed(b) => b,
            Cow::Owned(ref b) => b.as_slice(),
        }
    }

    /// Get the name of the type that failed to serialize.
    #[inline]
    #[must_use]
    pub const fn type_name(&self) -> &'static str { self.type_name }

    /// Get the inner [`SerErrorKind`].
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &SerErrorKind<W> { &self.reason }

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

    /// Set the source data that was used during serialization.
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

    /// Set the name of the type that failed to serialize.
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
    pub fn into_owned(self) -> SerError<'static, W> {
        SerError {
            source: Cow::Owned(self.source.into_owned()),
            type_name: self.type_name,
            reason: self.reason,
            span: self.span,
            #[cfg(feature = "rich-diagnostics")]
            help_notes: self
                .help_notes
                .into_iter()
                .map(|(s, b)| (s.into_owned().into(), b))
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

impl<W: Writer> Error for SerError<'_, W> {}
impl<W: Writer> Debug for SerError<'_, W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SerError").field("reason", &self.reason).finish_non_exhaustive()
    }
}
impl<W: Writer> Display for SerError<'_, W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to serialize `{}`, {}", self.type_name, self.reason.description())
    }
}

#[cfg(feature = "std")]
impl<W: Writer + 'static> From<SerError<'_, W>> for std::io::Error {
    fn from(value: SerError<'_, W>) -> Self {
        let value = value.into_owned();
        if let SerErrorKind::Reflect(_) = value.kind() {
            std::io::Error::new(std::io::ErrorKind::Unsupported, value)
        } else {
            std::io::Error::other(value)
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A type of error that can occur during serialization.
pub enum SerErrorKind<W: Writer> {
    /// An error occurred while writing bytes.
    Write(<W as Writer>::Error),
    /// An error occurred during reflection.
    Reflect(ReflectError),
    /// Some other error occurred.
    Other(Box<dyn Error + Send + Sync>),
}

impl<W: Writer> SerErrorKind<W> {
    /// Get a [`str`] with a brief description of this error.
    ///
    /// Because it does not allocate, this is suitable for use in
    /// `const` contexts.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            SerErrorKind::Write(_) => "Write error",
            SerErrorKind::Reflect(_) => "Reflection error",
            SerErrorKind::Other(_) => "Other",
        }
    }

    /// Create an error message for this error.
    #[must_use]
    pub fn as_message(&self) -> Cow<'static, str> {
        match self {
            SerErrorKind::Write(err) => Cow::Owned(format!("Failed to write bytes, {err}")),

            SerErrorKind::Reflect(reflect) => match reflect {
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

            SerErrorKind::Other(err) => Cow::Owned(err.to_string()),
        }
    }
}

impl<T: Writer> Error for SerErrorKind<T> {}
impl<T: Writer> Debug for SerErrorKind<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Write(arg0) => f.debug_tuple("Write").field(arg0).finish(),
            Self::Reflect(arg0) => f.debug_tuple("Reflect").field(arg0).finish(),
            Self::Other(arg0) => f.debug_tuple("Other").field(arg0).finish(),
        }
    }
}
impl<T: Writer> Display for SerErrorKind<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.write_str(&self.as_message()) }
}

impl<W: Writer> From<ReflectError> for SerErrorKind<W> {
    fn from(err: ReflectError) -> Self { SerErrorKind::Reflect(err) }
}
impl<T: Error + Send + Sync + 'static, W: Writer> From<Box<T>> for SerErrorKind<W> {
    fn from(err: Box<T>) -> Self { SerErrorKind::Other(err) }
}
