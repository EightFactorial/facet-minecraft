//! TODO

use facet_minetext::color::{Blue, DarkPurple, Gold, Green, MineColorize, MineColors, Red, Yellow};

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
    println!("{} {}", MSG.fg_using("aqua"), MSG.bg_using("dark_gray"));
    println!("{} {}", MSG.fg_using(MineColors::White), MSG.bg_using(MineColors::Black));
}
