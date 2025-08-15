#![doc = include_str!("../README.md")]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "facet")]
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "facet")]
pub mod deserialize;
#[cfg(feature = "facet")]
pub mod serialize;

pub mod borrowed;
pub mod format;
pub mod mutf8;

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod test;

pub mod prelude {
    //! Re-exports of common types and traits.
    pub use crate::{
        format::raw::{RawError, RawNbt},
        mutf8::Mutf8Str,
    };
    #[cfg(feature = "alloc")]
    pub use crate::{
        format::{
            borrowed::{BorrowedCompound, BorrowedNbt},
            owned::{Nbt, NbtCompound, NbtListTag, NbtTag},
        },
        mutf8::Mutf8String,
    };
}
