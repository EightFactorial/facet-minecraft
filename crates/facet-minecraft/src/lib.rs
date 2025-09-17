#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "custom")]
pub mod custom;
#[cfg(feature = "rich-diagnostics")]
pub mod report;

pub mod deserialize;
pub use deserialize::{deserialize, deserialize_value};

pub mod serialize;
pub use serialize::{serialize, serialize_value};

// -------------------------------------------------------------------------------------------------

/// The default protocol serializer/deserializer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Standard;

impl deserialize::Deserializer for Standard {}
impl serialize::Serializer for Standard {}
