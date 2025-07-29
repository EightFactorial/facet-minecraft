//! TODO
#![expect(missing_docs)]

#[cfg(feature = "alloc")]
pub mod borrowed;
#[cfg(feature = "alloc")]
pub mod owned;
pub mod raw;
