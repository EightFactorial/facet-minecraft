//! Tests for serializing and deserializing variable-length [`u16`]s.
use facet_minecraft::{Standard, deserialize::Deserializer, serialize::Serializer};

#[test]
fn verify() {
    assert_eq(0, vec![0]);
    assert_eq(1, vec![1]);
    assert_eq(2, vec![2]);
    assert_eq(3, vec![3]);
    assert_eq(4, vec![4]);
    assert_eq(5, vec![5]);
    assert_eq(6, vec![6]);
    assert_eq(7, vec![7]);
    assert_eq(8, vec![8]);

    assert_eq(126, vec![126]);
    assert_eq(127, vec![127]);
    assert_eq(128, vec![128, 1]);
    assert_eq(129, vec![129, 1]);
    assert_eq(130, vec![130, 1]);

    assert_eq(253, vec![253, 1]);
    assert_eq(254, vec![254, 1]);
    assert_eq(255, vec![255, 1]);
    assert_eq(256, vec![128, 2]);
    assert_eq(257, vec![129, 2]);
    assert_eq(258, vec![130, 2]);

    assert_eq(25563, vec![219, 199, 1]);
    assert_eq(25564, vec![220, 199, 1]);
    assert_eq(25565, vec![221, 199, 1]);
    assert_eq(25566, vec![222, 199, 1]);
    assert_eq(25567, vec![223, 199, 1]);

    assert_eq(65530, vec![250, 255, 3]);
    assert_eq(65531, vec![251, 255, 3]);
    assert_eq(65532, vec![252, 255, 3]);
    assert_eq(65533, vec![253, 255, 3]);
    assert_eq(65534, vec![254, 255, 3]);
    assert_eq(65535, vec![255, 255, 3]);
}

fn assert_eq(var: u16, bytes: Vec<u8>) {
    let mut buffer = Vec::new();
    Standard.serialize_var_u16(var, &mut buffer).unwrap();
    assert_eq!(buffer, bytes, "Expected {var} to serialize as {bytes:?}!");

    let (de, rem) = Standard.deserialize_var_u16(&bytes).unwrap();
    assert_eq!(de, var, "Expected {bytes:?} to deserialize as {var}!");
    assert!(rem.is_empty(), "Found remaining bytes after deserialization: {rem:?}!");
}
