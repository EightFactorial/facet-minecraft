#![doc = include_str!("../README.doc.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(core_io_borrowed_buf))]
#![no_std]

#[allow(dead_code)]
const ERROR_SOURCE: &str = env!("CARGO_CRATE_NAME");

extern crate alloc;
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;

mod adapter;
pub use adapter::{FacetAdapter, SliceCursor, WriteAdapter};

mod assert;
pub use assert::AssertProtocol;

#[cfg(feature = "custom")]
pub mod custom;

#[cfg(feature = "deserialize")]
mod deserialize;
#[cfg(feature = "deserialize")]
pub use deserialize::{
    DeserializeError, Deserializer, DeserializerExt, McDeserializer, deserialize,
    deserialize_iterative,
};

#[cfg(feature = "serialize")]
mod serialize;
#[cfg(feature = "serialize")]
pub use serialize::{
    McSerializer, OwnedPeek, SerializationTask, SerializeError, Serializer, SerializerExt,
    serialize, serialize_iterative,
};
