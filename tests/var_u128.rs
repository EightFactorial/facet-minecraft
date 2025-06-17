//! TODO
use facet_minecraft::{McSerializer, SerializerExt};

#[test]
#[rustfmt::skip]
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

    assert_eq(2097150, vec![254, 255, 127]);
    assert_eq(2097151, vec![255, 255, 127]);
    assert_eq(2097152, vec![128, 128, 128, 1]);
    assert_eq(2097153, vec![129, 128, 128, 1]);

    assert_eq(2147483646, vec![254, 255, 255, 255, 7]);
    assert_eq(2147483647, vec![255, 255, 255, 255, 7]);
    assert_eq(2147483648, vec![128, 128, 128, 128, 8]);

    assert_eq(4294967290, vec![250, 255, 255, 255, 15]);
    assert_eq(4294967291, vec![251, 255, 255, 255, 15]);
    assert_eq(4294967292, vec![252, 255, 255, 255, 15]);
    assert_eq(4294967293, vec![253, 255, 255, 255, 15]);
    assert_eq(4294967294, vec![254, 255, 255, 255, 15]);
    assert_eq(4294967295, vec![255, 255, 255, 255, 15]);
    assert_eq(4294967296, vec![128, 128, 128, 128, 16]);

    assert_eq(9223372036854775806, vec![254, 255, 255, 255, 255, 255, 255, 255, 127]);
    assert_eq(9223372036854775807, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
    assert_eq(9223372036854775808, vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);

    // TODO: Fix
    // assert_eq(1844674407370955160, vec![250, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    // assert_eq(1844674407370955161, vec![251, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    // assert_eq(1844674407370955162, vec![252, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    // assert_eq(1844674407370955163, vec![253, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    // assert_eq(18446744073709551614, vec![254, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    // assert_eq(18446744073709551615, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
}

fn assert_eq(var: u64, bytes: Vec<u8>) {
    let mut ser = McSerializer(Vec::new());
    ser.serialize_var_u64(var).unwrap();
    assert_eq!(ser.0, bytes, "Expected {var} to serialize as {bytes:?}!");
}
