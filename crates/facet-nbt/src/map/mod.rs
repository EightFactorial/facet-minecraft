//! TODO
#![expect(clippy::module_inception, reason = "It's a reasonable module name")]

mod cow;
pub use cow::ByteCow;

mod error;
pub use error::NbtError;

mod item;
pub use item::{NbtItem, NbtListItem};

mod map;
pub use map::NbtMap;

mod facet;
mod tape;
