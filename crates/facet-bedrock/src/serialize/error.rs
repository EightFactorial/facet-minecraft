use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An error that occurred during serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializeError {
    kind: SerializeErrorKind,
}

/// The type of serialization error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializeErrorKind {}

// -------------------------------------------------------------------------------------------------

impl Error for SerializeError {}
impl Display for SerializeError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result { todo!() }
}
