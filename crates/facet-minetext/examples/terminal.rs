//! An example showing how to create and style [`BorrowedText`] with children,
//! and why to `propagate` before printing to the terminal.

use facet_minetext::{prelude::*, text::content::TextComponent};

const BASE: BorrowedText<'static> =
    BorrowedText::new_with(TextContent::Text(TextComponent::new("")), TextStyle::NONE);

const TEXT_A: BorrowedText<'static> = BorrowedText::new_with(
    TextContent::Text(TextComponent::new("Text A ")),
    TextStyle::NONE.with_color(TextColor::Preset(MineColors::Red)),
);
const TEXT_A_CHILD_1: BorrowedText<'static> = BorrowedText::new_with(
    TextContent::Text(TextComponent::new("Child A1 ")),
    TextStyle::NONE.with_color(TextColor::Preset(MineColors::Blue)).with_bold(true),
);
const TEXT_A_CHILD_2: BorrowedText<'static> =
    BorrowedText::new_with(TextContent::Text(TextComponent::new("Child A2 ")), TextStyle::NONE);
const TEXT_A_CHILD_3: BorrowedText<'static> =
    BorrowedText::new_with(TextContent::Text(TextComponent::new("Child A3 ")), TextStyle::NONE);

const TEXT_B: BorrowedText<'static> = BorrowedText::new_with(
    TextContent::Text(TextComponent::new("Text B ")),
    TextStyle::NONE.with_color(TextColor::Preset(MineColors::Green)),
);
const TEXT_B_CHILD_1: BorrowedText<'static> =
    BorrowedText::new(TextContent::Text(TextComponent::new("Child B1 ")));
const TEXT_B_CHILD_2: BorrowedText<'static> = BorrowedText::new_with(
    TextContent::Text(TextComponent::new("Child B2")),
    TextStyle::NONE.with_strikethrough(true),
);

fn main() {
    let text = BASE.with_children(vec![
        TEXT_A.with_children(vec![
            TEXT_A_CHILD_1,
            TEXT_A_CHILD_2.with_children(vec![TEXT_A_CHILD_3]),
        ]),
        TEXT_B.with_children(vec![TEXT_B_CHILD_1, TEXT_B_CHILD_2]),
    ]);

    println!("Before: {text}");
    println!("After:  {}", text.propagate(None));
}
