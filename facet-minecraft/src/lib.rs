#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod attribute;
pub mod hint;
pub use replace_with;

pub mod deserialize;
pub use deserialize::{Deserialize, fns::*};

pub mod serialize;
pub use serialize::{Serialize, fns::*};
