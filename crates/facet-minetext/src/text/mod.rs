//! TODO

use alloc::vec::Vec;

pub mod content;
pub use content::TextContent;

mod interaction;

use crate::style::TextStyle;

/// A borrowed text component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct BorrowedText<'a> {
    /// The content of the text component.
    #[facet(flatten)]
    pub content: TextContent<'a>,
    /// The style of the text component.
    #[facet(flatten, skip_serializing_if = TextStyle::is_none)]
    pub style: TextStyle<'a>,

    /// Children that inherit the style of this text component.
    #[facet(skip_serializing_if = Vec::is_empty)]
    pub children: Vec<BorrowedText<'a>>,
}

// -------------------------------------------------------------------------------------------------

impl<'a> BorrowedText<'a> {
    /// Create a new [`BorrowedText`] from the given content.
    #[inline]
    #[must_use]
    pub const fn new(content: TextContent<'a>) -> Self {
        Self { content, style: TextStyle::NONE, children: Vec::new() }
    }

    /// Create a new [`BorrowedText`] with the given content and style.
    #[inline]
    #[must_use]
    pub const fn new_with(content: TextContent<'a>, style: TextStyle<'a>) -> Self {
        Self { content, style, children: Vec::new() }
    }

    /// Reborrow a reference to an owned [`BorrowedText`].
    #[must_use]
    pub fn reborrow(&self) -> BorrowedText<'_> {
        BorrowedText {
            content: self.content.reborrow(),
            style: self.style.reborrow(),
            children: self.children.iter().map(|child| child.reborrow()).collect(),
        }
    }

    /// Set the [`TextContent`] of the [`BorrowedText`].
    #[inline]
    #[must_use]
    pub fn with_content(mut self, content: TextContent<'a>) -> Self {
        self.content = content;
        self
    }

    /// Set the [`TextStyle`] of the [`BorrowedText`].
    #[inline]
    #[must_use]
    pub fn with_style(mut self, style: TextStyle<'a>) -> Self {
        self.style = style;
        self
    }

    /// Set the children of the [`BorrowedText`].
    #[inline]
    #[must_use]
    pub fn with_children(mut self, children: Vec<BorrowedText<'a>>) -> Self {
        self.children = children;
        self
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> BorrowedText<'a> {
    /// Returns a simplified version of the [`BorrowedText`] where redundant
    /// styling is removed.
    ///
    /// This is useful when serializing text components.
    ///
    /// # Examples
    /// ```rust
    /// use facet_minetext::{color::preset::Red, prelude::*, text::content::TextComponent};
    ///
    /// /// "Hello, " with a red color and italic style.
    /// const HELLO: BorrowedText<'static> = BorrowedText::new_with(
    ///     TextContent::Text(TextComponent::new("Hello, ")),
    ///     TextStyle::NONE.with_color(TextColor::Preset(MineColors::Red)).with_italic(true),
    /// );
    /// /// "World!" with a red color and bold style.
    /// const WORLD: BorrowedText<'static> = BorrowedText::new_with(
    ///     TextContent::Text(TextComponent::new("World!")),
    ///     TextStyle::NONE.with_color(TextColor::Preset(MineColors::Red)).with_bold(true),
    /// );
    ///
    /// /// An empty container with a red color and bold style
    /// let redundant = BorrowedText::new_with(
    ///     TextContent::Text(TextComponent::new("")),
    ///     TextStyle::NONE.with_color(TextColor::Preset(MineColors::Red)).with_bold(true),
    /// )
    /// .with_children(vec![HELLO, WORLD]);
    ///
    /// // Simplifying removes redundant styles, meaning any matching red color and bold styling
    /// // is removed from all children as they inherit these properties from the parent.
    /// let simplified = redundant.simplify(None);
    ///
    /// assert_eq!(simplified.style.color, Some(TextColor::Preset(MineColors::Red)));
    /// assert_eq!(simplified.style.bold, Some(true));
    ///
    /// // The `italic` property was not inherited, so only it remains.
    /// assert_eq!(simplified.children[0].style.color, None);
    /// assert_eq!(simplified.children[0].style.bold, None);
    /// assert_eq!(simplified.children[0].style.italic, Some(true));
    ///
    /// // All properties of the second child were inherited and thus removed.
    /// assert_eq!(simplified.children[1].style.color, None);
    /// assert_eq!(simplified.children[1].style.bold, None);
    /// assert_eq!(simplified.children[1].style.italic, None);
    /// ```
    #[must_use]
    pub fn simplify(&self, root: Option<TextStyle<'a>>) -> BorrowedText<'a> {
        let mut style = self.style.clone();
        if let Some(root) = root {
            style = style.diff_owned(&root).inherit_owned(&root);
        }

        let mut children = Vec::with_capacity(self.children.len());
        for child in &self.children {
            let mut child = child.simplify(Some(style.clone()));
            child.style = child.style.diff_owned(&style);
            children.push(child);
        }

        Self { content: self.content.clone(), style, children }
    }

    /// Propagate the styling of [`BorrowedText`] to all children.
    ///
    /// This is useful when displaying text components.
    ///
    /// # Examples
    /// ```rust
    /// use facet_minetext::{
    ///     color::preset::{Blue, Red},
    ///     prelude::*,
    ///     text::content::TextComponent,
    /// };
    ///
    /// /// "Hello, " with an italic style.
    /// const HELLO: BorrowedText<'static> = BorrowedText::new_with(
    ///     TextContent::Text(TextComponent::new("Hello, ")),
    ///     TextStyle::NONE.with_italic(true),
    /// );
    ///
    /// /// "World!" with a blue color and bold style.
    /// const WORLD: BorrowedText<'static> = BorrowedText::new_with(
    ///     TextContent::Text(TextComponent::new("World!")),
    ///     TextStyle::NONE.with_color(TextColor::Preset(MineColors::Blue)).with_bold(true),
    /// );
    ///
    /// /// An empty container with a red color and obfuscated style.
    /// let container = BorrowedText::new_with(
    ///     TextContent::Text(TextComponent::new("")),
    ///     TextStyle::NONE.with_color(TextColor::Preset(MineColors::Red)).with_obfuscation(true),
    /// )
    /// .with_children(vec![HELLO, WORLD]);
    ///
    /// // Propagate the container's style to its children.
    /// let propagated = container.propagate(None);
    ///
    /// // The first child inherits the red color and obfuscation, keeping its italic style.
    /// assert_eq!(propagated.children[0].style.color, Some(TextColor::Preset(MineColors::Red)));
    /// assert_eq!(propagated.children[0].style.bold, None);
    /// assert_eq!(propagated.children[0].style.italic, Some(true));
    /// assert_eq!(propagated.children[0].style.obfuscated, Some(true));
    ///
    /// // The second child inherits only the obfucation, as it already has a blue color.
    /// assert_eq!(propagated.children[1].style.color, Some(TextColor::Preset(MineColors::Blue)));
    /// assert_eq!(propagated.children[1].style.bold, Some(true));
    /// assert_eq!(propagated.children[1].style.italic, None);
    /// assert_eq!(propagated.children[1].style.obfuscated, Some(true));
    /// ```
    #[must_use]
    pub fn propagate(&self, root: Option<TextStyle<'a>>) -> BorrowedText<'a> {
        let mut style = self.style.clone();
        if let Some(root) = root {
            style = style.inherit_owned(&root);
        }

        let mut children = Vec::with_capacity(self.children.len());
        for child in &self.children {
            children.push(child.propagate(Some(style.clone())));
        }

        Self { content: self.content.clone(), style, children }
    }
}
