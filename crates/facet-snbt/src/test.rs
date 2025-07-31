#![allow(unused_variables)]

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, string::String};

use facet_nbt::prelude::*;

use crate::format::Legacy;

#[test]
#[cfg(feature = "alloc")]
fn hello_world() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../../../tests/nbt/hello_world.nbt").as_slice());

    let borrowed = RAW.to_borrowed();
    let snbt = crate::serialize::serialize::<Legacy>(&borrowed, Cow::Owned(String::new())).unwrap();

    #[cfg(feature = "std")]
    std::println!("\"hello_world\" RAW: {borrowed:?}\n\"hello_world\" SNBT: \"{snbt}\"");
}
