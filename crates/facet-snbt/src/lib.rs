#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;

pub mod format;
pub mod snbt;

#[cfg(feature = "alloc")]
pub mod deserialize;

#[cfg(feature = "alloc")]
pub mod serialize;
pub use serialize::{serialize, serialize_borrowed};

#[cfg(feature = "alloc")]
mod error;
#[cfg(feature = "alloc")]
pub use error::{DeserializeError, SerializeError};

#[cfg(test)]
mod test;

pub mod prelude {
    //! Re-exports of common types and traits.
    #[cfg(feature = "alloc")]
    pub use crate::{
        error::{DeserializeError, SerializeError},
        format::ModernSnbt,
    };
    pub use crate::{
        format::{LegacySnbt, SnbtFormat},
        snbt::Snbt,
    };
}
