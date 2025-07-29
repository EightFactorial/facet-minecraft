#![allow(unused_variables)]

use super::*;

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
fn hello_world() {
    static RAW: RawNbt<'static> =
        RawNbt::new_named(include_bytes!("../../../tests/hello_world.nbt").as_slice()).unwrap();
    borrow_and_own!("hello, world", RAW);
}

// #[test]
// fn hypixel() {
//     static RAW: RawNbt<'static> =
//         RawNbt::new_named(include_bytes!("../../../tests/hypixel.nbt").
// as_slice()).unwrap();     borrow_and_own!("hypixel", RAW);
// }

// #[test]
// fn inttest1023() {
//     static RAW: RawNbt<'static> =
//         RawNbt::new_named(include_bytes!("../../../tests/inttest1023.nbt").
// as_slice()).unwrap();     borrow_and_own!("inttest1023", RAW);
// }
