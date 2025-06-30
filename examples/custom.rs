//! A simple example of how to provide serialization overrides.
//!
//! TODO: Add a derive macro for this.
//!
//! See the [Minecraft Wiki] for more information about the protocol.
//!
//! [Minecraft Wiki]: https://minecraft.wiki/w/Java_Edition_protocol/Packets
#![no_std]

use alloc::{boxed::Box, string::String, vec::Vec};

use facet_derive::Facet;
use facet_minecraft::{
    DeserializeError, Deserializer, McDeserializer, SerializationTask, custom::FacetOverride,
    deserialize, serialize,
};
use facet_reflect::{Partial, Peek};

extern crate alloc;
extern crate facet_core as facet;

#[rustfmt::skip]
fn main() {
    let mut buffer = Vec::new();

    // A traditional `u64` serialization.
    serialize(&1024u64, &mut buffer).unwrap();
    assert_eq!(buffer, &[0, 4, 0, 0, 0, 0, 0, 0]);
    assert_eq!(deserialize::<u64>(&buffer).unwrap(), 1024u64);
    buffer.clear();

    // Using `LikeU32` to serialize a `u64` as a `u32`.
    serialize(&LikeU32(1024u64), &mut buffer).unwrap();
    assert_eq!(buffer, &[0, 4, 0, 0]);
    assert_eq!(deserialize::<LikeU32>(&buffer).unwrap(), LikeU32(1024u64));
    buffer.clear();

    // Using `Reversed` to serialize a string in reverse.
    serialize(&Reversed(String::from("Hello, World!")), &mut buffer).unwrap();
    assert_eq!(buffer, &[13u8, b'!', b'd', b'l', b'r', b'o', b'W', b' ', b',', b'o', b'l', b'l', b'e', b'H']);
    assert_eq!(deserialize::<Reversed>(&buffer).unwrap(), Reversed(String::from("Hello, World!")));
    buffer.clear();
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Facet)]
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

    fn deserialize<'input, 'partial, 'facet, 'shape>(
        partial: &'partial mut Partial<'facet, 'shape>,
        input: &'input [u8],
    ) -> Result<
        (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
        DeserializeError<'input, 'shape>,
    > {
        match McDeserializer.deserialize_u32(input) {
            Ok((val, remainder)) => match partial.set(LikeU32(val as u64)) {
                Ok(partial) => Ok((partial, remainder)),
                Err(err) => panic!("Failed to set LikeU32: {err:?}"),
            },
            Err(err) => panic!("Failed to deserialize LikeU32: {err:?}"),
        }
    }
}

facet_minecraft::custom::submit! {
    FacetOverride::new::<LikeU32>().with_ser(LikeU32::serialize).with_de(LikeU32::deserialize)
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Facet)]
#[facet(custom)]
struct Reversed(String);

impl Reversed {
    /// A custom serialization method that reverses the string.
    fn serialize<'mem, 'facet, 'shape>(
        peek: Peek<'_, 'facet, 'shape>,
        stack: &mut Vec<SerializationTask<'mem, 'facet, 'shape>>,
    ) {
        let val = &peek.get::<Self>().unwrap().0;
        let rev: String = val.chars().rev().collect();
        stack.push(SerializationTask::ValueOwned(Box::new(rev)));
    }

    fn deserialize<'input, 'partial, 'facet, 'shape>(
        partial: &'partial mut Partial<'facet, 'shape>,
        input: &'input [u8],
    ) -> Result<
        (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
        DeserializeError<'input, 'shape>,
    > {
        match McDeserializer.deserialize_str(input) {
            Ok((val, remainder)) => match partial.set(Reversed(val.chars().rev().collect())) {
                Ok(partial) => Ok((partial, remainder)),
                Err(err) => panic!("Failed to set Reversed: {err:?}"),
            },
            Err(err) => panic!("Failed to deserialize Reversed: {err:?}"),
        }
    }
}

facet_minecraft::custom::submit! {
    FacetOverride::new::<Reversed>().with_ser(Reversed::serialize).with_de(Reversed::deserialize)
}
