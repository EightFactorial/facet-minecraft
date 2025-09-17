#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[cfg(not(feature = "foldhash"))]
type Hasher = core::hash::BuildHasherDefault<fxhash::FxHasher>;
#[cfg(feature = "foldhash")]
type Hasher = foldhash::fast::FixedState;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod map;
pub use map::{NbtItem, NbtMap};

pub mod tape;
pub use tape::NbtTape;

pub mod mutf8;
pub use mutf8::{Mutf8Str, Mutf8String};
