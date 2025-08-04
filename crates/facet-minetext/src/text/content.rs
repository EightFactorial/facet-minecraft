//! TODO
#![expect(missing_docs)]

use alloc::{borrow::Cow, boxed::Box, vec::Vec};

use super::BorrowedText;
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

impl TextContent<'_> {
    /// Reborrow a reference to an owned [`TextContent`].
    #[must_use]
    pub fn reborrow(&self) -> TextContent<'_> {
        match self {
            TextContent::Text(text) => TextContent::Text(text.reborrow()),
            TextContent::Translation(translation) => {
                TextContent::Translation(translation.reborrow())
            }
            TextContent::Score(score) => TextContent::Score(score.reborrow()),
            TextContent::Selector(selector) => TextContent::Selector(selector.reborrow()),
            TextContent::Keybind(keybind) => TextContent::Keybind(keybind.reborrow()),
            TextContent::Nbt(nbt) => TextContent::Nbt(nbt.reborrow()),
        }
    }
}

impl<'a> From<TextComponent<'a>> for TextContent<'a> {
    fn from(value: TextComponent<'a>) -> Self { TextContent::Text(value) }
}
impl<'a> From<&'a str> for TextContent<'a> {
    fn from(value: &'a str) -> Self { TextContent::from(TextComponent::from(value)) }
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

impl<'a> TextComponent<'a> {
    /// Create a new [`TextComponent`] from a string slice.
    #[must_use]
    pub const fn new(text: &'a str) -> Self { TextComponent { text: Cow::Borrowed(text) } }

    /// Reborrow a reference to an owned [`TextComponent`].
    #[must_use]
    pub const fn reborrow(&self) -> TextComponent<'_> {
        match &self.text {
            Cow::Borrowed(s) => TextComponent { text: Cow::Borrowed(s) },
            Cow::Owned(s) => TextComponent { text: Cow::Borrowed(s.as_str()) },
        }
    }
}

impl<'a> From<Cow<'a, str>> for TextComponent<'a> {
    fn from(value: Cow<'a, str>) -> Self { TextComponent { text: value } }
}
impl<'a> From<&'a str> for TextComponent<'a> {
    fn from(value: &'a str) -> Self { TextComponent { text: Cow::Borrowed(value) } }
}

#[test]
#[cfg(feature = "json")]
fn text() {
    const TEXT: BorrowedText<'static> = BorrowedText {
        content: TextContent::Text(TextComponent { text: Cow::Borrowed("Hello, World!") }),
        style: TextStyle::NONE,
        children: Vec::new(),
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
    with: Vec<BorrowedText<'a>>,
}

impl TranslationComponent<'_> {
    /// Reborrow a reference to an owned [`TranslationComponent`].
    #[must_use]
    pub fn reborrow(&self) -> TranslationComponent<'_> {
        TranslationComponent {
            translate: match &self.translate {
                Cow::Borrowed(s) => Cow::Borrowed(s),
                Cow::Owned(s) => Cow::Borrowed(s.as_str()),
            },
            fallback: match &self.fallback {
                Some(Cow::Borrowed(s)) => Some(Cow::Borrowed(s)),
                Some(Cow::Owned(s)) => Some(Cow::Borrowed(s.as_str())),
                None => None,
            },
            with: self.with.iter().map(|child| child.reborrow()).collect(),
        }
    }
}

#[test]
#[cfg(feature = "json")]
fn translation() {
    let translation: BorrowedText<'static> = BorrowedText {
        content: TextContent::Translation(TranslationComponent {
            translate: Cow::Borrowed("minetext:example.translation"),
            fallback: None,
            with: alloc::vec![],
        }),
        style: TextStyle::NONE,
        children: Vec::new(),
    };

    let json = facet_json::to_string(&translation);

    #[cfg(feature = "std")]
    std::println!("\"translation\" RAW: {translation:?}\n\"translation\" JSON: {json}");
}

#[test]
#[cfg(feature = "json")]
fn translation_with() {
    let translation: BorrowedText<'static> = BorrowedText {
        content: TextContent::Translation(TranslationComponent {
            translate: Cow::Borrowed("minetext:example.translation2"),
            fallback: Some(Cow::Borrowed("Fallback")),
            with: alloc::vec![
                BorrowedText {
                    content: TextContent::Text(TextComponent { text: Cow::Borrowed("Hello, ") }),
                    style: TextStyle::NONE,
                    children: Vec::new(),
                },
                BorrowedText {
                    content: TextContent::Text(TextComponent { text: Cow::Borrowed("World!") }),
                    style: TextStyle::NONE,
                    children: Vec::new(),
                },
            ],
        }),
        style: TextStyle::NONE,
        children: Vec::new(),
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

impl ScoreComponent<'_> {
    /// Reborrow a reference to an owned [`ScoreComponent`].
    #[must_use]
    pub const fn reborrow(&self) -> ScoreComponent<'_> {
        ScoreComponent {
            name: match &self.name {
                Cow::Borrowed(s) => Cow::Borrowed(s),
                Cow::Owned(s) => Cow::Borrowed(s.as_str()),
            },
            objective: match &self.objective {
                Cow::Borrowed(s) => Cow::Borrowed(s),
                Cow::Owned(s) => Cow::Borrowed(s.as_str()),
            },
        }
    }
}

#[test]
#[cfg(feature = "json")]
fn score() {
    const SCORE: BorrowedText<'static> = BorrowedText {
        content: TextContent::Score(ScoreComponent {
            name: Cow::Borrowed("PlayerName"),
            objective: Cow::Borrowed("ObjectiveName"),
        }),
        style: TextStyle::NONE,
        children: Vec::new(),
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
    separator: Box<BorrowedText<'a>>,
}

impl SelectorComponent<'_> {
    /// Reborrow a reference to an owned [`SelectorComponent`].
    #[must_use]
    pub fn reborrow(&self) -> SelectorComponent<'_> {
        SelectorComponent {
            selector: match &self.selector {
                Cow::Borrowed(s) => Cow::Borrowed(s),
                Cow::Owned(s) => Cow::Borrowed(s.as_str()),
            },
            separator: Box::new(self.separator.reborrow()),
        }
    }
}

#[test]
#[cfg(feature = "json")]
fn selector() {
    let selector: BorrowedText<'static> = BorrowedText {
        content: TextContent::Selector(SelectorComponent {
            selector: Cow::Borrowed("@a"),
            separator: default_separator(),
        }),
        style: TextStyle::NONE,
        children: Vec::new(),
    };

    let json = facet_json::to_string(&selector);

    #[cfg(feature = "std")]
    std::println!("\"selector\" RAW: {selector:?}\n\"selector\" JSON: {json}");
}

#[test]
#[cfg(feature = "json")]
fn selector_separator() {
    let selector: BorrowedText<'static> = BorrowedText {
        content: TextContent::Selector(SelectorComponent {
            selector: Cow::Borrowed("@a"),
            separator: Box::new(BorrowedText {
                content: TextContent::Text(TextComponent { text: Cow::Borrowed(" | ") }),
                style: TextStyle::NONE,
                children: Vec::new(),
            }),
        }),
        style: TextStyle::NONE,
        children: Vec::new(),
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

impl KeybindComponent<'_> {
    /// Reborrow a reference to an owned [`KeybindComponent`].
    #[must_use]
    pub const fn reborrow(&self) -> KeybindComponent<'_> {
        KeybindComponent {
            keybind: match &self.keybind {
                Cow::Borrowed(s) => Cow::Borrowed(s),
                Cow::Owned(s) => Cow::Borrowed(s.as_str()),
            },
        }
    }
}

impl<'a> From<Cow<'a, str>> for KeybindComponent<'a> {
    fn from(value: Cow<'a, str>) -> Self { KeybindComponent { keybind: value } }
}
impl<'a> From<&'a str> for KeybindComponent<'a> {
    fn from(value: &'a str) -> Self { KeybindComponent { keybind: Cow::Borrowed(value) } }
}

#[test]
#[cfg(feature = "json")]
fn keybind() {
    const KEYBIND: BorrowedText<'static> = BorrowedText {
        content: TextContent::Keybind(KeybindComponent {
            keybind: Cow::Borrowed("key.minecraft.jump"),
        }),
        style: TextStyle::NONE,
        children: Vec::new(),
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
    separator: Box<BorrowedText<'a>>,

    #[facet(skip_serializing_if = Option::is_none)]
    block: Option<Cow<'a, str>>,
    #[facet(skip_serializing_if = Option::is_none)]
    entity: Option<Cow<'a, str>>,
    #[facet(skip_serializing_if = Option::is_none)]
    storage: Option<Cow<'a, str>>,
}

impl NbtComponent<'_> {
    /// Reborrow a reference to an owned [`NbtComponent`].
    #[must_use]
    pub fn reborrow(&self) -> NbtComponent<'_> {
        NbtComponent {
            source: self.source.as_ref().map(|s| Cow::Borrowed(s.as_ref())),
            path: self.path.as_ref().map(|s| Cow::Borrowed(s.as_ref())),
            interpret: self.interpret,
            separator: Box::new(self.separator.reborrow()),
            block: self.block.as_ref().map(|s| Cow::Borrowed(s.as_ref())),
            entity: self.entity.as_ref().map(|s| Cow::Borrowed(s.as_ref())),
            storage: self.storage.as_ref().map(|s| Cow::Borrowed(s.as_ref())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

static DEFAULT_SEPARATOR: BorrowedText<'static> = BorrowedText {
    content: TextContent::Text(TextComponent { text: Cow::Borrowed(", ") }),
    style: TextStyle::NONE,
    children: Vec::new(),
};

fn default_separator() -> Box<BorrowedText<'static>> { Box::new(DEFAULT_SEPARATOR.clone()) }

#[expect(clippy::borrowed_box)]
fn is_default_separator(separator: &Box<BorrowedText<'_>>) -> bool {
    separator.as_ref() == &DEFAULT_SEPARATOR
}
