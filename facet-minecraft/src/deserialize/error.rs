//! TODO
#![allow(
    clippy::new_without_default,
    unknown_lints,
    renamed_and_removed_lints,
    elidable_lifetime_names,
    reason = "Temporary"
)]

use alloc::boxed::Box;
use core::{
    error::Error,
    fmt::{self, Debug, Display},
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

impl Debug for DeserializeError<'_> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl Display for DeserializeError<'_> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl From<DeserializeError<'_>> for Box<dyn Error + Send + Sync> {
    fn from(_: DeserializeError<'_>) -> Self { todo!() }
}

impl<'facet> From<DeserializeIterError<'facet>> for DeserializeError<'facet> {
    fn from(_: DeserializeIterError<'facet>) -> Self {
        Self { _phantom: core::marker::PhantomData }
    }
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
