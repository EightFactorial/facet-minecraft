//! TODO

pub mod custom;
pub mod preset;

// TODO: Add `facet(untagged)` when it is implemented.
/// A color used in text rendering.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum TextColor<'a> {
    /// One of a predefined set of named colors.
    Preset(preset::MineColors),
    /// A custom color defined by a hexadecimal string.
    Custom(custom::CustomColor<'a>),
}
