//! TODO

use alloc::borrow::Cow;
use core::marker::PhantomData;

use crate::format::SnbtFormat;

/// A Stringified NBT value.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet_macros::Facet)]
pub struct Snbt<'a, F: SnbtFormat>(Cow<'a, str>, PhantomData<F>);

impl<'a, F: SnbtFormat> Snbt<'a, F> {
    /// Create a new [`Snbt`] from a string.
    ///
    /// # Warning
    /// This function does not validate the SNBT string!
    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked(content: Cow<'a, str>) -> Self { Self(content, PhantomData) }

    /// Get a reference to the inner SNBT string.
    #[inline]
    #[must_use]
    pub const fn as_inner(&self) -> &Cow<'a, str> { &self.0 }

    /// Get a reference to the inner SNBT string.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match &self.0 {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => s.as_str(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, F: SnbtFormat> core::convert::AsRef<Cow<'a, str>> for Snbt<'a, F> {
    fn as_ref(&self) -> &Cow<'a, str> { &self.0 }
}
impl<'a, F: SnbtFormat> core::convert::AsMut<Cow<'a, str>> for Snbt<'a, F> {
    fn as_mut(&mut self) -> &mut Cow<'a, str> { &mut self.0 }
}

impl<F: SnbtFormat> core::convert::AsRef<str> for Snbt<'_, F> {
    fn as_ref(&self) -> &str { &self.0 }
}
impl<F: SnbtFormat> core::convert::AsMut<str> for Snbt<'_, F> {
    fn as_mut(&mut self) -> &mut str { self.0.to_mut() }
}

impl<'a, F: SnbtFormat> core::borrow::Borrow<Cow<'a, str>> for Snbt<'a, F> {
    fn borrow(&self) -> &Cow<'a, str> { &self.0 }
}
impl<'a, F: SnbtFormat> core::borrow::BorrowMut<Cow<'a, str>> for Snbt<'a, F> {
    fn borrow_mut(&mut self) -> &mut Cow<'a, str> { &mut self.0 }
}

impl<F: SnbtFormat> core::borrow::Borrow<str> for Snbt<'_, F> {
    fn borrow(&self) -> &str { &self.0 }
}

impl<'a, F: SnbtFormat> core::ops::Deref for Snbt<'a, F> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<F: SnbtFormat> core::ops::DerefMut for Snbt<'_, F> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
