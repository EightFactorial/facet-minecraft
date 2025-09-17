//! An extremely fast NBT parser that holds element indexes in a flat vector.
//!
//! Only recommended for use cases where parsing performance is critical, but
//! usability and flexibility are not. Has longer access times due to the flat
//! structure requiring iterating over all elements, including nested ones, to
//! find the desired value.
#![expect(clippy::module_inception, reason = "It's a reasonable module name")]

mod error;
pub use error::{NbtTapeError, NbtTapeValidationError};

mod item;
pub use item::{NbtTapeItem, NbtTapeTag};

mod tape;
pub use tape::{NbtTape, NbtTapeSlice};

mod facet;
