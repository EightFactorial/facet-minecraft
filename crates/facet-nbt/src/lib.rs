#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;

pub mod borrowed;
pub mod owned;

pub mod deserialize;
pub mod serialize;

pub mod mutf8;

pub mod prelude {
    //! Re-exports of common types and traits.
    pub use crate::{
        mutf8::{Mutf8Str, Mutf8String},
        owned::{NbtCompound, NbtTag},
    };
}
