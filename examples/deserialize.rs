//! A simple example of how to use the [`facet_minecraft::deserialize`] method.
//!
//! See the [Minecraft Wiki] for more information about the protocol.
//!
//! [Minecraft Wiki]: https://minecraft.wiki/w/Java_Edition_protocol/Packets
#![allow(dead_code, unused_imports)]
#![no_std]

use facet_derive::Facet;
use facet_minecraft::deserialize;

extern crate alloc;
extern crate facet_core as facet;

#[rustfmt::skip]
fn main() {
    // u8
    let (de, rem) = deserialize::<u8>(&[0]).unwrap();
    assert_eq!(de, 0u8);
    assert!(rem.is_empty());

    let (de, rem) = deserialize::<u8>(&[127]).unwrap();
    assert_eq!(de, 127u8);
    assert!(rem.is_empty());

    // u32
    let (de, rem) = deserialize::<u32>(&[0, 0, 0, 0]).unwrap();
    assert_eq!(de, 0u32);
    assert!(rem.is_empty());

    let (de, rem) = deserialize::<u32>(&[127, 0, 0, 0]).unwrap();
    assert_eq!(de, 127u32);
    assert!(rem.is_empty());

    // u128
    let (de, rem) = deserialize::<u128>(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, 0u128);
    assert!(rem.is_empty());

    let (de, rem) = deserialize::<u128>(&[127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, 127u128);
    assert!(rem.is_empty());

    // let (de, rem) = deserialize::<Variable<u32>>(&[0]).unwrap();
    // assert_eq!(de.0, 0u32);
    // assert!(rem.is_empty());

    // let (de, rem) = deserialize::<Variable<u32>>(&[127]).unwrap();
    // assert_eq!(de.0, 127u32);
    // assert!(rem.is_empty());
}

// -------------------------------------------------------------------------------------------------

// #[derive(Facet)]
// struct Variable<T>(#[facet(var)] T);
