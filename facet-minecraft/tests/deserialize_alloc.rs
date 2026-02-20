//! TODO
#![no_std]

extern crate alloc;

use alloc::{string::String, vec, vec::Vec};

use facet::Facet;
use facet_minecraft::{Deserialize, deserialize::error::DeserializeError};

#[repr(transparent)]
struct TestCursor(&'static [u8]);

impl TestCursor {
    fn read<T: Deserialize<'static> + Facet<'static>>(
        &mut self,
    ) -> Result<T, DeserializeError<'static>> {
        let (value, remaining) = T::from_slice_remainder(self.0)?;
        self.0 = remaining;
        Ok(value)
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn string() {
    let mut cursor = TestCursor(&[
        0, 1, b'A', 3, b'F', b'o', b'o', 13, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o',
        b'r', b'l', b'd', b'!',
    ]);

    assert_eq!(cursor.read::<String>().unwrap(), "");
    assert_eq!(cursor.read::<String>().unwrap(), "A");
    assert_eq!(cursor.read::<String>().unwrap(), "Foo");
    assert_eq!(cursor.read::<String>().unwrap(), "Hello, World!");
}

#[test]
fn vec_u8() {
    let mut cursor = TestCursor(&[0, 3, 1, 2, 3, 5, 10, 20, 30, 40, 50, 1, 255]);

    assert_eq!(cursor.read::<Vec<u8>>().unwrap(), vec![]);
    assert_eq!(cursor.read::<Vec<u8>>().unwrap(), vec![1, 2, 3]);
    assert_eq!(cursor.read::<Vec<u8>>().unwrap(), vec![10, 20, 30, 40, 50]);
    assert_eq!(cursor.read::<Vec<u8>>().unwrap(), vec![255]);
}
