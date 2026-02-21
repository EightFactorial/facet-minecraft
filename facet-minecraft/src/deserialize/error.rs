//! TODO
#![allow(
    clippy::new_without_default,
    unknown_lints,
    renamed_and_removed_lints,
    elidable_lifetime_names,
    reason = "Temporary"
)]

use core::{
    error::Error,
    fmt::{self, Debug, Display},
    str::Utf8Error,
};

use facet_reflect::ReflectError;

/// An error that can occur during deserialization.
pub struct DeserializeError<'facet> {
    _phantom: core::marker::PhantomData<&'facet ()>,
}

impl<'facet> DeserializeError<'facet> {
    /// Create a new [`DeserializeError`].
    #[must_use]
    pub fn new() -> Self { Self { _phantom: core::marker::PhantomData } }
}

impl Error for DeserializeError<'_> {}
impl Display for DeserializeError<'_> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl Debug for DeserializeError<'_> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl<'facet> From<ReflectError> for DeserializeError<'facet> {
    fn from(_: ReflectError) -> Self { todo!() }
}
#[cfg(feature = "std")]
impl From<std::io::Error> for DeserializeError<'_> {
    fn from(_: std::io::Error) -> Self { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur during deserialization.
#[derive(Debug)]
pub struct DeserializeIterError<'facet> {
    _phantom: core::marker::PhantomData<&'facet ()>,
}

impl<'facet> DeserializeIterError<'facet> {
    /// Create a new [`DeserializeIterError`].
    #[must_use]
    pub fn new() -> Self { Self { _phantom: core::marker::PhantomData } }
}

impl From<ReflectError> for DeserializeIterError<'_> {
    fn from(_: ReflectError) -> Self { Self::new() }
}
impl From<Utf8Error> for DeserializeIterError<'_> {
    fn from(_: Utf8Error) -> Self { Self::new() }
}

impl<'facet> From<DeserializeIterError<'facet>> for DeserializeError<'facet> {
    fn from(_: DeserializeIterError<'facet>) -> Self {
        Self { _phantom: core::marker::PhantomData }
    }
}
