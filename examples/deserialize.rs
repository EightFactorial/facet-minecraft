//! A simple example of how to use the [`facet_minecraft::deserialize`] method.
//!
//! See the [Minecraft Wiki] for more information about the protocol.
//!
//! [Minecraft Wiki]: https://minecraft.wiki/w/Java_Edition_protocol/Packets
#![allow(dead_code, unused_imports)]
#![no_std]

use alloc::{boxed::Box, vec, vec::Vec};

use facet_derive::Facet;
use facet_minecraft::serialize;

extern crate alloc;
extern crate facet_core as facet;

fn main() {}
