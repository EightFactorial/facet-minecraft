//! TODO

pub mod content;
pub use content::TextContent;

use crate::style::TextStyle;

mod interaction;

/// A borrowed JSON text component.
///
/// Used as an intermediate for serializing and deserializing from JSON.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedJsonText<'a> {
    /// The content of the text component.
    #[cfg_attr(feature = "facet", facet(flatten))]
    pub content: TextContent<'a>,
    /// The style of the text component.
    #[cfg_attr(feature = "facet", facet(flatten, skip_serializing_if = TextStyle::is_none))]
    pub style: TextStyle<'a>,
}
