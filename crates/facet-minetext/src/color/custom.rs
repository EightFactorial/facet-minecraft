//! Custom color representation.

use alloc::borrow::Cow;

/// A string slice that represents a color.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct CustomColor<'a>(Cow<'a, str>);

impl<'a> CustomColor<'a> {
    /// Create a new [`CustomColor`] from a hexadecimal string slice.
    ///
    /// Supports both 6-character and 7-character strings,
    /// where the latter starts with a `#`.
    ///
    /// # Warning
    /// This constructor does not validate the string.
    #[inline]
    #[must_use]
    pub const fn new(color: &'a str) -> Self { Self(Cow::Borrowed(color)) }

    /// A `const` equivalent to [`PartialEq`].
    ///
    /// Should only be used in `const` contexts.
    #[must_use]
    pub const fn const_eq(&self, other: &CustomColor<'_>) -> bool {
        const_str::equal!(self.as_str(), other.as_str())
    }

    /// Reborrow a reference to an owned [`CustomColor`].
    #[must_use]
    pub const fn reborrow(&self) -> CustomColor<'_> { CustomColor(Cow::Borrowed(self.as_str())) }

    /// Get the color's inner string slice.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match &self.0 {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => s.as_str(),
        }
    }

    /// Try to convert the color string to a [`u32`] value.
    ///
    /// # Errors
    /// Returns an error if the string is not in a valid hexadecimal format.
    ///
    /// # Examples
    /// ```rust
    /// use facet_minetext::prelude::*;
    ///
    /// let color = CustomColor::new("#FF5733");
    /// assert_eq!(color.try_as_u32(), Ok(0xFF5733));
    ///
    /// let color = CustomColor::new("FF5733");
    /// assert_eq!(color.try_as_u32(), Ok(0xFF5733));
    /// ```
    pub const fn try_as_u32(&self) -> Result<u32, ParseColorError> {
        match self.as_str().len() {
            6 => match u32::from_str_radix(self.as_str(), 16) {
                Ok(value) => Ok(value),
                Err(err) => Err(ParseColorError::ParseIntError(err)),
            },
            7 if self.as_str().as_bytes()[0] == b'#' => {
                match u32::from_str_radix(self.as_str().split_at(1).1, 16) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(ParseColorError::ParseIntError(err)),
                }
            }
            _ => Err(ParseColorError::InvalidString),
        }
    }

    /// Try to convert the color string to a
    /// [`DynColors`](owo_colors::DynColors).
    ///
    /// # Errors
    /// Returns an error if the string is not in a valid hexadecimal format.
    pub const fn try_as_dyncolor(&self) -> Result<owo_colors::DynColors, ParseColorError> {
        match self.try_as_u32() {
            Ok(value) => Ok(owo_colors::DynColors::Rgb(
                ((value >> 16) & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                (value & 0xFF) as u8,
            )),
            Err(err) => Err(err),
        }
    }
}

impl<'a> From<&'a str> for CustomColor<'a> {
    #[inline]
    fn from(color: &'a str) -> Self { Self::new(color) }
}
impl<'a> From<&'a Cow<'_, str>> for CustomColor<'a> {
    #[inline]
    fn from(color: &'a Cow<'_, str>) -> Self { Self::new(color) }
}

impl TryFrom<CustomColor<'_>> for u32 {
    type Error = ParseColorError;

    #[inline]
    fn try_from(value: CustomColor) -> Result<Self, Self::Error> { value.try_as_u32() }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when parsing a custom color string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseColorError {
    /// The string is not in a hexadecimal format.
    InvalidString,
    /// The string failed to parse as a [`u32`].
    ParseIntError(core::num::ParseIntError),
}
