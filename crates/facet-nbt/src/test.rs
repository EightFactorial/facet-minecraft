#![allow(unused_variables, dead_code)]

use crate::format::raw::{RawError, RawErrorKind, RawNbt};

macro_rules! borrow_and_own {
    ($name:expr, $raw:expr) => {
        #[cfg(feature = "alloc")]
        {
            let borrowed = $raw.to_borrowed();
            let owned = borrowed.clone().to_owned();

            #[cfg(feature = "std")]
            std::println!("\"{name}\" Raw: {raw:?}\n\"{name}\" Borrowed: {borrowed:?}\n\"{name}\" Owned: {owned:?}", name = $name, raw = $raw);
        }
    };
}

#[test]
fn invalid_empty() {
    static RAW: Result<RawNbt<'static>, RawError<'static>> = RawNbt::try_new_named(&[]);
    assert_eq!(RAW.clone().unwrap_err().kind(), RawErrorKind::EndOfInput);
}

#[test]
fn invalid_tag() {
    static RAW: Result<RawNbt<'static>, RawError<'static>> = RawNbt::try_new_named(&[42]);
    assert_eq!(RAW.clone().unwrap_err().kind(), RawErrorKind::InvalidTagType(42));
}

#[test]
fn empty_named() {
    static RAW: RawNbt<'static> = RawNbt::new_named(&[10, 0, 2, b'H', b'i', 0]);
    borrow_and_own!("named", RAW);
}

#[test]
fn empty_unnamed() {
    static RAW: RawNbt<'static> = RawNbt::new_unnamed(&[10, 0]);
    borrow_and_own!("unnamed", RAW);
}

#[test]
fn nested_named() {
    static RAW: RawNbt<'static> = RawNbt::new_named(&[
        10, 0, 2, b'H', b'i', 10, 0, 3, b'O', b'n', b'e', 1, 0, 4, b'B', b'y', b't', b'e', 255, 0,
        0,
    ]);
    borrow_and_own!("nested_named", RAW);
}

#[test]
fn nested_unnamed() {
    static RAW: RawNbt<'static> = RawNbt::new_unnamed(&[
        10, 10, 0, 3, b'T', b'w', b'o', 1, 0, 4, b'B', b'y', b't', b'e', 255, 0, 0,
    ]);
    borrow_and_own!("nested_unnamed", RAW);
}

#[test]
fn hello_world() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../tests/hello_world.nbt").as_slice());
    borrow_and_own!("hello, world", RAW);
}

#[test]
fn hypixel() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../tests/hypixel.nbt").as_slice());
    borrow_and_own!("hypixel", RAW);
}

#[test]
fn inttest1023() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../tests/inttest1023.nbt").as_slice());
    borrow_and_own!("inttest1023", RAW);
}
