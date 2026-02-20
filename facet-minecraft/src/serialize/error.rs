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

/// An error that can occur during serialization.
pub struct SerializeError<'mem, 'facet> {
    #[expect(clippy::type_complexity, reason = "Forces invariant lifetime")]
    _invariant: core::marker::PhantomData<(&'mem (), fn(&'facet ()) -> &'facet ())>,
}

impl<'mem, 'facet> SerializeError<'mem, 'facet> {
    /// Create a new [`SerializeError`].
    #[must_use]
    pub fn new() -> Self { Self { _invariant: core::marker::PhantomData } }
}

impl Debug for SerializeError<'_, '_> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl Display for SerializeError<'_, '_> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl From<SerializeError<'_, '_>> for Box<dyn Error + Send + Sync> {
    fn from(_: SerializeError<'_, '_>) -> Self { todo!() }
}

impl<'mem, 'facet> From<SerializeIterError<'mem, 'facet>> for SerializeError<'mem, 'facet> {
    fn from(_: SerializeIterError<'mem, 'facet>) -> Self {
        Self { _invariant: core::marker::PhantomData }
    }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur during serialization.
#[derive(Debug)]
pub struct SerializeIterError<'mem, 'facet> {
    #[expect(clippy::type_complexity, reason = "Forces invariant lifetime")]
    _invariant: core::marker::PhantomData<(&'mem (), fn(&'facet ()) -> &'facet ())>,
}

impl<'mem, 'facet> SerializeIterError<'mem, 'facet> {
    /// Create a new [`SerializeIterError`].
    #[must_use]
    pub fn new() -> Self { Self { _invariant: core::marker::PhantomData } }
}

impl From<ReflectError> for SerializeIterError<'_, '_> {
    fn from(_: ReflectError) -> Self { Self::new() }
}
