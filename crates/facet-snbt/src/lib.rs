#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate facet_core as facet;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod format;
pub mod snbt;

#[cfg(feature = "alloc")]
pub mod deserialize;
// #[cfg(feature = "alloc")]
// public use deserialize::{deserialize, deserialize_borrowed};

#[cfg(feature = "alloc")]
pub mod serialize;
#[cfg(feature = "alloc")]
pub use serialize::{serialize, serialize_borrowed};

#[cfg(test)]
mod test;

pub mod prelude {
    //! Re-exports of common types and traits.
    #[cfg(feature = "alloc")]
    pub use crate::format::ModernSnbt;
    pub use crate::{
        format::{LegacySnbt, SnbtFormat},
        snbt::Snbt,
    };
}
