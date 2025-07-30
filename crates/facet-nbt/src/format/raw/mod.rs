//! Data is stored as raw bytes and decoded only when needed.
//!
//! The benefit of this approach is it requires no dependencies or allocations
//! and can be done in a `const` context, however, it requires iterating over
//! and decoding all preceding data every time a field is accessed.
//!
//! This means it is great for validation of data at compile time, but can be
//! hard to use and isn't ideal for runtime performance.

mod compound;
pub use compound::{RawCompound, RawNbt};

mod error;
pub use error::{RawError, RawErrorKind};

mod tag;
pub use tag::{RawListTag, RawTag, RawTagType};
