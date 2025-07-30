//! TODO

use core::marker::PhantomData;

use crate::format::SnbtFormat;

/// A Stringified NBT value.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet_macros::Facet)]
pub struct Snbt<'a, F: SnbtFormat<'a>>(F::Inner, PhantomData<F>);

impl<'a, F: SnbtFormat<'a>> Snbt<'a, F> {
    /// Create a new [`Snbt`] from a string.
    ///
    /// # Warning
    /// This function does not validate the SNBT string!
    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked(content: F::Inner) -> Self { Self(content, PhantomData) }

    /// Get a reference to the inner SNBT string.
    #[inline]
    #[must_use]
    pub const fn as_inner(&self) -> &F::Inner { &self.0 }
}

// -------------------------------------------------------------------------------------------------

impl<'a, F: SnbtFormat<'a>> core::convert::AsRef<F::Inner> for Snbt<'a, F> {
    fn as_ref(&self) -> &F::Inner { &self.0 }
}
impl<'a, F: SnbtFormat<'a>> core::convert::AsMut<F::Inner> for Snbt<'a, F> {
    fn as_mut(&mut self) -> &mut F::Inner { &mut self.0 }
}

impl<'a, F: SnbtFormat<'a>> core::ops::Deref for Snbt<'a, F> {
    type Target = F::Inner;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<'a, F: SnbtFormat<'a>> core::ops::DerefMut for Snbt<'a, F> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
