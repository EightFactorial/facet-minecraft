#![allow(missing_docs)]
#![expect(clippy::ref_option_ref)]

use crate::color::{TextColor, preset::MineColors};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TextStyle<'a> {
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub font: Option<&'a str>,
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub color: Option<TextColor<'a>>,
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub bold: Option<bool>,
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub italic: Option<bool>,
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub underlined: Option<bool>,
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub strikethrough: Option<bool>,
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub obfuscated: Option<bool>,
}

impl TextStyle<'_> {
    /// A [`TextStyle`] with no properties set.
    ///
    /// Transparently passes through any styling
    /// from the parent component, if any.
    pub const NONE: TextStyle<'static> = TextStyle {
        font: None,
        color: None,
        bold: None,
        italic: None,
        underlined: None,
        strikethrough: None,
        obfuscated: None,
    };
    /// The default [`TextStyle`] used for the root component
    /// of a text component tree.
    pub const ROOT: TextStyle<'static> = TextStyle {
        font: Some("minecraft:text"),
        color: Some(TextColor::Preset(MineColors::White)),
        bold: Some(false),
        italic: Some(false),
        underlined: Some(false),
        strikethrough: Some(false),
        obfuscated: Some(false),
    };

    /// Returns `true` if the style has no properties set.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.font.is_none()
            && self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
    }
}

// -------------------------------------------------------------------------------------------------
