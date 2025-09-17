//! Tests for serialization.
#![allow(
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    reason = "These are tests, not library code"
)]

use facet_minecraft::{
    Standard,
    deserialize::Deserializer,
    serialize::{SliceCursor, serialize},
};
use proptest::prelude::*;

// Tests for primitive types
proptest! {
    #[test]
    fn serialize_bool(input in prop::bool::ANY) {
        let mut slice = [0u8; 1];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(slice[0] != 0, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_u8(input in prop::bits::u8::ANY) {
        let mut slice = [0u8; 1];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(slice[0], input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_u16(input in prop::bits::u16::ANY) {
        let mut slice = [0u8; 2];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(u16::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_u32(input in prop::bits::u32::ANY) {
        let mut slice = [0u8; 4];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(u32::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_u64(input in prop::bits::u64::ANY) {
        let mut slice = [0u8; 8];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(u64::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_u128(input_a in prop::bits::u64::ANY, input_b in prop::bits::u64::ANY) {
        let input = u128::from(input_a) << 64 | u128::from(input_b);
        let mut slice = [0u8; 16];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(u128::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_wrap, reason = "Casting `u8` to `i8` is intentional")]
    fn serialize_i8(input in prop::bits::i8::ANY) {
        let mut slice = [0u8; 1];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(slice[0] as i8, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_i16(input in prop::bits::i16::ANY) {
        let mut slice = [0u8; 2];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(i16::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_i32(input in prop::bits::i32::ANY) {
        let mut slice = [0u8; 4];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(i32::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_i64(input in prop::bits::i64::ANY) {
        let mut slice = [0u8; 8];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(i64::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_i128(input_a in prop::bits::i64::ANY, input_b in prop::bits::i64::ANY) {
        let input = i128::from(input_a) << 64 | i128::from(input_b);
        let mut slice = [0u8; 16];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(i128::from_be_bytes(slice), input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_string(input in ".*") {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, remaining) = Standard.deserialize_var_usize(&buffer).unwrap();
                let string = core::str::from_utf8(remaining).unwrap();
                assert_eq!(len, input.len());
                assert_eq!(string, input);
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}

// Tests for Arrays and Vectors
proptest! {
    #[test]
    fn serialize_array_u8(input in prop::array::uniform4(prop::bits::u8::ANY)) {
        let mut slice = [0u8; 4];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(slice, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_vector_u8(input in prop::collection::vec(prop::bits::u8::ANY, 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, remaining) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());
                assert_eq!(remaining, input.as_slice());
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_vector_u32(input in prop::collection::vec(prop::bits::u32::ANY, 0..64)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, remaining) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());
                let deserialized: Vec<u32> = remaining.as_chunks::<4>().0.iter()
                    .map(|bytes| u32::from_be_bytes(*bytes))
                    .collect();
                assert_eq!(deserialized, input);
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_array_i64(input in prop::array::uniform8(prop::bits::i64::ANY)) {
        let mut slice = [0u8; 64];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => {
                for (i, chunk) in slice.chunks_exact(8).enumerate() {
                    assert_eq!(i64::from_be_bytes(chunk.try_into().unwrap()), input[i]);
                }
            }
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_vector_string(input in prop::collection::vec(".*", 0..32)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (string_count, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(string_count, input.len());

                for original in &input {
                    let (len, remaining) = Standard.deserialize_var_usize(rest).unwrap();
                    let (string, remaining) = remaining.split_at(len);
                    rest = remaining;

                    let string = core::str::from_utf8(string).unwrap();
                    assert_eq!(len, original.len());
                    assert_eq!(string, original);
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}

// Tests for Maps and Sets
proptest! {
    #[test]
    fn serialize_hashset_u8(input in prop::collection::hash_set(prop::bits::u8::ANY, 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, remaining) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for value in input {
                    assert!(remaining.contains(&value));
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_hashset_str(input in prop::collection::hash_set(".*", 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for _ in 0..len {
                    let (str_len, remaining) = Standard.deserialize_var_usize(rest).unwrap();
                    let (string, remaining) = remaining.split_at(str_len);
                    rest = remaining;

                    let string = core::str::from_utf8(string).unwrap();
                    assert!(input.contains(string));
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_btreeset_i32(input in prop::collection::btree_set(prop::bits::i32::ANY, 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for _ in 0..len {
                    let (value_bytes, remaining) = rest.split_at(4);
                    rest = remaining;

                    let value = i32::from_be_bytes(value_bytes.try_into().unwrap());
                    assert!(input.contains(&value));
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_btreeset_string(input in prop::collection::btree_set(".*", 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for _ in 0..len {
                    let (str_len, remaining) = Standard.deserialize_var_usize(rest).unwrap();
                    let (string, remaining) = remaining.split_at(str_len);
                    rest = remaining;

                    let string = core::str::from_utf8(string).unwrap();
                    assert!(input.contains(string));
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_hashmap_u8_u64(input in prop::collection::hash_map(prop::bits::u8::ANY, prop::bits::u64::ANY, 0..32)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for (k, v) in &input {
                    let (key, remaining) = rest.split_at(1);
                    let (value, remaining) = remaining.split_at(8);
                    rest = remaining;

                    assert_eq!(key[0], *k);
                    assert_eq!(u64::from_be_bytes(value.try_into().unwrap()), *v);
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_hashmap_string_i32(input in prop::collection::hash_map(".*", prop::bits::i32::ANY, 0..32)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for (k, v) in &input {
                    let (key_len, remaining) = Standard.deserialize_var_usize(rest).unwrap();
                    let (key, remaining) = remaining.split_at(key_len);
                    let (value, remaining) = remaining.split_at(4);
                    rest = remaining;

                    let key_str = core::str::from_utf8(key).unwrap();
                    assert_eq!(key_str, k);
                    assert_eq!(i32::from_be_bytes(value.try_into().unwrap()), *v);
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_btreemap_i16_string(input in prop::collection::btree_map(prop::bits::i16::ANY, ".*", 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for (k, v) in &input {
                    let (key, remaining) = rest.split_at(2);
                    let (value_len, remaining) = Standard.deserialize_var_usize(remaining).unwrap();
                    let (value, remaining) = remaining.split_at(value_len);
                    rest = remaining;

                    let value_str = core::str::from_utf8(value).unwrap();
                    assert_eq!(i16::from_be_bytes(key.try_into().unwrap()), *k);
                    assert_eq!(value_str, v);
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_btreemap_string_u32(input in prop::collection::btree_map(".*", prop::bits::u32::ANY, 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for (k, v) in &input {
                    let (key_len, remaining) = Standard.deserialize_var_usize(rest).unwrap();
                    let (key, remaining) = remaining.split_at(key_len);
                    let (value, remaining) = remaining.split_at(4);
                    rest = remaining;

                    let key_str = core::str::from_utf8(key).unwrap();
                    assert_eq!(key_str, k);
                    assert_eq!(u32::from_be_bytes(value.try_into().unwrap()), *v);
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}

// Tests for Options
proptest! {
    #[test]
    fn serialize_option_unit(input in prop::option::of(Just(()))) {
        let mut slice = [0u8; 1];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => assert_eq!(slice[0] == 0, input.is_none()),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_option_u8(input in prop::option::of(prop::bits::u8::ANY)) {
        let mut slice = [0u8; 2];
        match serialize(&input, &mut SliceCursor::new(&mut slice)) {
            Ok(()) => match input {
                Some(value) => assert_eq!(slice, [0x01, value]),
                None => assert_eq!(slice, [0x00, 0x00]),
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_option_string(input in prop::option::of(".*")) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                if let Some(value) = &input {
                    let (len, remaining) = Standard.deserialize_var_usize(&buffer[1..]).unwrap();
                    let string = core::str::from_utf8(remaining).unwrap();
                    assert_eq!(len, value.len());
                    assert_eq!(string, value);
                } else {
                    assert_eq!(buffer, vec![0x00]);
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_option_hashmap_u32_string(input in prop::option::of(prop::collection::hash_map(prop::bits::u32::ANY, ".*", 0..16))) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                if let Some(map) = &input {
                    let (len, mut rest) = Standard.deserialize_var_usize(&buffer[1..]).unwrap();
                    assert_eq!(len, map.len());

                    for (k, v) in map {
                        let (key_bytes, remaining) = rest.split_at(4);
                        let (value_len, remaining) = Standard.deserialize_var_usize(remaining).unwrap();
                        let (value, remaining) = remaining.split_at(value_len);
                        rest = remaining;

                        let value_str = core::str::from_utf8(value).unwrap();
                        assert_eq!(u32::from_be_bytes(key_bytes.try_into().unwrap()), *k);
                        assert_eq!(value_str, v);
                    }
                } else {
                    assert_eq!(buffer, vec![0x00]);
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn serialize_btreemap_u8_option_u32(input in prop::collection::btree_map(prop::bits::u8::ANY, prop::option::of(prop::bits::u32::ANY), 0..16)) {
        let mut buffer = Vec::new();
        match serialize(&input, &mut buffer) {
            Ok(()) => {
                let (len, mut rest) = Standard.deserialize_var_usize(&buffer).unwrap();
                assert_eq!(len, input.len());

                for (k, v) in &input {
                    let (key_bytes, remaining) = rest.split_at(1);
                    let (option_byte, remaining) = remaining.split_at(1);
                    rest = remaining;

                    assert_eq!(key_bytes[0], *k);
                    match v {
                        Some(value) => {
                            let (value_bytes, remaining) = rest.split_at(4);
                            rest = remaining;
                            assert_eq!(option_byte[0], 0x01);
                            assert_eq!(u32::from_be_bytes(value_bytes.try_into().unwrap()), *value);
                        },
                        None => {
                            assert_eq!(option_byte[0], 0x00);
                        },
                    }
                }
            },
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}
