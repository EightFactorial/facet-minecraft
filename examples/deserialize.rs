//! A simple example of how to use the [`facet_minecraft::deserialize`] method.
//!
//! See the [Minecraft Wiki] for more information about the protocol.
//!
//! [Minecraft Wiki]: https://minecraft.wiki/w/Java_Edition_protocol/Packets
#![allow(dead_code)]
#![no_std]

use alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use facet_derive::Facet;
use facet_minecraft::deserialize_remainder;

extern crate alloc;
extern crate facet_core as facet;

#[rustfmt::skip]
fn main() {
    // u8
    let (de, rem) = deserialize_remainder::<u8>(&[0]).unwrap();
    assert_eq!(de, 0u8);
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<u8>(&[127]).unwrap();
    assert_eq!(de, 127u8);
    assert!(rem.is_empty());

    // Vec<u8>
    let (de, rem) = deserialize_remainder::<Vec<u8>>(&[3, 0, 0, 0]).unwrap();
    assert_eq!(de, &[0, 0, 0]);
    assert!(rem.is_empty());

    // u32
    let (de, rem) = deserialize_remainder::<u32>(&[0, 0, 0, 0]).unwrap();
    assert_eq!(de, 0u32);
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<u32>(&[127, 0, 0, 0]).unwrap();
    assert_eq!(de, 127u32);
    assert!(rem.is_empty());

    // Variable<u32>
    let (de, rem) = deserialize_remainder::<Variable<u32>>(&[0]).unwrap();
    assert_eq!(de.0, 0u32);
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<Variable<u32>>(&[127]).unwrap();
    assert_eq!(de.0, 127u32);
    assert!(rem.is_empty());

    // Vec<u32>
    let (de, rem) = deserialize_remainder::<Vec<u32>>(&[4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, &[0u32, 0u32, 0u32, 0u32]);
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<Vec<u32>>(&[1, 127, 0, 0, 0]).unwrap();
    assert_eq!(de, &[127u32]);
    assert!(rem.is_empty());

    // Variable<Vec<u32>>
    let (de, rem) = deserialize_remainder::<Variable<Vec<u32>>>(&[4, 0, 0, 127, 127]).unwrap();
    assert_eq!(de.0, &[0u32, 0u32, 127u32, 127u32]);
    assert!(rem.is_empty());

    // u128
    let (de, rem) = deserialize_remainder::<u128>(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, 0u128);
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<u128>(&[127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, 127u128);
    assert!(rem.is_empty());

    // Variable<u128>
    let (de, rem) = deserialize_remainder::<Variable<u128>>(&[0]).unwrap();
    assert_eq!(de.0, 0u128);
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<Variable<u128>>(&[127]).unwrap();
    assert_eq!(de.0, 127u128);
    assert!(rem.is_empty());

    // Vec<u128>
    let (de, rem) = deserialize_remainder::<Vec<u128>>(&[0]).unwrap();
    assert_eq!(de, &[]);
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<Vec<u128>>(&[1, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, &[127u128]);
    assert!(rem.is_empty());

    // Variable<Vec<u128>>
    let (de, rem) = deserialize_remainder::<Variable<Vec<u128>>>(&[6, 0, 0, 0, 127, 127, 127]).unwrap();
    assert_eq!(de.0, &[0u128, 0u128, 0u128, 127u128, 127u128, 127u128]);
    assert!(rem.is_empty());

    // ExampleEnum::A
    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[0, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, ExampleEnum::A(0));
    assert!(rem.is_empty());

    // ExampleEnum::B
    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[1, 129, 1, 0, 0, 0, 0]).unwrap();
    assert_eq!(de, ExampleEnum::B(129, 0));
    assert!(rem.is_empty());

    // ExampleEnum::C
    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[2, 2]).unwrap();
    assert_eq!(de, ExampleEnum::C(Variable(2)));
    assert!(rem.is_empty());

    // ExampleEnum::D
    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[3, 2, 1, b'a', 1, b'1', 1, b'b', 1, b'2']).unwrap();
    assert_eq!(
        de,
        ExampleEnum::D(BTreeMap::from_iter([
            (String::from("a"), String::from("1")),
            (String::from("b"), String::from("2")),
        ]))
    );
    assert!(rem.is_empty());

    // ExampleEnum::E && ExampleEnum::Z
    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[4, 4, 4, 255, 1]).unwrap();
    assert_eq!(de, ExampleEnum::E(Box::new(ExampleEnum::E(Box::new(ExampleEnum::E(Box::new(ExampleEnum::Z)))))));
    assert!(rem.is_empty());

    // ExampleEnum::E && ExampleEnum::F && ExampleEnum::Z
    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[5, 4, 5, 255, 1]).unwrap();
    assert_eq!(de, ExampleEnum::F(Arc::new(ExampleEnum::E(Box::new(ExampleEnum::F(Arc::new(ExampleEnum::Z)))))));
    assert!(rem.is_empty());

    // ExampleEnum::G
    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[6, 1, 255, 255, 127]).unwrap();
    assert_eq!(de, ExampleEnum::G(Some(-1)));
    assert!(rem.is_empty());

    let (de, rem) = deserialize_remainder::<ExampleEnum>(&[6, 0]).unwrap();
    assert_eq!(de, ExampleEnum::G(None));
    assert!(rem.is_empty());
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Facet)]
struct Variable<T>(#[facet(var)] T);

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Facet)]
enum ExampleEnum {
    A(u32),
    B(#[facet(var)] u32, u32),
    C(Variable<u64>),
    D(BTreeMap<String, String>),
    E(Box<ExampleEnum>),
    F(Arc<ExampleEnum>),
    G(#[facet(var)] Option<i16>),
    Z = 255,
}
