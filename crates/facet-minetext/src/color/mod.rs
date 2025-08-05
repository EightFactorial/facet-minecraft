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
    /// Get the color as a [`u32`].
    ///
    /// # Errors
    /// Returns an error if the color is a custom color that cannot be parsed.
    pub const fn as_u32(&self) -> Result<u32, custom::ParseColorError> {
        match self {
            TextColor::Preset(color) => Ok(color.fg_u32()),
            TextColor::Custom(color) => color.try_as_u32(),
        }
    }

    /// Get the color as a [`DynColors`](owo_colors::DynColors).
    ///
    /// # Errors
    /// Returns an error if the color is a custom color that cannot be parsed.
    pub const fn as_dyncolor(&self) -> Result<owo_colors::DynColors, custom::ParseColorError> {
        match self {
            TextColor::Preset(color) => Ok(color.fg()),
            TextColor::Custom(color) => color.try_as_dyncolor(),
        }
    }

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
