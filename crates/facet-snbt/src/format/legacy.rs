use super::SnbtFormat;
use crate::snbt::Snbt;

/// The legacy SNBT format.
///
/// Used in Minecraft versions before 1.21.5,
/// but still compatible with later versions.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, facet_macros::Facet)]
pub struct Legacy;

impl<'a> SnbtFormat<'a> for Legacy {
    type Inner = &'a str;
}

/// The legacy SNBT format.
///
/// Used in Minecraft versions before 1.21.5,
/// but still compatible with later versions.
pub type LegacySnbt<'a> = Snbt<'a, Legacy>;

// -------------------------------------------------------------------------------------------------
