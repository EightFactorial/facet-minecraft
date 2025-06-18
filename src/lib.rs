#![doc = include_str!("../README.doc.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(core_io_borrowed_buf))]
#![no_std]

extern crate alloc;
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;

pub mod adapter;
pub mod assert;

#[cfg(feature = "custom")]
pub mod custom;

#[cfg(feature = "deserialize")]
mod deserialize;
#[cfg(feature = "deserialize")]
pub use deserialize::{DeserializeError, deserialize};

#[cfg(feature = "serialize")]
mod serialize;
#[cfg(feature = "serialize")]
pub use serialize::{
    McSerializer, SerializationTask, Serializer, SerializerExt, serialize, serialize_iterative,
};

/// The Minecraft protocol
pub struct Minecraft;
