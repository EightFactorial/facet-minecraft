use core::{
    error::Error,
    fmt::{Debug, Display},
};

use facet_reflect::Peek;

/// An error that occurred during serialization.
pub enum SerializeError<'mem, 'facet, T> {
    /// An error that occurred while serializing a value.
    InvalidType(SerializeErrorData<'mem, 'facet>),
    /// An error that occurred while writing a value.
    WriteError(T),
}

/// Data associated with a serialization error.
#[allow(unreachable_pub)]
pub struct SerializeErrorData<'mem, 'facet> {
    /// The value that caused the error
    pub value: Option<Peek<'mem, 'facet>>,
    /// The reason for the error.
    pub reason: &'static str,
    /// The source identifier.
    pub source: &'static str,
}

impl<'mem, 'facet, T> SerializeError<'mem, 'facet, T> {
    /// Create a new [`SerializeError`] indicating a value
    /// and the reason for the error.
    #[must_use]
    pub(super) fn new(value: Peek<'mem, 'facet>, reason: &'static str) -> Self {
        SerializeError::InvalidType(SerializeErrorData {
            reason,
            value: Some(value),
            source: crate::ERROR_SOURCE,
        })
    }

    /// Create a new [`SerializeError`] indicating the reason for the error.
    #[must_use]
    pub(super) fn new_reason(reason: &'static str) -> Self {
        SerializeError::InvalidType(SerializeErrorData {
            reason,
            value: None,
            source: crate::ERROR_SOURCE,
        })
    }

    /// Drop the inner data, unbinding the error from the lifetime of the value.
    #[must_use]
    pub(super) fn into_owned<'owned>(self) -> SerializeError<'owned, 'facet, T> {
        match self {
            SerializeError::WriteError(err) => SerializeError::WriteError(err),
            SerializeError::InvalidType(data) => SerializeError::InvalidType(SerializeErrorData {
                value: None,
                reason: data.reason,
                source: data.source,
            }),
        }
    }
}

impl<T: Display + Error> Error for SerializeError<'_, '_, T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SerializeError::InvalidType(..) => None,
            SerializeError::WriteError(err) => err.source(),
        }
    }
}

impl<T> From<T> for SerializeError<'_, '_, T> {
    fn from(err: T) -> Self { SerializeError::WriteError(err) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(not(feature = "rich-diagnostics"))]
impl<T: Display> Debug for SerializeError<'_, '_, T> {
    #[inline(always)]
    #[expect(clippy::inline_always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { Display::fmt(self, f) }
}

#[cfg(not(feature = "rich-diagnostics"))]
impl<T: Display> Display for SerializeError<'_, '_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerializeError::WriteError(err) => Display::fmt(err, f),
            SerializeError::InvalidType(data) => {
                use facet::TypeNameOpts;

                write!(f, "Cannot serialize type, ")?;
                f.write_str(data.reason)?;

                if let Some(value) = data.value {
                    f.write_str(": `")?;
                    value.shape().write_type_name(f, TypeNameOpts::infinite())?;
                    f.write_str("`")?;
                }

                Ok(())
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "rich-diagnostics")]
impl<T: Display> Debug for SerializeError<'_, '_, T> {
    #[inline(always)]
    #[expect(clippy::inline_always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { Display::fmt(self, f) }
}

#[cfg(feature = "rich-diagnostics")]
impl<T: Display> Display for SerializeError<'_, '_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerializeError::WriteError(err) => Display::fmt(err, f),
            SerializeError::InvalidType(data) => {
                use facet::TypeNameOpts;

                write!(f, "Cannot serialize type, ")?;
                f.write_str(data.reason)?;

                if let Some(value) = data.value {
                    f.write_str(": `")?;
                    value.shape().write_type_name(f, TypeNameOpts::infinite())?;
                    f.write_str("`")?;
                }

                Ok(())
            }
        }
    }
}
