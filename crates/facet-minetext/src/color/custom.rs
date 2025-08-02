//! Custom color representation.

/// A string slice that represents a color.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
pub struct CustomColor<'a>(&'a str);

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
    pub const fn new(color: &'a str) -> Self { Self(color) }

    /// Get the color's inner string slice.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'a str { self.0 }

    /// Create a new [`CustomColor`] from a `u32` value.
    #[must_use]
    pub fn new_u32(color: u32, buffer: &'a mut itoa::Buffer) -> Self { Self(buffer.format(color)) }

    /// Try to convert the color string to a [`u32`] value.
    pub const fn try_as_u32(&self) -> Result<u32, ParseColorError> {
        match self.0.len() {
            6 => match u32::from_str_radix(self.0, 16) {
                Ok(value) => Ok(value),
                Err(err) => Err(ParseColorError::ParseIntError(err)),
            },
            7 if self.0.as_bytes()[0] == b'#' => {
                match u32::from_str_radix(self.0.split_at(1).1, 16) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(ParseColorError::ParseIntError(err)),
                }
            }
            _ => Err(ParseColorError::InvalidString),
        }
    }
}

impl<'a> From<&'a str> for CustomColor<'a> {
    #[inline]
    fn from(color: &'a str) -> Self { Self::new(color) }
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
