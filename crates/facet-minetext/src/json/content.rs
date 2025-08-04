//! TODO
#![expect(missing_docs)]

use alloc::{borrow::Cow, boxed::Box, vec::Vec};

use super::BorrowedJsonText;
use crate::style::TextStyle;

// TODO: Add `facet(untagged)` when it is implemented.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub enum TextContent<'a> {
    Text(TextComponent<'a>),
    Translation(TranslationComponent<'a>),
    Score(ScoreComponent<'a>),
    Selector(SelectorComponent<'a>),
    Keybind(KeybindComponent<'a>),
    Nbt(NbtComponent<'a>),
}

impl<'a> From<TextComponent<'a>> for TextContent<'a> {
    fn from(value: TextComponent<'a>) -> Self { TextContent::Text(value) }
}
impl<'a> From<TranslationComponent<'a>> for TextContent<'a> {
    fn from(value: TranslationComponent<'a>) -> Self { TextContent::Translation(value) }
}
impl<'a> From<ScoreComponent<'a>> for TextContent<'a> {
    fn from(value: ScoreComponent<'a>) -> Self { TextContent::Score(value) }
}
impl<'a> From<SelectorComponent<'a>> for TextContent<'a> {
    fn from(value: SelectorComponent<'a>) -> Self { TextContent::Selector(value) }
}
impl<'a> From<KeybindComponent<'a>> for TextContent<'a> {
    fn from(value: KeybindComponent<'a>) -> Self { TextContent::Keybind(value) }
}
impl<'a> From<NbtComponent<'a>> for TextContent<'a> {
    fn from(value: NbtComponent<'a>) -> Self { TextContent::Nbt(value) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct TextComponent<'a> {
    pub text: Cow<'a, str>,
}

impl<'a> From<Cow<'a, str>> for TextComponent<'a> {
    fn from(value: Cow<'a, str>) -> Self { TextComponent { text: value } }
}
impl<'a> From<&'a str> for TextComponent<'a> {
    fn from(value: &'a str) -> Self { TextComponent { text: Cow::Borrowed(value) } }
}

#[test]
fn text() {
    const TEXT: BorrowedJsonText<'static> = BorrowedJsonText {
        content: TextContent::Text(TextComponent { text: Cow::Borrowed("Hello, World!") }),
        style: TextStyle::NONE,
    };

    let json = facet_json::to_string(&TEXT);

    #[cfg(feature = "std")]
    std::println!("\"text\" RAW: {TEXT:?}\n\"text\" JSON: {json}");
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct TranslationComponent<'a> {
    translate: Cow<'a, str>,
    #[facet(skip_serializing_if = Option::is_none)]
    fallback: Option<Cow<'a, str>>,
    #[facet(skip_serializing_if = Vec::is_empty)]
    with: Vec<BorrowedJsonText<'a>>,
}

#[test]
fn translation() {
    let translation: BorrowedJsonText<'static> = BorrowedJsonText {
        content: TextContent::Translation(TranslationComponent {
            translate: Cow::Borrowed("minetext:example.translation"),
            fallback: None,
            with: alloc::vec![],
        }),
        style: TextStyle::NONE,
    };

    let json = facet_json::to_string(&translation);

    #[cfg(feature = "std")]
    std::println!("\"translation\" RAW: {translation:?}\n\"translation\" JSON: {json}");
}

#[test]
fn translation_with() {
    let translation: BorrowedJsonText<'static> = BorrowedJsonText {
        content: TextContent::Translation(TranslationComponent {
            translate: Cow::Borrowed("minetext:example.translation2"),
            fallback: Some(Cow::Borrowed("Fallback")),
            with: alloc::vec![
                BorrowedJsonText {
                    content: TextContent::Text(TextComponent { text: Cow::Borrowed("Hello, ") }),
                    style: TextStyle::NONE,
                },
                BorrowedJsonText {
                    content: TextContent::Text(TextComponent { text: Cow::Borrowed("World!") }),
                    style: TextStyle::NONE,
                },
            ],
        }),
        style: TextStyle::NONE,
    };

    let json = facet_json::to_string(&translation);

    #[cfg(feature = "std")]
    std::println!("\"translation_with\" RAW: {translation:?}\n\"translation_with\" JSON: {json}");
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct ScoreComponent<'a> {
    pub name: Cow<'a, str>,
    pub objective: Cow<'a, str>,
}

#[test]
fn score() {
    const SCORE: BorrowedJsonText<'static> = BorrowedJsonText {
        content: TextContent::Score(ScoreComponent {
            name: Cow::Borrowed("PlayerName"),
            objective: Cow::Borrowed("ObjectiveName"),
        }),
        style: TextStyle::NONE,
    };

    let json = facet_json::to_string(&SCORE);

    #[cfg(feature = "std")]
    std::println!("\"score\" RAW: {SCORE:?}\n\"score\" JSON: {json}");
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct SelectorComponent<'a> {
    selector: Cow<'a, str>,
    #[facet(default = default_separator())]
    #[facet(skip_serializing_if = is_default_separator)]
    separator: Box<BorrowedJsonText<'a>>,
}

#[test]
fn selector() {
    let selector: BorrowedJsonText<'static> = BorrowedJsonText {
        content: TextContent::Selector(SelectorComponent {
            selector: Cow::Borrowed("@a"),
            separator: default_separator(),
        }),
        style: TextStyle::NONE,
    };

    let json = facet_json::to_string(&selector);

    #[cfg(feature = "std")]
    std::println!("\"selector\" RAW: {selector:?}\n\"selector\" JSON: {json}");
}

#[test]
fn selector_separator() {
    let selector: BorrowedJsonText<'static> = BorrowedJsonText {
        content: TextContent::Selector(SelectorComponent {
            selector: Cow::Borrowed("@a"),
            separator: Box::new(BorrowedJsonText {
                content: TextContent::Text(TextComponent { text: Cow::Borrowed(" | ") }),
                style: TextStyle::NONE,
            }),
        }),
        style: TextStyle::NONE,
    };

    let json = facet_json::to_string(&selector);

    #[cfg(feature = "std")]
    std::println!(
        "\"selector_separator\" with custom separator RAW: {selector:?}\n\"selector_separator\" JSON: {json}"
    );
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct KeybindComponent<'a> {
    pub keybind: Cow<'a, str>,
}

impl<'a> From<Cow<'a, str>> for KeybindComponent<'a> {
    fn from(value: Cow<'a, str>) -> Self { KeybindComponent { keybind: value } }
}
impl<'a> From<&'a str> for KeybindComponent<'a> {
    fn from(value: &'a str) -> Self { KeybindComponent { keybind: Cow::Borrowed(value) } }
}

#[test]
fn keybind() {
    const KEYBIND: BorrowedJsonText<'static> = BorrowedJsonText {
        content: TextContent::Keybind(KeybindComponent {
            keybind: Cow::Borrowed("key.minecraft.jump"),
        }),
        style: TextStyle::NONE,
    };

    let json = facet_json::to_string(&KEYBIND);

    #[cfg(feature = "std")]
    std::println!("\"keybind\" RAW: {KEYBIND:?}\n\"keybind\" JSON: {json}");
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, facet::Facet)]
pub struct NbtComponent<'a> {
    #[facet(skip_serializing_if = Option::is_none)]
    source: Option<Cow<'a, str>>,
    #[facet(skip_serializing_if = Option::is_none)]
    path: Option<Cow<'a, str>>,

    #[facet(skip_serializing_if = Option::is_none)]
    interpret: Option<bool>,
    #[facet(default = default_separator())]
    #[facet(skip_serializing_if = is_default_separator)]
    separator: Box<BorrowedJsonText<'a>>,

    #[facet(skip_serializing_if = Option::is_none)]
    block: Option<Cow<'a, str>>,
    #[facet(skip_serializing_if = Option::is_none)]
    entity: Option<Cow<'a, str>>,
    #[facet(skip_serializing_if = Option::is_none)]
    storage: Option<Cow<'a, str>>,
}

// -------------------------------------------------------------------------------------------------

static DEFAULT_SEPARATOR: BorrowedJsonText<'static> = BorrowedJsonText {
    content: TextContent::Text(TextComponent { text: Cow::Borrowed(", ") }),
    style: TextStyle::NONE,
};

fn default_separator() -> Box<BorrowedJsonText<'static>> { Box::new(DEFAULT_SEPARATOR.clone()) }

#[expect(clippy::borrowed_box)]
fn is_default_separator(separator: &Box<BorrowedJsonText<'_>>) -> bool {
    separator.as_ref() == &DEFAULT_SEPARATOR
}
