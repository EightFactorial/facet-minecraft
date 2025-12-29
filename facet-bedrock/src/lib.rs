#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod attribute;

pub mod deserialize;
#[cfg(feature = "futures-lite")]
pub use deserialize::from_async_reader;
#[cfg(feature = "streaming")]
pub use deserialize::from_reader;
#[cfg(feature = "tokio")]
pub use deserialize::from_tokio_reader;
pub use deserialize::{from_slice, from_slice_borrowed};

pub mod serialize;
#[cfg(feature = "futures-lite")]
pub use serialize::to_async_writer;
#[cfg(feature = "tokio")]
pub use serialize::to_tokio_writer;
#[cfg(feature = "streaming")]
pub use serialize::to_writer;
pub use serialize::{to_buffer, to_vec};
