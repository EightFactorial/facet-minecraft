//! Tests for serializing and deserializing variable-length [`u64`]s.
#![allow(clippy::unreadable_literal, reason = "Just a test")]

use facet_minecraft::{Standard, deserialize::Deserializer, serialize::Serializer};

#[test]
#[rustfmt::skip]
fn verify() {
    assert_eq(0u64, vec![0]);
    assert_eq(1u64, vec![1]);
    assert_eq(2u64, vec![2]);
    assert_eq(3u64, vec![3]);
    assert_eq(4u64, vec![4]);
    assert_eq(5u64, vec![5]);
    assert_eq(6u64, vec![6]);
    assert_eq(7u64, vec![7]);
    assert_eq(8u64, vec![8]);

    assert_eq(126u64, vec![126]);
    assert_eq(127u64, vec![127]);
    assert_eq(128u64, vec![128, 1]);
    assert_eq(129u64, vec![129, 1]);
    assert_eq(130u64, vec![130, 1]);

    assert_eq(253u64, vec![253, 1]);
    assert_eq(254u64, vec![254, 1]);
    assert_eq(255u64, vec![255, 1]);
    assert_eq(256u64, vec![128, 2]);
    assert_eq(257u64, vec![129, 2]);
    assert_eq(258u64, vec![130, 2]);

    assert_eq(25563u64, vec![219, 199, 1]);
    assert_eq(25564u64, vec![220, 199, 1]);
    assert_eq(25565u64, vec![221, 199, 1]);
    assert_eq(25566u64, vec![222, 199, 1]);
    assert_eq(25567u64, vec![223, 199, 1]);

    assert_eq(65530u64, vec![250, 255, 3]);
    assert_eq(65531u64, vec![251, 255, 3]);
    assert_eq(65532u64, vec![252, 255, 3]);
    assert_eq(65533u64, vec![253, 255, 3]);
    assert_eq(65534u64, vec![254, 255, 3]);
    assert_eq(65535u64, vec![255, 255, 3]);

    assert_eq(2097150u64, vec![254, 255, 127]);
    assert_eq(2097151u64, vec![255, 255, 127]);
    assert_eq(2097152u64, vec![128, 128, 128, 1]);
    assert_eq(2097153u64, vec![129, 128, 128, 1]);

    assert_eq(2147483646, vec![254, 255, 255, 255, 7]);
    assert_eq(2147483647, vec![255, 255, 255, 255, 7]);
    assert_eq(2147483648, vec![128, 128, 128, 128, 8]);
    assert_eq(2147483649, vec![129, 128, 128, 128, 8]);
    assert_eq(2147483650, vec![130, 128, 128, 128, 8]);

    assert_eq(4294967290, vec![250, 255, 255, 255, 15]);
    assert_eq(4294967291, vec![251, 255, 255, 255, 15]);
    assert_eq(4294967292, vec![252, 255, 255, 255, 15]);
    assert_eq(4294967293, vec![253, 255, 255, 255, 15]);
    assert_eq(4294967294, vec![254, 255, 255, 255, 15]);
    assert_eq(4294967295, vec![255, 255, 255, 255, 15]);
    assert_eq(4294967296, vec![128, 128, 128, 128, 16]);
    assert_eq(4294967297, vec![129, 128, 128, 128, 16]);
    assert_eq(4294967298, vec![130, 128, 128, 128, 16]);

    assert_eq(9223372036854775806, vec![254, 255, 255, 255, 255, 255, 255, 255, 127]);
    assert_eq(9223372036854775807, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
    assert_eq(9223372036854775808, vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
    assert_eq(9223372036854775809, vec![129, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
    assert_eq(9223372036854775810, vec![130, 128, 128, 128, 128, 128, 128, 128, 128, 1]);

    assert_eq(18446744073709551610, vec![250, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    assert_eq(18446744073709551611, vec![251, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    assert_eq(18446744073709551612, vec![252, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    assert_eq(18446744073709551613, vec![253, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    assert_eq(18446744073709551614, vec![254, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    assert_eq(18446744073709551615, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
}

fn assert_eq(var: u64, bytes: Vec<u8>) {
    let mut buffer = Vec::new();
    Standard.serialize_var_u64(var, &mut buffer).unwrap();
    assert_eq!(buffer, bytes, "Expected {var} to serialize as {bytes:?}!");

    let (de, rem) = Standard.deserialize_var_u64(&bytes).unwrap();
    assert_eq!(de, var, "Expected {bytes:?} to deserialize as {var}!");
    assert!(rem.is_empty(), "Found remaining bytes after deserialization: {rem:?}!");
}
