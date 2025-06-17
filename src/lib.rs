#![doc = include_str!("../README.md.doc")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(core_io_borrowed_buf))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

extern crate facet_core as facet;

pub mod adapter;
pub mod assert;

#[cfg(feature = "deserialize")]
mod deserialize;
#[cfg(feature = "deserialize")]
pub use deserialize::{DeserializeError, deserialize};

#[cfg(feature = "serialize")]
mod serialize;
#[cfg(feature = "serialize")]
pub use serialize::serialize;

/// The Minecraft protocol
pub struct Minecraft;
