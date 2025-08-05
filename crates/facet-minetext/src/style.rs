#![allow(clippy::ref_option_ref, missing_docs)]

use alloc::borrow::Cow;

use crate::color::{
    TextColor,
    preset::{MineColor, MineColors, White},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct TextStyle<'a> {
    #[facet(default, skip_serializing_if = Option::is_none)]
    pub font: Option<Cow<'a, str>>,
    #[facet(default, skip_serializing_if = Option::is_none)]
    pub color: Option<TextColor<'a>>,
    #[facet(default, skip_serializing_if = Option::is_none)]
    pub bold: Option<bool>,
    #[facet(default, skip_serializing_if = Option::is_none)]
    pub italic: Option<bool>,
    #[facet(default, skip_serializing_if = Option::is_none)]
    pub underlined: Option<bool>,
    #[facet(default, skip_serializing_if = Option::is_none)]
    pub strikethrough: Option<bool>,
    #[facet(default, skip_serializing_if = Option::is_none)]
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

    /// Reborrow a reference to an owned [`TextStyle`].
    #[must_use]
    pub const fn reborrow(&self) -> TextStyle<'_> {
        TextStyle {
            font: match &self.font {
                Some(Cow::Owned(font)) => Some(Cow::Borrowed(font.as_str())),
                Some(Cow::Borrowed(font)) => Some(Cow::Borrowed(font)),
                None => None,
            },
            color: match &self.color {
                Some(color) => Some(color.reborrow()),
                None => None,
            },
            bold: self.bold,
            italic: self.italic,
            underlined: self.underlined,
            strikethrough: self.strikethrough,
            obfuscated: self.obfuscated,
        }
    }

    /// Set the font of the [`TextStyle`].
    #[must_use]
    pub const fn with_font<'b>(&'b self, font: Cow<'b, str>) -> TextStyle<'b>
    where 'a: 'b {
        TextStyle {
            font: Some(font),
            color: match &self.color {
                Some(color) => Some(color.reborrow()),
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
    #[must_use]
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
            font: match or(self.font.as_ref(), parent.font.as_ref()) {
                Some(Cow::Owned(font)) => Some(Cow::Borrowed(font.as_str())),
                Some(Cow::Borrowed(font)) => Some(Cow::Borrowed(font)),
                None => None,
            },
            color: match or(self.color.as_ref(), parent.color.as_ref()) {
                Some(color) => Some(color.reborrow()),
                None => None,
            },
            bold: or(self.bold, parent.bold),
            italic: or(self.italic, parent.italic),
            underlined: or(self.underlined, parent.underlined),
            strikethrough: or(self.strikethrough, parent.strikethrough),
            obfuscated: or(self.obfuscated, parent.obfuscated),
        }
    }

    /// Create a new [`TextStyle`] that inherits unset properties from a parent
    /// style.
    ///
    /// This is similar to [`TextStyle::inherit`], but allows for a more
    /// flexible lifetime relationship between the parent and child styles.
    #[must_use]
    pub fn inherit_owned(&self, parent: &TextStyle<'a>) -> TextStyle<'a> {
        TextStyle {
            font: self.font.as_ref().or(parent.font.as_ref()).cloned(),
            color: self.color.as_ref().or(parent.color.as_ref()).cloned(),
            bold: self.bold.or(parent.bold),
            italic: self.italic.or(parent.italic),
            underlined: self.underlined.or(parent.underlined),
            strikethrough: self.strikethrough.or(parent.strikethrough),
            obfuscated: self.obfuscated.or(parent.obfuscated),
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
    pub const fn diff<'b>(&'b self, other: &'b TextStyle<'_>) -> TextStyle<'b>
    where 'a: 'b {
        /// Return `a` if not equal, either if only one value is set, or `None`.
        const fn or_neq<T: Copy>(a: Option<T>, b: Option<T>, eq: bool) -> Option<T> {
            match (a, b, eq) {
                (Some(val), Some(_), false) | (Some(val), None, _) | (None, Some(val), _) => {
                    Some(val)
                }
                _ => None,
            }
        }

        /// Return `a` if not equal, either if only one value is set, or `None`.
        #[expect(clippy::ptr_arg)]
        const fn cow_or_neq<'c>(a: &'c Cow<'_, str>, b: &'c Cow<'_, str>) -> Option<Cow<'c, str>> {
            match (a, b) {
                (Cow::Owned(a), Cow::Owned(b)) => {
                    match or_neq(Some(a), Some(b), const_str::equal!(a.as_str(), b.as_str())) {
                        Some(val) => Some(Cow::Borrowed(val.as_str())),
                        None => None,
                    }
                }
                (Cow::Borrowed(a), Cow::Borrowed(b)) => {
                    match or_neq(Some(*a), Some(*b), const_str::equal!(*a, *b)) {
                        Some(val) => Some(Cow::Borrowed(val)),
                        None => None,
                    }
                }
                (Cow::Owned(a), Cow::Borrowed(b)) => {
                    match or_neq(Some(a.as_str()), Some(*b), const_str::equal!(a.as_str(), *b)) {
                        Some(val) => Some(Cow::Borrowed(val)),
                        None => None,
                    }
                }
                (Cow::Borrowed(a), Cow::Owned(b)) => {
                    match or_neq(Some(*a), Some(b.as_str()), const_str::equal!(*a, b.as_str())) {
                        Some(val) => Some(Cow::Borrowed(val)),
                        None => None,
                    }
                }
            }
        }

        /// A `const` equivalent to `bool::eq`
        const fn bool_eq(a: Option<bool>, b: Option<bool>) -> bool {
            matches!((a, b), (Some(true), Some(true)) | (Some(false), Some(false)) | (None, None))
        }

        TextStyle {
            font: match (&self.font, &other.font) {
                (Some(font), Some(other)) => cow_or_neq(font, other),
                (Some(font), None) | (None, Some(font)) => match font {
                    Cow::Owned(font) => Some(Cow::Borrowed(font.as_str())),
                    Cow::Borrowed(font) => Some(Cow::Borrowed(font)),
                },
                _ => None,
            },
            color: match (&self.color, &other.color) {
                (Some(color), Some(other)) if !color.const_eq(other) => Some(color.reborrow()),
                (Some(color), None) | (None, Some(color)) => Some(color.reborrow()),
                _ => None,
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

    /// Create a new [`TextStyle`] that only contains properties that differ
    /// from another style.
    ///
    /// This is similar to [`TextStyle::diff`], but allows for a more
    /// flexible lifetime relationship between the two styles.
    #[must_use]
    pub fn diff_owned(&self, other: &TextStyle<'a>) -> TextStyle<'a> {
        /// Return `a` if not equal, either if only one value is set, or `None`.
        fn or_neq<T: PartialEq>(a: Option<T>, b: Option<T>) -> Option<T> {
            match (a, b) {
                (Some(val), Some(other)) if val != other => Some(val),
                (Some(val), None) | (None, Some(val)) => Some(val),
                _ => None,
            }
        }

        TextStyle {
            font: or_neq(self.font.as_ref(), other.font.as_ref()).cloned(),
            color: or_neq(self.color.as_ref(), other.color.as_ref()).cloned(),
            bold: or_neq(self.bold, other.bold),
            italic: or_neq(self.italic, other.italic),
            underlined: or_neq(self.underlined, other.underlined),
            strikethrough: or_neq(self.strikethrough, other.strikethrough),
            obfuscated: or_neq(self.obfuscated, other.obfuscated),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl From<TextStyle<'_>> for owo_colors::Style {
    #[inline]
    fn from(value: TextStyle<'_>) -> Self { value.to_owo_style() }
}
impl From<&TextStyle<'_>> for owo_colors::Style {
    fn from(value: &TextStyle<'_>) -> Self { value.to_owo_style() }
}

impl TextStyle<'_> {
    /// Create a [`owo_colors::Style`] from the [`TextStyle`].
    #[must_use]
    pub const fn to_owo_style(&self) -> owo_colors::Style {
        let mut style = owo_colors::Style::new();

        match self.color.as_ref() {
            Some(color) => match color.as_u32() {
                Ok(value) => {
                    style = style.truecolor(
                        ((value >> 16) & 0xFF) as u8,
                        ((value >> 8) & 0xFF) as u8,
                        (value & 0xFF) as u8,
                    );
                }
                Err(..) => style = style.fg::<<White as MineColor>::Foreground>(),
            },
            None => style = style.fg::<<White as MineColor>::Foreground>(),
        }

        if let Some(true) = self.bold {
            style = style.bold();
        }
        if let Some(true) = self.italic {
            style = style.italic();
        }
        if let Some(true) = self.underlined {
            style = style.underline();
        }
        if let Some(true) = self.strikethrough {
            style = style.strikethrough();
        }

        style
    }
}
