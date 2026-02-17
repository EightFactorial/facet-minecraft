use alloc::borrow::Cow;
use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use facet::{Facet, Shape};
use facet_format::ScalarValue;

/// An error that occurred during serialization.
#[derive(Debug)]
pub struct SerializeError {
    kind: SerializeErrorKind,
}

impl SerializeError {
    /// Create a new [`SerializeError`].
    #[must_use]
    pub const fn new(kind: SerializeErrorKind) -> Self { Self { kind } }

    /// Create a new [`SerializeError`] for an unsupported type.
    #[inline]
    #[must_use]
    pub const fn unsupported_type<'a, T: Facet<'a>>() -> Self { Self::unsupported_shape(T::SHAPE) }

    /// Create a new [`SerializeError`] for an unsupported shape.
    #[inline]
    #[must_use]
    pub const fn unsupported_shape(shape: &'static Shape) -> Self {
        Self::new(SerializeErrorKind::UnsupportedType(shape))
    }

    /// Create a new [`SerializeError`] for an attempt to variable-length
    /// serialize an unsupported type.
    #[inline]
    #[must_use]
    pub const fn variable_length(shape: &Shape) -> Self {
        Self::new(SerializeErrorKind::VariableLength(shape.type_identifier))
    }

    /// Create a new [`SerializeError`] for an attempt to variable-length
    /// serialize a [`ScalarValue`] that does not support it.
    #[must_use]
    pub const fn variable_length_scalar(scalar: &ScalarValue) -> Self {
        match scalar {
            ScalarValue::Unit | ScalarValue::Null => Self::variable_length(<()>::SHAPE),
            ScalarValue::Bool(_) => Self::variable_length(bool::SHAPE),
            ScalarValue::Char(_) => Self::variable_length(char::SHAPE),
            ScalarValue::I64(_) => Self::variable_length(i64::SHAPE),
            ScalarValue::U64(_) => Self::variable_length(u64::SHAPE),
            ScalarValue::I128(_) => Self::variable_length(i128::SHAPE),
            ScalarValue::U128(_) => Self::variable_length(u128::SHAPE),
            ScalarValue::F64(_) => Self::variable_length(f64::SHAPE),
            ScalarValue::Str(_) => Self::variable_length(Cow::<str>::SHAPE),
            ScalarValue::Bytes(_) => Self::variable_length(Cow::<[u8]>::SHAPE),
        }
    }

    /// Get the kind of serialization error.
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &SerializeErrorKind { &self.kind }
}

/// The type of serialization error.
#[derive(Debug)]
pub enum SerializeErrorKind {
    /// An error occurred while writing to the buffer.
    BufferError,
    /// Could not get the discriminant of an enum variant.
    DiscriminantMissing,

    /// Attempted to serialize a type that is not supported.
    UnsupportedType(&'static Shape),
    /// Attempted to variable-length serialize a type that does not support it.
    VariableLength(&'static str),

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
