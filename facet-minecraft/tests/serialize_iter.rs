//! TODO
#![allow(dead_code, reason = "Used in tests")]
#![no_std]

extern crate alloc;

use facet::Facet;
use facet_minecraft::{
    self as mc, SerializeFn,
    serialize::iter::{PeekValue, SerializeIter},
};

#[test]
fn primitive() {
    let mut iter = SerializeIter::new(&0u8).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&0i8).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&256u128).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U128(256))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&256i128).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U128(256))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new("string").unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(6))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bytes(b"string"))));
    assert!(iter.next().is_none());
}

#[test]
fn list() {
    static ARRAY: &'static [u8; 4] = &[0u8; 4];
    static SLICE: &'static [u8] = &[0u8; 4];

    let mut iter = SerializeIter::new(ARRAY).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(SLICE).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(4))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(0))));
    assert!(iter.next().is_none());

    let vec = alloc::vec![0u32; 4];
    let mut iter = SerializeIter::new(&vec).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(4))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(0))));
    assert!(iter.next().is_none());
}

#[test]
fn map() {
    let mut bset = alloc::collections::BTreeSet::new();
    for i in 0..8 {
        bset.insert(i);
    }
    let mut iter = SerializeIter::new(&bset).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(8))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(2))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(3))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(4))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(5))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(6))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(7))));
    assert!(iter.next().is_none());

    let mut bmap = alloc::collections::BTreeMap::new();
    for i in 0..8u32 {
        bmap.insert(i, 255);
    }
    let mut iter = SerializeIter::new(&bmap).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(8))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(0))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(2))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(3))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(4))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(5))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(6))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(7))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(255))));
    assert!(iter.next().is_none());
}

#[test]
fn r#struct() {
    #[derive(Facet)]
    struct Unit;

    #[derive(Facet)]
    struct NewType(&'static str);

    #[derive(Facet)]
    struct Generic<T>(T);

    #[derive(Facet)]
    struct Fields {
        a: u8,
        b: u16,
        c: bool,
    }

    #[derive(Facet)]
    struct Variable {
        a: u8,
        #[facet(mc::variable)]
        b: u16,
        c: bool,
    }

    #[derive(Facet)]
    struct OptionalVariable {
        #[facet(mc::variable)]
        a: u16,
        #[facet(mc::variable)]
        b: Option<u16>,
        c: Variable,
    }

    #[derive(Facet)]
    struct Custom {
        a: u8,
        #[facet(mc::serialize = SerializeFn::new(|_, _| todo!()))]
        b: u16,
        c: bool,
    }

    let mut iter = SerializeIter::new(&Unit).unwrap();
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&NewType("string2")).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(7))));
    assert!(iter.next().is_some_and(|r| r.is_ok_and(|val| val == PeekValue::Bytes(b"string2"))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Generic(NewType("string3"))).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(7))));
    assert!(iter.next().is_some_and(|r| r.is_ok_and(|val| val == PeekValue::Bytes(b"string3"))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Fields { a: 1, b: 2, c: false }).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U16(2))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bool(false))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Variable { a: 1, b: 2, c: false }).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(2))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bool(false))));
    assert!(iter.next().is_none());

    let input = OptionalVariable { a: 1, b: Some(2), c: Variable { a: 1, b: 2, c: false } };
    let mut iter = SerializeIter::new(&input).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(2))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(2))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bool(false))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Custom { a: 1, b: 2, c: false }).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U8(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|v| matches!(v, PeekValue::Custom(_, _)))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bool(false))));
    assert!(iter.next().is_none());
}

#[test]
fn r#enum() {
    #[repr(i8)]
    #[derive(Facet)]
    enum Example {
        Unit,
        UnitExplicit(()),
        Single(bool),
        Tuple(bool, bool),
        Fields { a: &'static str, b: Option<u32> },
        Negative = -1,
    }

    let mut iter = SerializeIter::new(&Example::Unit).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(0))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Example::UnitExplicit(())).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(1))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Example::Single(true)).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(2))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bool(true))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Example::Tuple(true, true)).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(3))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bool(true))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bool(true))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Example::Fields { a: "123", b: None }).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(4))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(3))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bytes(b"123"))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(0))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Example::Fields { a: "123", b: Some(123) }).unwrap();
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(4))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(3))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Bytes(b"123"))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::Variable(1))));
    assert!(iter.next().is_some_and(|res| res.is_ok_and(|val| val == PeekValue::U32(123))));
    assert!(iter.next().is_none());

    let mut iter = SerializeIter::new(&Example::Negative).unwrap();
    assert_eq!((-1i128).to_le_bytes(), 340282366920938463463374607431768211455u128.to_le_bytes());
    assert!(iter.next().is_some_and(|res| {
        res.is_ok_and(|val| val == PeekValue::Variable(340282366920938463463374607431768211455u128))
    }));
    assert!(iter.next().is_none());
}
