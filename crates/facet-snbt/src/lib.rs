#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;

pub mod format;
pub mod snbt;

pub mod deserialize;
pub mod serialize;

mod error;
pub use error::{DeserializeError, SerializeError};

pub mod prelude {
    //! Re-exports of common types and traits.
    pub use crate::{
        format::{LegacySnbt, ModernSnbt, SnbtFormat},
        snbt::Snbt,
    };
}
