use alloc::borrow::Cow;

use super::{LegacySnbt, SnbtFormat};
use crate::snbt::Snbt;

/// The modern SNBT format.
///
/// Used in Minecraft versions 1.21.5 and later,
/// supports more formatting options and data types.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, facet_macros::Facet)]
pub struct Modern;

/// The modern SNBT format.
///
/// Used in Minecraft versions 1.21.5 and later,
/// supports more formatting options and data types.
pub type ModernSnbt<'a> = Snbt<'a, Modern>;

impl LegacySnbt<'_> {
    /// Upgrade a [`LegacySnbt`] into a [`ModernSnbt`].
    ///
    /// This method is essentially a reborrow since the
    /// new format is a superset of the old one.
    #[must_use]
    pub const fn upgrade(&self) -> ModernSnbt<'_> {
        match self.as_inner() {
            Cow::Borrowed(val) => ModernSnbt::new_unchecked(Cow::Borrowed(val)),
            Cow::Owned(val) => ModernSnbt::new_unchecked(Cow::Borrowed(val.as_str())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl SnbtFormat for Modern {}
