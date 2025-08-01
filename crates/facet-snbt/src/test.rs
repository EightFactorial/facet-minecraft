#![allow(unused_variables)]

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, string::String};

use facet_nbt::prelude::*;

use crate::{serialize, serialize_borrowed};

macro_rules! borrow_and_serialize {
    ($name:expr, $raw:expr) => {
        let borrowed = $raw.to_borrowed();
        let borrowed_snbt =
            serialize_borrowed::<crate::format::Legacy>(&borrowed, Cow::Owned(String::new()))
                .unwrap();

        #[cfg(feature = "std")]
        std::println!(
            "\"{name}\" Borrowed: {borrowed:?}\n\"{name}\" SNBT: \"{borrowed_snbt}\"",
            name = $name,
        );

        let owned = borrowed.to_owned();
        let owned_snbt =
            serialize::<crate::format::Legacy>(&owned, Cow::Owned(String::new())).unwrap();

        pretty_assertions::assert_eq!(
            borrowed_snbt.as_ref(),
            owned_snbt.as_ref(),
            "SNBT serialization mismatch for \"{name}\"",
            name = $name
        );
    };
}

#[test]
#[cfg(feature = "alloc")]
fn hello_world() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../../../tests/nbt/hello_world.nbt").as_slice());

    borrow_and_serialize!("hello_world", RAW);
}

#[test]
#[cfg(feature = "alloc")]
fn hypixel() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../../../tests/nbt/hypixel.nbt").as_slice());
    borrow_and_serialize!("hypixel", RAW);
}

#[test]
#[cfg(feature = "alloc")]
fn inttest1023() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../../../tests/nbt/inttest1023.nbt").as_slice());

    borrow_and_serialize!("inttest1023", RAW);
}
