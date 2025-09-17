//! Tests for deserialization.
#![allow(
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    reason = "These are tests, not library code"
)]

use std::collections::{BTreeMap, HashMap};

use facet_minecraft::deserialize;
use proptest::prelude::*;

// Tests for primitive types
proptest! {
    #[test]
    fn deserialize_bool(input in prop::bits::u8::ANY) {
        let slice = [input];
        let result = deserialize::<bool>(&slice);
        match input {
            0x00 => assert!(!result.unwrap()),
            0x01 => assert!(result.unwrap()),
            _ => {
                result.unwrap_err();
            }
        }
    }

    #[test]
    fn deserialize_u8(input in prop::bits::u8::ANY) {
        match deserialize::<u8>(core::array::from_ref(&input)) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_u16(input in prop::bits::u16::ANY) {
        match deserialize::<u16>(input.to_be_bytes().as_slice()) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_u32(input in prop::bits::u32::ANY) {
        match deserialize::<u32>(input.to_be_bytes().as_slice()) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_u64(input in prop::bits::u64::ANY) {
        match deserialize::<u64>(input.to_be_bytes().as_slice()) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_u128(input_a in prop::bits::u64::ANY, input_b in prop::bits::u64::ANY) {
        let input = u128::from(input_a) << 64 | u128::from(input_b);
        match deserialize::<u128>(input.to_be_bytes().as_slice()){
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_sign_loss, reason = "Casting `i8` to `u8` is intentional")]
    fn deserialize_i8(input in prop::bits::i8::ANY) {
        let casted = input as u8;
        match deserialize::<i8>(core::array::from_ref(&casted)) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_i16(input in prop::bits::i16::ANY) {
        match deserialize::<i16>(input.to_be_bytes().as_slice()) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_i32(input in prop::bits::i32::ANY) {
        match deserialize::<i32>(input.to_be_bytes().as_slice()) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_i64(input in prop::bits::i64::ANY) {
        match deserialize::<i64>(input.to_be_bytes().as_slice()) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_i128(input_a in prop::bits::i64::ANY, input_b in prop::bits::i64::ANY) {
        let input = i128::from(input_a) << 64 | i128::from(input_b);
        match deserialize::<i128>(input.to_be_bytes().as_slice()) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 255")]
    fn deserialize_string(input in ".*") {
        let mut prefixed = vec![input.len() as u8];
        prefixed.extend(input.as_bytes());

        match deserialize::<&str>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}

// Tests for Arrays and Vectors
proptest! {
    #[test]
    fn deserialize_array_u8(input in prop::array::uniform4(prop::bits::u8::ANY)) {
        match deserialize::<[u8; 4]>(&input) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16")]
    fn deserialize_vector_u8(input in prop::collection::vec(prop::bits::u8::ANY, 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        prefixed.extend(input.iter());

        match deserialize::<Vec<u8>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 64")]
    fn deserialize_vector_u32(input in prop::collection::vec(prop::bits::u32::ANY, 0..64)) {
        let mut prefixed = vec![input.len() as u8];
        prefixed.extend(input.iter().flat_map(|x| x.to_be_bytes()));

        match deserialize::<Vec<u32>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_array_i64(input in prop::array::uniform8(prop::bits::i64::ANY)) {
        let bytes: Vec<u8> = input.iter().flat_map(|x| x.to_be_bytes()).collect();
        match deserialize::<[i64; 8]>(&bytes) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "String length is limited to 255, Vec to 32")]
    fn deserialize_vector_string(input in prop::collection::vec(".*", 0..32)) {
        let mut prefixed = vec![input.len() as u8];
        for s in &input {
            prefixed.push(s.len() as u8);
            prefixed.extend(s.as_bytes());
        }

        match deserialize::<Vec<String>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}

// Tests for Maps and Sets
proptest! {
    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16")]
    fn deserialize_hashset_u8(input in prop::collection::hash_set(prop::bits::u8::ANY, 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        prefixed.extend(input.iter());

        // TODO: Enable when `facet` supports Sets
        // match deserialize::<std::collections::HashSet<u8>>(&prefixed) {
        //     Ok(value) => assert_eq!(value, input),
        //     Err(err) => panic!("{}", err.as_report()),
        // }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16")]
    fn deserialize_hashset_str(input in prop::collection::hash_set(".*", 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        for s in &input {
            prefixed.push(s.len() as u8);
            prefixed.extend(s.as_bytes());
        }

        // TODO: Enable when `facet` supports Sets
        // match deserialize::<std::collections::HashSet<String>>(&prefixed) {
        //     Ok(value) => assert_eq!(value, input),
        //     Err(err) => panic!("{}", err.as_report()),
        // }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16")]
    fn deserialize_btreeset_i32(input in prop::collection::btree_set(prop::bits::i32::ANY, 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        for &s in &input {
            prefixed.extend(s.to_be_bytes());
        }

        // TODO: Enable when `facet` supports Sets
        // match deserialize::<std::collections::BTreeSet<i32>>(&prefixed) {
        //     Ok(value) => assert_eq!(value, input),
        //     Err(err) => panic!("{}", err.as_report()),
        // }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16")]
    fn deserialize_btreeset_string(input in prop::collection::btree_set(".*", 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        for s in &input {
            prefixed.push(s.len() as u8);
            prefixed.extend(s.as_bytes());
        }

        // TODO: Enable when `facet` supports Sets
        // match deserialize::<std::collections::BTreeSet<String>>(&prefixed) {
        //     Ok(value) => assert_eq!(value, input),
        //     Err(err) => panic!("{}", err.as_report()),
        // }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 32")]
    fn deserialize_hashmap_u8_u64(input in prop::collection::hash_map(prop::bits::u8::ANY, prop::bits::u64::ANY, 0..32)) {
        let mut prefixed = vec![input.len() as u8];
        for (k, v) in &input {
            prefixed.push(*k);
            prefixed.extend(v.to_be_bytes());
        }

        match deserialize::<HashMap<u8, u64>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 32")]
    fn deserialize_hashmap_string_i32(input in prop::collection::hash_map(".*", prop::bits::i32::ANY, 0..32)) {
        let mut prefixed = vec![input.len() as u8];
        for (k, v) in &input {
            prefixed.push(k.len() as u8);
            prefixed.extend(k.as_bytes());
            prefixed.extend(v.to_be_bytes());
        }

        match deserialize::<HashMap<String, i32>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16")]
    fn deserialize_btreemap_i16_string(input in prop::collection::btree_map(prop::bits::i16::ANY, ".*", 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        for (k, v) in &input {
            prefixed.extend(k.to_be_bytes());
            prefixed.push(v.len() as u8);
            prefixed.extend(v.as_bytes());
        }

        match deserialize::<BTreeMap<i16, String>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16")]
    fn deserialize_btreemap_string_u32(input in prop::collection::btree_map(".*", prop::bits::u32::ANY, 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        for (k, v) in &input {
            prefixed.push(k.len() as u8);
            prefixed.extend(k.as_bytes());
            prefixed.extend(v.to_be_bytes());
        }

        match deserialize::<BTreeMap<String, u32>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}

// Tests for Options
proptest! {
    #[test]
    fn deserialize_option_unit(input in prop::option::of(Just(()))) {
        let prefixed = match input {
            Some(()) => vec![0x01],
            None => vec![0x00],
        };

        match deserialize::<Option<()>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    fn deserialize_option_u8(input in prop::option::of(prop::bits::u8::ANY)) {
        let mut prefixed = vec![];
        match input {
            Some(value) => {
                prefixed.push(0x01);
                prefixed.push(value);
            },
            None => {
                prefixed.push(0x00);
            }
        }

        match deserialize::<Option<u8>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "String length is limited to 255")]
    fn deserialize_option_string(input in prop::option::of(".*")) {
        let mut prefixed = vec![];
        match input.as_ref() {
            Some(value) => {
                prefixed.push(0x01);
                prefixed.push(value.len() as u8);
                prefixed.extend(value.as_bytes());
            },
            None => {
                prefixed.push(0x00);
            }
        }

        match deserialize::<Option<String>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16") ]
    fn deserialize_option_hashmap_u32_string(input in prop::option::of(prop::collection::hash_map(prop::bits::u32::ANY, ".*", 0..16))) {
        let mut prefixed = vec![];
        match input.as_ref() {
            Some(map) => {
                prefixed.push(0x01);
                prefixed.push(map.len() as u8);
                for (k, v) in map {
                    prefixed.extend(k.to_be_bytes());
                    prefixed.push(v.len() as u8);
                    prefixed.extend(v.as_bytes());
                }
            },
            None => {
                prefixed.push(0x00);
            }
        }

        match deserialize::<Option<HashMap<u32, String>>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, reason = "Length is limited to 16") ]
    fn deserialize_btreemap_u8_option_u32(input in prop::collection::btree_map(prop::bits::u8::ANY, prop::option::of(prop::bits::u32::ANY), 0..16)) {
        let mut prefixed = vec![input.len() as u8];
        for (k, v) in &input {
            prefixed.push(*k);
            match v {
                Some(value) => {
                    prefixed.push(0x01);
                    prefixed.extend(value.to_be_bytes());
                },
                None => {
                    prefixed.push(0x00);
                }
            }
        }

        match deserialize::<BTreeMap<u8, Option<u32>>>(&prefixed) {
            Ok(value) => assert_eq!(value, input),
            Err(err) => panic!("{}", err.as_report()),
        }
    }
}
