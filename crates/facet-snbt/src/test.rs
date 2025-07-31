#![allow(unused_variables)]

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, string::String};

use facet_nbt::prelude::*;

macro_rules! borrow_and_serialize {
    ($name:expr, $raw:expr) => {
        let borrowed = $raw.to_borrowed();
        let snbt =
            crate::serialize::serialize::<crate::format::Legacy>(&borrowed, Cow::Owned(String::new())).unwrap();

        #[cfg(feature = "std")]
        std::println!(
            "\"{name}\" Raw: {raw:?}\n\"{name}\" Borrowed: {borrowed:?}\n\"{name}\" SNBT: \"{snbt}\"",
            name = $name,
            raw = $raw,
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
