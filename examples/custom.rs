//! A simple example of how to provide serialization overrides.
//!
//! TODO: Add a derive macro for this.
//!
//! See the [Minecraft Wiki] for more information about the protocol.
//!
//! [Minecraft Wiki]: https://minecraft.wiki/w/Java_Edition_protocol/Packets
#![allow(dead_code)]
#![no_std]

use alloc::{boxed::Box, string::String, vec, vec::Vec};

use facet_derive::Facet;
use facet_minecraft::{SerializationTask, custom::FacetOverride, serialize};
use facet_reflect::Peek;

extern crate alloc;
extern crate facet_core as facet;

fn main() {
    let mut buffer = Vec::new();

    // A traditional `u64` serialization.
    serialize(&1024u64, &mut buffer).unwrap();
    assert_eq!(buffer, vec![0, 4, 0, 0, 0, 0, 0, 0]);
    buffer.clear();

    // Using `LikeU32` to serialize a `u64` as a `u32`.
    serialize(&LikeU32(1024u64), &mut buffer).unwrap();
    assert_eq!(buffer, vec![0, 4, 0, 0]);
    buffer.clear();

    // Using `Reversed` to serialize a string in reverse.
    serialize(&Reversed("Hello, World!"), &mut buffer).unwrap();
    assert_eq!(
        buffer,
        vec![13u8, b'!', b'd', b'l', b'r', b'o', b'W', b' ', b',', b'o', b'l', b'l', b'e', b'H']
    );
    buffer.clear();
}

// -------------------------------------------------------------------------------------------------

#[derive(Facet)]
#[facet(custom)]
struct LikeU32(u64);

impl LikeU32 {
    /// A custom serialization method that casts the `u64` to a `u32`.
    fn serialize<'mem, 'facet, 'shape>(
        peek: Peek<'_, 'facet, 'shape>,
        stack: &mut Vec<SerializationTask<'mem, 'facet, 'shape>>,
    ) {
        let val = peek.get::<Self>().unwrap().0 as u32;
        stack.push(SerializationTask::ValueOwned(Box::new(val)));
    }
}

facet_minecraft::custom::submit! {
    FacetOverride::new::<LikeU32>().with_ser(LikeU32::serialize)
}

// -------------------------------------------------------------------------------------------------

#[derive(Facet)]
#[facet(custom)]
struct Reversed(&'static str);

impl Reversed {
    /// A custom serialization method that reverses the string.
    fn serialize<'mem, 'facet, 'shape>(
        peek: Peek<'_, 'facet, 'shape>,
        stack: &mut Vec<SerializationTask<'mem, 'facet, 'shape>>,
    ) {
        let val = peek.get::<Self>().unwrap().0;
        let rev: String = val.chars().rev().collect();
        stack.push(SerializationTask::ValueOwned(Box::new(rev)));
    }
}

facet_minecraft::custom::submit! {
    FacetOverride::new::<Reversed>().with_ser(Reversed::serialize)
}
