//! A simple example of deserializing a `&[str]` and `&[u8]` from a byte slice.
//!
//! Here, the lifetimes of the deserialized values are tied to the input data,
//! meaning they cannot outlive the input slice. In this example the input is
//! a constant, so the deserialized values have a static lifetime.
//!
//! ### WARNING
//!
//! [Facet] is a powerful tool, but it's still very new and uses
//! lots of unsafe code under the hood.
//!
//! Double check your lifetimes and don't deserialize static values
//! from a non-static input!
//!
//! [Facet]: https://github.com/facet-rs/facet
#![no_std]

use facet_macros::Facet;
use facet_minecraft::deserialize_remainder;

extern crate alloc;
extern crate facet_core as facet;

#[rustfmt::skip]
fn main() {
    const PREFIXED_DATA: &[u8] =
        &[13u8, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!'];

    // `&str`
    let (str, rem) = deserialize_remainder::<&str>(PREFIXED_DATA).unwrap();
    assert_eq!(str, "Hello, World!");
    assert!(rem.is_empty());
    assert!(is_static(str));

    // `StringRef`
    let (str_ref, rem) = deserialize_remainder::<StringRef>(PREFIXED_DATA).unwrap();
    assert_eq!(str_ref.0, "Hello, World!");
    assert!(rem.is_empty());
    assert!(is_static(str_ref.0));

    // `&[u8]`
    let (dat, rem) = deserialize_remainder::<&[u8]>(PREFIXED_DATA).unwrap();
    assert_eq!(
        dat,
        &[b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!']
    );
    assert!(rem.is_empty());
    assert!(is_static(dat));

    // `ByteRef`
    let (byte_ref, rem) = deserialize_remainder::<ByteRef>(PREFIXED_DATA).unwrap();
    assert_eq!(
        byte_ref.0,
        &[b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!']
    );
    assert!(rem.is_empty());
    assert!(is_static(byte_ref.0));

    // And to demonstrate `is_static` requires static values, this will fail to compile:
    // assert!(is_static(&alloc::string::String::from("Non-Static String")));

    // TODO: Fix soundness issues
    // // `StaticRef`
    // let str_ref = {
    //     #[derive(Facet)]
    //     struct StaticRef(&'static str);
    //
    //     let data = PREFIXED_DATA.to_vec();
    //     let (str_ref, rem) = deserialize_remainder::<StaticRef>(&data).unwrap();
    //     assert_eq!(str_ref.0, "Hello, World!");
    //     assert!(rem.is_empty());
    //     assert!(is_static(str_ref.0));
    //
    //     str_ref
    // };
    // assert_eq!(str_ref.0, "Hello, World!");
}

// -------------------------------------------------------------------------------------------------

fn is_static<T: ?Sized>(_: &'static T) -> bool { true }

#[derive(Facet)]
struct StringRef<'a>(&'a str);

#[derive(Facet)]
struct ByteRef<'a>(&'a [u8]);
