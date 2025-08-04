#![allow(clippy::ref_option_ref, missing_docs)]

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

#[cfg(feature = "alloc")]
use crate::color::custom::CustomColor;
use crate::color::{TextColor, preset::MineColors};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TextStyle<'a> {
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub font: Option<&'a str>,
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "facet", facet(default, skip_serializing_if = Option::is_none))]
    pub font: Option<Cow<'a, str>>,
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
        #[cfg(not(feature = "alloc"))]
        font: Some("minecraft:text"),
        #[cfg(feature = "alloc")]
        font: Some(Cow::Borrowed("minecraft:text")),
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
    #[cfg(not(feature = "alloc"))]
    pub const fn with_font<'b>(self, font: &'b str) -> TextStyle<'b>
    where 'a: 'b {
        TextStyle {
            font: Some(font),
            color: self.color,
            bold: self.bold,
            italic: self.italic,
            underlined: self.underlined,
            strikethrough: self.strikethrough,
            obfuscated: self.obfuscated,
        }
    }

    /// Set the font of the [`TextStyle`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn with_font<'b>(&'b self, font: Cow<'b, str>) -> TextStyle<'b>
    where 'a: 'b {
        TextStyle {
            font: Some(font),
            color: match &self.color {
                Some(TextColor::Custom(color)) => {
                    Some(TextColor::Custom(CustomColor::new(color.as_str())))
                }
                Some(TextColor::Preset(color)) => Some(TextColor::Preset(*color)),
                None => None,
            },
            bold: self.bold,
            italic: self.italic,
            underlined: self.underlined,
            strikethrough: self.strikethrough,
            obfuscated: self.obfuscated,
        }
    }

    /// Set the color of the [`TextStyle`].
    #[inline]
    #[must_use]
    #[cfg(not(feature = "alloc"))]
    pub const fn with_color<'b>(&'b self, color: TextColor<'b>) -> TextStyle<'b>
    where 'a: 'b {
        TextStyle {
            font: self.font,
            color: Some(color),
            bold: self.bold,
            italic: self.italic,
            underlined: self.underlined,
            strikethrough: self.strikethrough,
            obfuscated: self.obfuscated,
        }
    }

    /// Set the color of the [`TextStyle`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn with_color<'b>(&'b self, color: TextColor<'b>) -> TextStyle<'b>
    where 'a: 'b {
        TextStyle {
            font: match &self.font {
                Some(Cow::Owned(font)) => Some(Cow::Borrowed(font.as_str())),
                Some(Cow::Borrowed(font)) => Some(Cow::Borrowed(font)),
                None => None,
            },
            color: Some(color),
            bold: self.bold,
            italic: self.italic,
            underlined: self.underlined,
            strikethrough: self.strikethrough,
            obfuscated: self.obfuscated,
        }
    }

    /// Set the bold property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_bold(mut self, bold: bool) -> TextStyle<'a> {
        self.bold = Some(bold);
        self
    }

    /// Set the italic property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_italic(mut self, italic: bool) -> TextStyle<'a> {
        self.italic = Some(italic);
        self
    }

    /// Set the underline property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_underline(mut self, underline: bool) -> TextStyle<'a> {
        self.underlined = Some(underline);
        self
    }

    /// Set the strikethrough property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_strikethrough(mut self, strikethrough: bool) -> TextStyle<'a> {
        self.strikethrough = Some(strikethrough);
        self
    }

    /// Set the obfuscated property of the [`TextStyle`].
    #[inline]
    #[must_use]
    pub const fn with_obfuscation(mut self, obfuscation: bool) -> TextStyle<'a> {
        self.obfuscated = Some(obfuscation);
        self
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
    /// assert_eq!(CHILD.inherit(&ROOT), ROOT);
    ///
    /// const BOLD: TextStyle<'static> = TextStyle::NONE.with_bold(true);
    ///
    /// // `BOLD` only has the bold property set,
    /// // so it will be identical to `ROOT` *except* that it will be bold.
    /// assert_ne!(BOLD.inherit(&ROOT), ROOT);
    /// assert_eq!(BOLD.inherit(&ROOT), ROOT.with_bold(true));
    /// ```
    #[must_use]
    pub const fn inherit<'b>(&'b self, parent: &'b TextStyle<'_>) -> TextStyle<'b>
    where 'a: 'b {
        /// A `const` equivalent to `Option::or`
        const fn or<T: Copy>(a: Option<T>, b: Option<T>) -> Option<T> {
            match (a, b) {
                (Some(val), _) | (None, Some(val)) => Some(val),
                (None, None) => None,
            }
        }

        TextStyle {
            font: match (&self.font, &parent.font) {
                (Some(Cow::Owned(font)), _) | (None, Some(Cow::Owned(font))) => {
                    Some(Cow::Borrowed(font.as_str()))
                }
                (Some(Cow::Borrowed(font)), _) | (None, Some(Cow::Borrowed(font))) => {
                    Some(Cow::Borrowed(font))
                }
                _ => None,
            },
            color: match (&self.color, &parent.color) {
                (Some(TextColor::Custom(color)), _) | (None, Some(TextColor::Custom(color))) => {
                    Some(TextColor::Custom(CustomColor::new(color.as_str())))
                }
                (Some(TextColor::Preset(color)), _) | (None, Some(TextColor::Preset(color))) => {
                    Some(TextColor::Preset(*color))
                }
                _ => None,
            },
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
    /// assert_eq!(CHILD.diff(&ROOT), ROOT);
    ///
    /// const BOLD: TextStyle<'static> = TextStyle::ROOT.with_bold(true);
    ///
    /// // `BOLD` is identical except the bold property set to true,
    /// // while `ROOT` has it set to false. The result will only contain the bold property.
    /// assert_ne!(BOLD.diff(&ROOT), ROOT);
    /// assert_eq!(BOLD.diff(&ROOT), TextStyle::NONE.with_bold(true));
    /// ```
    #[must_use]
    #[expect(clippy::too_many_lines)]
    pub const fn diff<'b>(&'b self, other: &'b TextStyle<'_>) -> TextStyle<'b>
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
        #[cfg(not(feature = "alloc"))]
        const fn str_eq(a: Option<&str>, b: Option<&str>) -> bool {
            match (a, b) {
                (Some(a), Some(b)) => const_str::equal!(a, b),
                (None, None) => true,
                _ => false,
            }
        }

        /// A `const` equivalent to `TextColor::eq`
        #[cfg(not(feature = "alloc"))]
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
            matches!((a, b), (Some(true), Some(true)) | (Some(false), Some(false)) | (None, None))
        }

        TextStyle {
            #[cfg(not(feature = "alloc"))]
            font: or_neq(self.font, other.font, str_eq(self.font, other.font)),
            #[cfg(feature = "alloc")]
            font: match (&self.font, &other.font) {
                (Some(font), Some(other)) => match (font, other) {
                    (Cow::Owned(font), Cow::Owned(other)) => {
                        if const_str::equal!(font.as_str(), other.as_str()) {
                            None
                        } else {
                            Some(Cow::Borrowed(font.as_str()))
                        }
                    }
                    (Cow::Borrowed(font), Cow::Borrowed(other)) => {
                        if const_str::equal!(*font, *other) {
                            None
                        } else {
                            Some(Cow::Borrowed(font))
                        }
                    }
                    (Cow::Owned(font), Cow::Borrowed(other)) => {
                        if const_str::equal!(font.as_str(), *other) {
                            None
                        } else {
                            Some(Cow::Borrowed(font.as_str()))
                        }
                    }
                    (Cow::Borrowed(font), Cow::Owned(other)) => {
                        if const_str::equal!(*font, other.as_str()) {
                            None
                        } else {
                            Some(Cow::Borrowed(font))
                        }
                    }
                },
                (Some(font), None) | (None, Some(font)) => match font {
                    Cow::Owned(font) => Some(Cow::Borrowed(font.as_str())),
                    Cow::Borrowed(font) => Some(Cow::Borrowed(font)),
                },
                _ => None,
            },
            #[cfg(not(feature = "alloc"))]
            color: or_neq(self.color, other.color, color_eq(self.color, other.color)),
            #[cfg(feature = "alloc")]
            color: match (&self.color, &other.color) {
                (Some(TextColor::Custom(color)), Some(TextColor::Custom(other))) => {
                    if const_str::equal!(color.as_str(), other.as_str()) {
                        None
                    } else {
                        Some(TextColor::Custom(CustomColor::new(color.as_str())))
                    }
                }
                (Some(TextColor::Preset(color)), Some(TextColor::Preset(other))) => {
                    if *color as u8 == *other as u8 {
                        None
                    } else {
                        Some(TextColor::Preset(*color))
                    }
                }
                (Some(TextColor::Custom(color)), Some(TextColor::Preset(_))) => {
                    Some(TextColor::Custom(CustomColor::new(color.as_str())))
                }
                (Some(TextColor::Preset(color)), Some(TextColor::Custom(_))) => {
                    Some(TextColor::Preset(*color))
                }
                (Some(color), None) | (None, Some(color)) => match color {
                    TextColor::Custom(color) => {
                        Some(TextColor::Custom(CustomColor::new(color.as_str())))
                    }
                    TextColor::Preset(color) => Some(TextColor::Preset(*color)),
                },
                (None, None) => None,
            },
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
