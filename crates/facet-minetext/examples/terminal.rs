//! An example showing how to use [`MineColors`] and [`MineColorize`]
//! to print colored text to the terminal.

use facet_minetext::color::preset::{
    Blue, DarkPurple, Gold, Green, MineColorize, MineColors, Red, Yellow,
};

static MSG: &str = "Hello, world!";

fn main() {
    // The `fg` and `bg` methods to color the text at compile time.
    println!("{} {}", MSG.fg::<Red>(), MSG.bg::<Red>());
    println!("{} {}", MSG.fg::<Gold>(), MSG.bg::<Gold>());
    println!("{} {}", MSG.fg::<Yellow>(), MSG.bg::<Yellow>());
    println!("{} {}", MSG.fg::<Green>(), MSG.bg::<Green>());
    println!("{} {}", MSG.fg::<Blue>(), MSG.bg::<Blue>());
    println!("{} {}", MSG.fg::<DarkPurple>(), MSG.bg::<DarkPurple>());

    println!();

    // The `fg_using` and `bg_using` methods to color the text at runtime.
    println!("{} {}", MSG.fg_using("aqua"), MSG.bg_using(MineColors::Aqua));
    println!("{} {}", MSG.fg_using("dark_gray"), MSG.bg_using(MineColors::DarkGray));

    println!();

    // Invalid string colors will fallback to white.
    println!("{}", MSG.fb_using("crimson", "transparent"));
}
