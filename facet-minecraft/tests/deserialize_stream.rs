//! TODO

use std::io::Cursor;

use facet_format::DeserializeError as FDError;
use facet_minecraft::{Deserializable, deserialize::DeserializeError};

#[repr(transparent)]
struct TestCursor(Cursor<&'static [u8]>);

impl TestCursor {
    fn read<T: Deserializable<'static>>(&mut self) -> Result<T, FDError<DeserializeError>> {
        T::from_reader(&mut self.0)
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[expect(clippy::bool_assert_comparison, reason = "Easier to read")]
fn bool() {
    let mut cursor = TestCursor(Cursor::new(&[0, 1, 0, 1, 0, 1, 0, 1, 2]));

    assert_eq!(cursor.read::<bool>().unwrap(), false);
    assert_eq!(cursor.read::<bool>().unwrap(), true);
    assert_eq!(cursor.read::<bool>().unwrap(), false);
    assert_eq!(cursor.read::<bool>().unwrap(), true);
    assert_eq!(cursor.read::<bool>().unwrap(), false);
    assert_eq!(cursor.read::<bool>().unwrap(), true);
    assert_eq!(cursor.read::<bool>().unwrap(), false);
    assert_eq!(cursor.read::<bool>().unwrap(), true);
    assert!(cursor.read::<bool>().is_err());
}

#[test]
fn u8() {
    let mut cursor =
        TestCursor(Cursor::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]));

    assert_eq!(cursor.read::<u8>().unwrap(), 0u8);
    assert_eq!(cursor.read::<u8>().unwrap(), 1u8);
    assert_eq!(cursor.read::<u8>().unwrap(), 2u8);
    assert_eq!(cursor.read::<u8>().unwrap(), 3u8);
    assert_eq!(cursor.read::<u8>().unwrap(), 4u8);
    assert_eq!(cursor.read::<u8>().unwrap(), 5u8);
    assert_eq!(cursor.read::<u8>().unwrap(), 6u8);
    assert_eq!(cursor.read::<u8>().unwrap(), 7u8);
}

#[test]
fn i8() {
    let mut cursor =
        TestCursor(Cursor::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]));

    assert_eq!(cursor.read::<i8>().unwrap(), 0i8);
    assert_eq!(cursor.read::<i8>().unwrap(), 1i8);
    assert_eq!(cursor.read::<i8>().unwrap(), 2i8);
    assert_eq!(cursor.read::<i8>().unwrap(), 3i8);
    assert_eq!(cursor.read::<i8>().unwrap(), 4i8);
    assert_eq!(cursor.read::<i8>().unwrap(), 5i8);
    assert_eq!(cursor.read::<i8>().unwrap(), 6i8);
    assert_eq!(cursor.read::<i8>().unwrap(), 7i8);
}

#[test]
fn u16() {
    let mut cursor = TestCursor(Cursor::new(&[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]));

    assert_eq!(cursor.read::<u16>().unwrap(), 0u16);
    assert_eq!(cursor.read::<u16>().unwrap(), 1u16);
    assert_eq!(cursor.read::<u16>().unwrap(), 2u16);
    assert_eq!(cursor.read::<u16>().unwrap(), 3u16);
    assert_eq!(cursor.read::<u16>().unwrap(), 4u16);
    assert_eq!(cursor.read::<u16>().unwrap(), 5u16);
    assert_eq!(cursor.read::<u16>().unwrap(), 6u16);
    assert_eq!(cursor.read::<u16>().unwrap(), 7u16);
}

#[test]
fn i16() {
    let mut cursor = TestCursor(Cursor::new(&[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]));

    assert_eq!(cursor.read::<i16>().unwrap(), 0i16);
    assert_eq!(cursor.read::<i16>().unwrap(), 1i16);
    assert_eq!(cursor.read::<i16>().unwrap(), 2i16);
    assert_eq!(cursor.read::<i16>().unwrap(), 3i16);
    assert_eq!(cursor.read::<i16>().unwrap(), 4i16);
    assert_eq!(cursor.read::<i16>().unwrap(), 5i16);
    assert_eq!(cursor.read::<i16>().unwrap(), 6i16);
    assert_eq!(cursor.read::<i16>().unwrap(), 7i16);
}

#[test]
fn u32() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0,
        0, 7,
    ]));

    assert_eq!(cursor.read::<u32>().unwrap(), 0u32);
    assert_eq!(cursor.read::<u32>().unwrap(), 1u32);
    assert_eq!(cursor.read::<u32>().unwrap(), 2u32);
    assert_eq!(cursor.read::<u32>().unwrap(), 3u32);
    assert_eq!(cursor.read::<u32>().unwrap(), 4u32);
    assert_eq!(cursor.read::<u32>().unwrap(), 5u32);
    assert_eq!(cursor.read::<u32>().unwrap(), 6u32);
    assert_eq!(cursor.read::<u32>().unwrap(), 7u32);
}

#[test]
fn i32() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0,
        0, 7,
    ]));

    assert_eq!(cursor.read::<i32>().unwrap(), 0i32);
    assert_eq!(cursor.read::<i32>().unwrap(), 1i32);
    assert_eq!(cursor.read::<i32>().unwrap(), 2i32);
    assert_eq!(cursor.read::<i32>().unwrap(), 3i32);
    assert_eq!(cursor.read::<i32>().unwrap(), 4i32);
    assert_eq!(cursor.read::<i32>().unwrap(), 5i32);
    assert_eq!(cursor.read::<i32>().unwrap(), 6i32);
    assert_eq!(cursor.read::<i32>().unwrap(), 7i32);
}

#[test]
fn u64() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0,
        0, 0, 0, 7,
    ]));

    assert_eq!(cursor.read::<u64>().unwrap(), 0u64);
    assert_eq!(cursor.read::<u64>().unwrap(), 1u64);
    assert_eq!(cursor.read::<u64>().unwrap(), 2u64);
    assert_eq!(cursor.read::<u64>().unwrap(), 3u64);
    assert_eq!(cursor.read::<u64>().unwrap(), 4u64);
    assert_eq!(cursor.read::<u64>().unwrap(), 5u64);
    assert_eq!(cursor.read::<u64>().unwrap(), 6u64);
    assert_eq!(cursor.read::<u64>().unwrap(), 7u64);
}

#[test]
fn i64() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0,
        0, 0, 0, 7,
    ]));

    assert_eq!(cursor.read::<i64>().unwrap(), 0i64);
    assert_eq!(cursor.read::<i64>().unwrap(), 1i64);
    assert_eq!(cursor.read::<i64>().unwrap(), 2i64);
    assert_eq!(cursor.read::<i64>().unwrap(), 3i64);
    assert_eq!(cursor.read::<i64>().unwrap(), 4i64);
    assert_eq!(cursor.read::<i64>().unwrap(), 5i64);
    assert_eq!(cursor.read::<i64>().unwrap(), 6i64);
    assert_eq!(cursor.read::<i64>().unwrap(), 7i64);
}

#[test]
fn u128() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 7,
    ]));

    assert_eq!(cursor.read::<u128>().unwrap(), 0u128);
    assert_eq!(cursor.read::<u128>().unwrap(), 1u128);
    assert_eq!(cursor.read::<u128>().unwrap(), 2u128);
    assert_eq!(cursor.read::<u128>().unwrap(), 3u128);
    assert_eq!(cursor.read::<u128>().unwrap(), 4u128);
    assert_eq!(cursor.read::<u128>().unwrap(), 5u128);
    assert_eq!(cursor.read::<u128>().unwrap(), 6u128);
    assert_eq!(cursor.read::<u128>().unwrap(), 7u128);
}

#[test]
fn i128() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 7,
    ]));

    assert_eq!(cursor.read::<i128>().unwrap(), 0i128);
    assert_eq!(cursor.read::<i128>().unwrap(), 1i128);
    assert_eq!(cursor.read::<i128>().unwrap(), 2i128);
    assert_eq!(cursor.read::<i128>().unwrap(), 3i128);
    assert_eq!(cursor.read::<i128>().unwrap(), 4i128);
    assert_eq!(cursor.read::<i128>().unwrap(), 5i128);
    assert_eq!(cursor.read::<i128>().unwrap(), 6i128);
    assert_eq!(cursor.read::<i128>().unwrap(), 7i128);
}

#[test]
fn f32() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 63, 128, 0, 0, 64, 0, 0, 0, 64, 64, 0, 0, 64, 128, 0, 0, 64, 160, 0, 0, 64,
        192, 0, 0, 64, 224, 0, 0, 65, 0, 0, 0,
    ]));

    assert!((cursor.read::<f32>().unwrap() - 0.0f32).abs() < f32::EPSILON);
    assert!((cursor.read::<f32>().unwrap() - 1.0f32).abs() < f32::EPSILON);
    assert!((cursor.read::<f32>().unwrap() - 2.0f32).abs() < f32::EPSILON);
    assert!((cursor.read::<f32>().unwrap() - 3.0f32).abs() < f32::EPSILON);
    assert!((cursor.read::<f32>().unwrap() - 4.0f32).abs() < f32::EPSILON);
    assert!((cursor.read::<f32>().unwrap() - 5.0f32).abs() < f32::EPSILON);
    assert!((cursor.read::<f32>().unwrap() - 6.0f32).abs() < f32::EPSILON);
    assert!((cursor.read::<f32>().unwrap() - 7.0f32).abs() < f32::EPSILON);
}

#[test]
fn f64() {
    let mut cursor = TestCursor(Cursor::new(&[
        0, 0, 0, 0, 0, 0, 0, 0, 63, 240, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0,
        0, 0, 0, 64, 16, 0, 0, 0, 0, 0, 0, 64, 20, 0, 0, 0, 0, 0, 0, 64, 24, 0, 0, 0, 0, 0, 0, 64,
        28, 0, 0, 0, 0, 0, 0, 64, 32, 0, 0,
    ]));

    assert!((cursor.read::<f64>().unwrap() - 0.0f64).abs() < f64::EPSILON);
    assert!((cursor.read::<f64>().unwrap() - 1.0f64).abs() < f64::EPSILON);
    assert!((cursor.read::<f64>().unwrap() - 2.0f64).abs() < f64::EPSILON);
    assert!((cursor.read::<f64>().unwrap() - 3.0f64).abs() < f64::EPSILON);
    assert!((cursor.read::<f64>().unwrap() - 4.0f64).abs() < f64::EPSILON);
    assert!((cursor.read::<f64>().unwrap() - 5.0f64).abs() < f64::EPSILON);
    assert!((cursor.read::<f64>().unwrap() - 6.0f64).abs() < f64::EPSILON);
    assert!((cursor.read::<f64>().unwrap() - 7.0f64).abs() < f64::EPSILON);
}
