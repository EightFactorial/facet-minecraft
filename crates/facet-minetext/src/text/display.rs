use core::fmt::{Display, Formatter, Result};

use super::{BorrowedText, TextContent};
use crate::style::TextStyle;

pub trait TextDisplayData: Sized {}

// -------------------------------------------------------------------------------------------------

/// The [`TextDisplayData`] used by the [`Display`] trait.
struct Empty;
impl TextDisplayData for Empty {}

impl Display for BorrowedText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { Self::display(self, &mut Empty, f) }
}
impl Display for TextContent<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { self.display(&TextStyle::NONE, &mut Empty, f) }
}

// -------------------------------------------------------------------------------------------------

impl BorrowedText<'_> {
    /// Display the [`BorrowedText] with its content and style.
    ///
    /// # Errors
    /// If the content cannot be displayed, an error is returned.
    pub fn display<D: TextDisplayData>(&self, data: &mut D, fmt: &mut Formatter<'_>) -> Result {
        self.content.display(&self.style, data, fmt)?;
        for child in &self.children {
            child.display(data, fmt)?;
        }
        Ok(())
    }
}

impl TextContent<'_> {
    /// Display the [`TextContent`] with the given [`TextStyle`].
    ///
    /// # Errors
    /// If the content cannot be displayed, an error is returned.
    pub fn display<D: TextDisplayData>(
        &self,
        style: &TextStyle<'_>,
        _data: &mut D,
        fmt: &mut Formatter<'_>,
    ) -> Result {
        let style = ::owo_colors::Style::from(style);

        match self {
            TextContent::Text(c) => Display::fmt(&style.style(&c.text), fmt),
            TextContent::Translation(_c) => todo!(),
            TextContent::Score(_c) => todo!(),
            TextContent::Selector(_c) => todo!(),
            TextContent::Keybind(_c) => todo!(),
            TextContent::Nbt(_c) => todo!(),
        }
    }
}
