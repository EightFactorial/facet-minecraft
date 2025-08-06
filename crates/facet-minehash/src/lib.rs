#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

mod assert;
pub use assert::AssertHashable;

mod hash;
pub use hash::hash;
