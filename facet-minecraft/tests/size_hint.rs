//! Verify size hints for various data structures.
#![allow(clippy::std_instead_of_alloc, reason = "`std` example")]
#![allow(clippy::zero_sized_map_values, reason = "Used by example")]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use facet::Facet;
use facet_minecraft::{self as mc, Serialize, hint::TypeSizeHint};
use uuid::Uuid;

#[test]
fn verify() {
    // Size: Exactly `[u8; 0]`
    assert::<()>(TypeSizeHint::Exact { size: 0 });
    assert::<&()>(TypeSizeHint::Exact { size: 0 });

    // Size: Exactly `[u8; 1]`
    assert::<bool>(TypeSizeHint::Exact { size: 1 });
    assert::<&bool>(TypeSizeHint::Exact { size: 1 });
    assert::<u8>(TypeSizeHint::Exact { size: 1 });
    assert::<&u8>(TypeSizeHint::Exact { size: 1 });
    assert::<i8>(TypeSizeHint::Exact { size: 1 });
    assert::<&i8>(TypeSizeHint::Exact { size: 1 });

    // Size: Exactly `[u8; 2]`
    assert::<u16>(TypeSizeHint::Exact { size: 2 });
    assert::<&u16>(TypeSizeHint::Exact { size: 2 });
    assert::<i16>(TypeSizeHint::Exact { size: 2 });
    assert::<&i16>(TypeSizeHint::Exact { size: 2 });
    // // Size: Min `[u8; 1]`, Max `[u8; 3]`
    // assert::<Var<u16>>(TypeSerializeHint::Range { min: 1, max: Some(3) });
    // assert::<Var<i16>>(TypeSerializeHint::Range { min: 1, max: Some(3) });

    // Size: Exactly `[u8; 4]`
    assert::<u32>(TypeSizeHint::Exact { size: 4 });
    assert::<&u32>(TypeSizeHint::Exact { size: 4 });
    assert::<i32>(TypeSizeHint::Exact { size: 4 });
    assert::<&i32>(TypeSizeHint::Exact { size: 4 });
    assert::<f32>(TypeSizeHint::Exact { size: 4 });
    assert::<&f32>(TypeSizeHint::Exact { size: 4 });
    // // Size: Min `[u8; 1]`, Max `[u8; 5]`
    // assert::<Var<u32>>(TypeSerializeHint::Range { min: 1, max: Some(5) });
    // assert::<Var<i32>>(TypeSerializeHint::Range { min: 1, max: Some(5) });

    // Size: Exactly `[u8; 8]`
    assert::<u64>(TypeSizeHint::Exact { size: 8 });
    assert::<&u64>(TypeSizeHint::Exact { size: 8 });
    assert::<i64>(TypeSizeHint::Exact { size: 8 });
    assert::<&i64>(TypeSizeHint::Exact { size: 8 });
    assert::<f64>(TypeSizeHint::Exact { size: 8 });
    assert::<&f64>(TypeSizeHint::Exact { size: 8 });
    // // Size: Min `[u8; 1]`, Max `[u8; 10]`
    // assert::<Var<u64>>(TypeSerializeHint::Range { min: 1, max: Some(10) });
    // assert::<Var<i64>>(TypeSerializeHint::Range { min: 1, max: Some(10) });

    // Size: Exactly `[u8; 16]`
    assert::<u128>(TypeSizeHint::Exact { size: 16 });
    assert::<&u128>(TypeSizeHint::Exact { size: 16 });
    assert::<i128>(TypeSizeHint::Exact { size: 16 });
    assert::<&i128>(TypeSizeHint::Exact { size: 16 });
    assert::<Uuid>(TypeSizeHint::Exact { size: 16 });
    assert::<&Uuid>(TypeSizeHint::Exact { size: 16 });
    // // Size: Min `[u8; 1]`, Max `[u8; 19]`
    // assert::<Var<u128>>(TypeSerializeHint::Range { min: 1, max: Some(19) });
    // assert::<Var<i128>>(TypeSerializeHint::Range { min: 1, max: Some(19) });

    // Size: Min `[u8; 1]`, Max: Unbounded
    assert::<&[u8]>(TypeSizeHint::Range { min: 1, max: None });
    assert::<&str>(TypeSizeHint::Range { min: 1, max: None });
    assert::<Vec<u8>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<String>(TypeSizeHint::Range { min: 1, max: None });

    // Size: Min `[u8; 1]`, Max: Unbounded
    assert::<Vec<u16>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<Vec<u32>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<Vec<u64>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<Vec<String>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<Vec<Vec<()>>>(TypeSizeHint::Range { min: 1, max: None });

    // Size: Min `[u8; 1]`, Max: Unbounded
    assert::<HashSet<u8>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<HashSet<u16>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<HashMap<u32, u8>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<HashMap<u64, String>>(TypeSizeHint::Range { min: 1, max: None });
    assert::<HashMap<u128, HashMap<u64, String>>>(TypeSizeHint::Range { min: 1, max: None });

    // Size: Min `[u8; 1]`, Max `[u8; 5]` (length repr)
    assert::<Vec<()>>(TypeSizeHint::Range { min: 1, max: Some(5) });
    assert::<HashSet<()>>(TypeSizeHint::Range { min: 1, max: Some(5) });
    assert::<HashMap<(), ()>>(TypeSizeHint::Range { min: 1, max: Some(5) });
    assert::<BTreeMap<(), ()>>(TypeSizeHint::Range { min: 1, max: Some(5) });
    assert::<BTreeSet<()>>(TypeSizeHint::Range { min: 1, max: Some(5) });

    // Size: None (Unsupported)
    assert::<char>(TypeSizeHint::None);
    assert::<*const u8>(TypeSizeHint::None);

    // Size: None (Unsupported)
    assert::<Var<()>>(TypeSizeHint::None);
    assert::<Var<u8>>(TypeSizeHint::None);
    assert::<Var<&u8>>(TypeSizeHint::None);
    assert::<Var<i8>>(TypeSizeHint::None);
    assert::<Var<&i8>>(TypeSizeHint::None);
    assert::<Var<f32>>(TypeSizeHint::None);
    assert::<&Var<f32>>(TypeSizeHint::None);
    assert::<Var<f64>>(TypeSizeHint::None);
    assert::<&Var<f64>>(TypeSizeHint::None);
}

// -------------------------------------------------------------------------------------------------

/// A helper struct with a variable-size field.
#[derive(Facet)]
struct Var<T>(#[facet(mc::variable)] T);

/// A helper function to verify the [`TypeSerializeHint`] of a given type.
fn assert<'facet, T: Serialize<'facet>>(hint: TypeSizeHint) {
    assert_eq!(
        &hint,
        T::SIZE_HINT,
        "The size of `{}` does not match the expected value!\nDef:\n  {:?}\nType:\n  {:?}\n",
        T::SHAPE.type_name(),
        T::SHAPE.def,
        T::SHAPE.ty,
    );
}
