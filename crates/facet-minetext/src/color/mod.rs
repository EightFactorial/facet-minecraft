//! TODO

pub mod custom;
pub mod preset;

// TODO: Add `facet(untagged)` when it is implemented.
/// A color used in text rendering.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub enum TextColor<'a> {
    /// One of a predefined set of named colors.
    Preset(preset::MineColors),
    /// A custom color defined by a hexadecimal string.
    Custom(custom::CustomColor<'a>),
}

impl TextColor<'_> {
    /// A `const` equivalent to [`PartialEq`].
    ///
    /// Should only be used in `const` contexts.
    #[must_use]
    pub const fn const_eq(&self, other: &TextColor<'_>) -> bool {
        match (self, other) {
            (TextColor::Preset(p1), TextColor::Preset(p2)) => *p1 as u8 == *p2 as u8,
            (TextColor::Custom(c1), TextColor::Custom(c2)) => c1.const_eq(c2),
            _ => false,
        }
    }

    /// Reborrow a reference as an owned [`TextColor`].
    #[must_use]
    pub const fn reborrow(&self) -> TextColor<'_> {
        match self {
            TextColor::Preset(p) => TextColor::Preset(*p),
            TextColor::Custom(c) => TextColor::Custom(c.reborrow()),
        }
    }
}
