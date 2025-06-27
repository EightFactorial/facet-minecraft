//! A simple example of how to use the [`facet_minecraft::serialize`] method.
//!
//! See the [Minecraft Wiki] for more information about the protocol.
//!
//! [Minecraft Wiki]: https://minecraft.wiki/w/Java_Edition_protocol/Packets
#![no_std]

use alloc::{boxed::Box, vec, vec::Vec};

use facet_derive::Facet;
use facet_minecraft::serialize;

extern crate alloc;
extern crate facet_core as facet;

fn main() {
    let mut buffer = Vec::new();

    // A regular `u32` will be serialized as a 4-byte integer.
    serialize(&0u32, &mut buffer).unwrap();
    assert_eq!(buffer, &[0, 0, 0, 0]);
    buffer.clear();

    // A variable-length `u32` will have a variable number of bytes.
    serialize(&Variable(0u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[0]);
    buffer.clear();

    serialize(&127u32, &mut buffer).unwrap();
    assert_eq!(buffer, &[127, 0, 0, 0]);
    buffer.clear();

    serialize(&Variable(127u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[127]);
    buffer.clear();

    serialize(&Variable(128u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[128, 1]);
    buffer.clear();

    serialize(&Variable(255u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[255, 1]);
    buffer.clear();

    serialize(&Variable(256u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[128, 2]);
    buffer.clear();

    serialize(&Variable(2_097_151u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[255, 255, 127]);
    buffer.clear();

    serialize(&2_147_483_647u32, &mut buffer).unwrap();
    assert_eq!(buffer, &[255, 255, 255, 127]);
    buffer.clear();

    serialize(&Variable(2_147_483_647u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[255, 255, 255, 255, 7]);
    buffer.clear();

    serialize(&4_294_967_295u32, &mut buffer).unwrap();
    assert_eq!(buffer, &[255, 255, 255, 255]);
    buffer.clear();

    serialize(&Variable(4_294_967_295u32), &mut buffer).unwrap();
    assert_eq!(buffer, &[255, 255, 255, 255, 15]);
    buffer.clear();

    // For an `Option`, the first byte indicates whether the value is present.
    serialize(&None::<&str>, &mut buffer).unwrap();
    assert_eq!(buffer, &[0]);
    buffer.clear();

    // Here a `1` indicates the `str` is present, followed by its length and bytes.
    serialize(&Some("Hello, World!"), &mut buffer).unwrap();
    assert_eq!(buffer, &[1, 13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]);
    buffer.clear();

    // An enum is serialized with the disciminant first, followed by its data.
    serialize(&ExampleEnum::First(42), &mut buffer).unwrap();
    assert_eq!(buffer, &[0, 42, 0, 0, 0]);
    buffer.clear();

    // The second variant (1), followed by its two `u8` values.
    serialize(&ExampleEnum::Second(1, 2), &mut buffer).unwrap();
    assert_eq!(buffer, &[1, 1, 2]);
    buffer.clear();

    // The third variant (2), which contains a variable-length `u32`.
    serialize(&ExampleEnum::Third(Variable(100)), &mut buffer).unwrap();
    assert_eq!(buffer, &[2, 100]);
    buffer.clear();

    // The fourth variant (66), contains a nested enum using a `Box`.
    serialize(&ExampleEnum::Fourth(Box::new(ExampleEnum::First(99))), &mut buffer).unwrap();
    assert_eq!(buffer, &[66, 0, 99, 0, 0, 0]);
    buffer.clear();

    // The fifth variant (255), which is a unit variant.
    serialize(&ExampleEnum::Fifth, &mut buffer).unwrap();
    assert_eq!(buffer, &[255, 1]);
    buffer.clear();

    #[cfg(feature = "json")]
    {
        // The sixth variant (100), which contains a JSON-encoded enum.
        serialize(&ExampleEnum::Sixth(Box::new(ExampleEnum::First(42))), &mut buffer).unwrap();
        assert_eq!(buffer, &[100, 12, 123, 34, 70, 105, 114, 115, 116, 34, 58, 52, 50, 125]); // 100, 12, '{"First":42}'
        buffer.clear();
    }

    // A `Vec` begins with its size, followed by its elements.
    serialize(&vec!["Hello, ", "World!"], &mut buffer).unwrap();
    assert_eq!(buffer, &[2, 7, 72, 101, 108, 108, 111, 44, 32, 6, 87, 111, 114, 108, 100, 33]);
    buffer.clear();

    // Here is a `4` indicating the length, followed by the bytes of each `u32`.
    serialize(&vec![123, 234, 567, 890], &mut buffer).unwrap();
    assert_eq!(buffer, &[4, 123, 0, 0, 0, 234, 0, 0, 0, 55, 2, 0, 0, 122, 3, 0, 0]);
    buffer.clear();

    // Here is the length, followed by the bytes of each variable-length `u32`.
    serialize(&Variable(vec![123, 234, 567, 890]), &mut buffer).unwrap();
    assert_eq!(buffer, &[4, 123, 234, 1, 183, 4, 250, 6]);
    buffer.clear();
}

// -------------------------------------------------------------------------------------------------

#[derive(Facet)]
struct Variable<T>(#[facet(var)] T);

#[repr(u8)]
#[derive(Facet)]
#[allow(dead_code)]
enum ExampleEnum {
    First(u32),
    Second(u8, u8),
    Third(Variable<u32>),
    Fourth(Box<ExampleEnum>) = 66,
    Fifth = 255,
    #[cfg(feature = "json")]
    Sixth(#[facet(json)] Box<ExampleEnum>) = 100,
}
