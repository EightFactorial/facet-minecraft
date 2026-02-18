//! TODO

use facet_reflect::ReflectError;

/// An error that can occur during serialization.
#[derive(Debug)]
pub struct SerializeIterError<'mem, 'facet> {
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

// -------------------------------------------------------------------------------------------------
