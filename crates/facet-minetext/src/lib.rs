#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod color;
pub mod style;

pub mod snbt;
pub mod text;

pub mod prelude {
    //! Re-exports of common types and traits.
    pub use crate::{
        color::{
            TextColor,
            custom::CustomColor,
            preset::{MineColorize, MineColors},
        },
        style::TextStyle,
        text::{BorrowedText, TextContent},
    };
}
