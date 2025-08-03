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

impl<'a> TextStyle<'a> {
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

    /// Set the font of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_font<'b>(self, font: &'b str) -> TextStyle<'b>
    where 'a: 'b {
        TextStyle { font: Some(font), ..self }
    }

    /// Set the color of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_color<'b>(self, color: TextColor<'b>) -> TextStyle<'b>
    where 'a: 'b {
        TextStyle { color: Some(color), ..self }
    }

    /// Set the bold property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_bold(self, bold: bool) -> TextStyle<'a> {
        TextStyle { bold: Some(bold), ..self }
    }

    /// Set the italic property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_italic(self, italic: bool) -> TextStyle<'a> {
        TextStyle { italic: Some(italic), ..self }
    }

    /// Set the underline property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_underline(self, underline: bool) -> TextStyle<'a> {
        TextStyle { underlined: Some(underline), ..self }
    }

    /// Set the strikethrough property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_strikethrough(self, strikethrough: bool) -> TextStyle<'a> {
        TextStyle { strikethrough: Some(strikethrough), ..self }
    }

    /// Set the obfuscated property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_obfuscation(self, obfuscation: bool) -> TextStyle<'a> {
        TextStyle { obfuscated: Some(obfuscation), ..self }
    }

    /// Create a new [`TextStyle`] that inherits unset properties from a parent
    /// style.
    ///
    /// # Examples
    /// ```rust
    /// use facet_minetext::prelude::*;
    ///
    /// const ROOT: TextStyle<'static> = TextStyle::ROOT;
    /// const CHILD: TextStyle<'static> = TextStyle::NONE;
    ///
    /// // `NONE` has no properties, so it will inherit everything from `ROOT`.
    /// assert_eq!(CHILD.inherit(ROOT), ROOT);
    ///
    /// const BOLD: TextStyle<'static> = TextStyle::NONE.with_bold(true);
    ///
    /// // `BOLD` only has the bold property set,
    /// // so it will be identical to `ROOT` *except* that it will be bold.
    /// assert_ne!(BOLD.inherit(ROOT), ROOT);
    /// assert_eq!(BOLD.inherit(ROOT), ROOT.with_bold(true));
    /// ```
    #[must_use]
    pub const fn inherit<'b>(self, parent: TextStyle<'b>) -> TextStyle<'b>
    where 'a: 'b {
        /// A `const` equivalent to `Option::or`
        const fn or<T: Copy>(a: Option<T>, b: Option<T>) -> Option<T> {
            match (a, b) {
                (Some(val), _) | (None, Some(val)) => Some(val),
                (None, None) => None,
            }
        }

        TextStyle {
            font: or(self.font, parent.font),
            color: or(self.color, parent.color),
            bold: or(self.bold, parent.bold),
            italic: or(self.italic, parent.italic),
            underlined: or(self.underlined, parent.underlined),
            strikethrough: or(self.strikethrough, parent.strikethrough),
            obfuscated: or(self.obfuscated, parent.obfuscated),
        }
    }

    /// Create a new [`TextStyle`] that only contains properties that differ
    /// from another style.
    ///
    /// # Examples
    /// ```rust
    /// use facet_minetext::prelude::*;
    ///
    /// const ROOT: TextStyle<'static> = TextStyle::ROOT;
    /// const CHILD: TextStyle<'static> = TextStyle::NONE;
    ///
    /// // `NONE` has no properties, meaning every property is different.
    /// assert_eq!(CHILD.difference(ROOT), ROOT);
    ///
    /// const BOLD: TextStyle<'static> = TextStyle::ROOT.with_bold(true);
    ///
    /// // `BOLD` is identical except the bold property set to true,
    /// // while `ROOT` has it set to false. The result will only contain the bold property.
    /// assert_ne!(BOLD.difference(ROOT), ROOT);
    /// assert_eq!(BOLD.difference(ROOT), TextStyle::NONE.with_bold(true));
    /// ```
    #[must_use]
    pub const fn difference<'b>(self, other: TextStyle<'b>) -> TextStyle<'b>
    where 'a: 'b {
        /// Return `a` if not equal, either if only one value is set, or `None`.
        const fn or_neq<T: Copy + PartialEq>(a: Option<T>, b: Option<T>, eq: bool) -> Option<T> {
            match (a, b, eq) {
                (Some(val), Some(_), false) | (Some(val), None, _) | (None, Some(val), _) => {
                    Some(val)
                }
                _ => None,
            }
        }

        /// A `const` equivalent to `str::eq`
        const fn str_eq(a: Option<&str>, b: Option<&str>) -> bool {
            match (a, b) {
                (Some(a), Some(b)) => const_str::equal!(a, b),
                (None, None) => true,
                _ => false,
            }
        }

        /// A `const` equivalent to `TextColor::eq`
        const fn color_eq(a: Option<TextColor<'_>>, b: Option<TextColor<'_>>) -> bool {
            match (a, b) {
                (Some(TextColor::Preset(a)), Some(TextColor::Preset(b))) => a as u8 == b as u8,
                (Some(TextColor::Custom(a)), Some(TextColor::Custom(b))) => {
                    str_eq(Some(a.as_str()), Some(b.as_str()))
                }
                (None, None) => true,
                _ => false,
            }
        }

        /// A `const` equivalent to `bool::eq`
        const fn bool_eq(a: Option<bool>, b: Option<bool>) -> bool {
            match (a, b) {
                (Some(true), Some(true)) | (Some(false), Some(false)) | (None, None) => true,
                _ => false,
            }
        }

        TextStyle {
            font: or_neq(self.font, other.font, str_eq(self.font, other.font)),
            color: or_neq(self.color, other.color, color_eq(self.color, other.color)),
            bold: or_neq(self.bold, other.bold, bool_eq(self.bold, other.bold)),
            italic: or_neq(self.italic, other.italic, bool_eq(self.italic, other.italic)),
            underlined: or_neq(
                self.underlined,
                other.underlined,
                bool_eq(self.underlined, other.underlined),
            ),
            strikethrough: or_neq(
                self.strikethrough,
                other.strikethrough,
                bool_eq(self.strikethrough, other.strikethrough),
            ),
            obfuscated: or_neq(
                self.obfuscated,
                other.obfuscated,
                bool_eq(self.obfuscated, other.obfuscated),
            ),
        }
    }
}

// -------------------------------------------------------------------------------------------------
