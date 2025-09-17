#![expect(missing_docs, reason = "WIP")]

use alloc::{borrow::Cow, vec::Vec};

use zerocopy::big_endian::{F32, F64, I16, I32, I64};

use crate::{
    Mutf8Str, Mutf8String,
    map::{ByteCow, NbtMap},
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NbtItem<'a> {
    Byte(ByteCow<'a, i8>),
    Short(ByteCow<'a, I16>),
    Int(ByteCow<'a, I32>),
    Long(ByteCow<'a, I64>),
    Float(ByteCow<'a, F32>),
    Double(ByteCow<'a, F64>),
    ByteArray(ByteCow<'a, [i8]>),
    String(Cow<'a, Mutf8Str>),
    List(NbtListItem<'a>),
    Compound(NbtMap<'a>),
    IntArray(ByteCow<'a, [I32]>),
    LongArray(ByteCow<'a, [I64]>),
}

impl NbtItem<'_> {
    /// Attempts to get the inner data as a byte slice.
    ///
    /// Returns `None` if the item is a [`NbtItem::List`] or
    /// [`NbtItem::Compound`].
    #[must_use]
    pub const fn try_as_slice(&self) -> Option<&[u8]> {
        match self {
            NbtItem::Byte(b) => Some(b.as_slice()),
            NbtItem::Short(b) => Some(b.as_slice()),
            NbtItem::Int(b) => Some(b.as_slice()),
            NbtItem::Long(b) => Some(b.as_slice()),
            NbtItem::Float(b) => Some(b.as_slice()),
            NbtItem::Double(b) => Some(b.as_slice()),
            NbtItem::ByteArray(b) => Some(b.as_slice()),
            NbtItem::IntArray(b) => Some(b.as_slice()),
            NbtItem::LongArray(b) => Some(b.as_slice()),
            NbtItem::String(b) => match b {
                Cow::Borrowed(s) => Some(s.as_bytes()),
                Cow::Owned(s) => Some(s.as_mutf8_str().as_bytes()),
            },
            NbtItem::List(_) | NbtItem::Compound(_) => None,
        }
    }

    /// Convert this [`NbtItem`] into an owned copy, cloning any borrowed data
    /// in the process.
    #[must_use]
    pub fn into_owned(self) -> NbtItem<'static> {
        match self {
            NbtItem::Byte(b) => NbtItem::Byte(b.into_owned()),
            NbtItem::Short(b) => NbtItem::Short(b.into_owned()),
            NbtItem::Int(b) => NbtItem::Int(b.into_owned()),
            NbtItem::Long(b) => NbtItem::Long(b.into_owned()),
            NbtItem::Float(b) => NbtItem::Float(b.into_owned()),
            NbtItem::Double(b) => NbtItem::Double(b.into_owned()),
            NbtItem::ByteArray(b) => NbtItem::ByteArray(b.into_owned()),
            NbtItem::String(b) => NbtItem::String(Cow::Owned(b.into_owned())),
            NbtItem::List(b) => NbtItem::List(b.into_owned()),
            NbtItem::Compound(b) => NbtItem::Compound(b.into_owned()),
            NbtItem::IntArray(b) => NbtItem::IntArray(b.into_owned()),
            NbtItem::LongArray(b) => NbtItem::LongArray(b.into_owned()),
        }
    }
}

impl From<i8> for NbtItem<'static> {
    fn from(value: i8) -> Self { NbtItem::Byte(ByteCow::new(&value)) }
}
impl<'a> From<&'a i8> for NbtItem<'a> {
    fn from(value: &'a i8) -> Self { NbtItem::Byte(ByteCow::new_ref(value)) }
}

impl From<i16> for NbtItem<'static> {
    fn from(value: i16) -> Self { I16::from(value).into() }
}
impl From<I16> for NbtItem<'static> {
    fn from(value: I16) -> Self { NbtItem::Short(ByteCow::new(&value)) }
}
impl<'a> From<&'a I16> for NbtItem<'a> {
    fn from(value: &'a I16) -> Self { NbtItem::Short(ByteCow::new_ref(value)) }
}

impl From<i32> for NbtItem<'static> {
    fn from(value: i32) -> Self { I32::from(value).into() }
}
impl From<I32> for NbtItem<'static> {
    fn from(value: I32) -> Self { NbtItem::Int(ByteCow::new(&value)) }
}
impl<'a> From<&'a I32> for NbtItem<'a> {
    fn from(value: &'a I32) -> Self { NbtItem::Int(ByteCow::new_ref(value)) }
}

impl From<i64> for NbtItem<'static> {
    fn from(value: i64) -> Self { I64::from(value).into() }
}
impl From<I64> for NbtItem<'static> {
    fn from(value: I64) -> Self { NbtItem::Long(ByteCow::new(&value)) }
}
impl<'a> From<&'a I64> for NbtItem<'a> {
    fn from(value: &'a I64) -> Self { NbtItem::Long(ByteCow::new_ref(value)) }
}

impl From<f32> for NbtItem<'static> {
    fn from(value: f32) -> Self { F32::from(value).into() }
}
impl From<F32> for NbtItem<'static> {
    fn from(value: F32) -> Self { NbtItem::Float(ByteCow::new(&value)) }
}
impl<'a> From<&'a F32> for NbtItem<'a> {
    fn from(value: &'a F32) -> Self { NbtItem::Float(ByteCow::new_ref(value)) }
}

impl From<f64> for NbtItem<'static> {
    fn from(value: f64) -> Self { F64::from(value).into() }
}
impl From<F64> for NbtItem<'static> {
    fn from(value: F64) -> Self { NbtItem::Double(ByteCow::new(&value)) }
}
impl<'a> From<&'a F64> for NbtItem<'a> {
    fn from(value: &'a F64) -> Self { NbtItem::Double(ByteCow::new_ref(value)) }
}

impl From<Vec<i8>> for NbtItem<'static> {
    fn from(value: Vec<i8>) -> Self { NbtItem::ByteArray(ByteCow::new(&value)) }
}
impl<'a> From<&'a [i8]> for NbtItem<'a> {
    fn from(value: &'a [i8]) -> Self { NbtItem::ByteArray(ByteCow::new_ref(value)) }
}

impl From<Mutf8String> for NbtItem<'static> {
    fn from(value: Mutf8String) -> Self { NbtItem::String(Cow::Owned(value)) }
}
impl<'a> From<&'a Mutf8Str> for NbtItem<'a> {
    fn from(value: &'a Mutf8Str) -> Self { NbtItem::String(Cow::Borrowed(value)) }
}

impl<'a> From<NbtMap<'a>> for NbtItem<'a> {
    fn from(value: NbtMap<'a>) -> Self { NbtItem::Compound(value) }
}

impl<'a> From<NbtListItem<'a>> for NbtItem<'a> {
    fn from(value: NbtListItem<'a>) -> Self { NbtItem::List(value) }
}

impl From<Vec<i32>> for NbtItem<'static> {
    fn from(value: Vec<i32>) -> Self { value.into_iter().map(I32::from).collect::<Vec<_>>().into() }
}
impl From<Vec<I32>> for NbtItem<'static> {
    fn from(value: Vec<I32>) -> Self { NbtItem::IntArray(ByteCow::new(&value)) }
}
impl From<&[i32]> for NbtItem<'static> {
    fn from(value: &[i32]) -> Self {
        value.iter().copied().map(I32::from).collect::<Vec<_>>().into()
    }
}
impl<'a> From<&'a [I32]> for NbtItem<'a> {
    fn from(value: &'a [I32]) -> Self { NbtItem::IntArray(ByteCow::new_ref(value)) }
}

impl From<Vec<i64>> for NbtItem<'static> {
    fn from(value: Vec<i64>) -> Self { value.into_iter().map(I64::from).collect::<Vec<_>>().into() }
}
impl From<Vec<I64>> for NbtItem<'static> {
    fn from(value: Vec<I64>) -> Self { NbtItem::LongArray(ByteCow::new(&value)) }
}
impl From<&[i64]> for NbtItem<'static> {
    fn from(value: &[i64]) -> Self {
        value.iter().copied().map(I64::from).collect::<Vec<_>>().into()
    }
}
impl<'a> From<&'a [I64]> for NbtItem<'a> {
    fn from(value: &'a [I64]) -> Self { NbtItem::LongArray(ByteCow::new_ref(value)) }
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NbtListItem<'a> {
    Empty,
    Byte(ByteCow<'a, [i8]>),
    Short(ByteCow<'a, [I16]>),
    Int(ByteCow<'a, [I32]>),
    Long(ByteCow<'a, [I64]>),
    Float(ByteCow<'a, [F32]>),
    Double(ByteCow<'a, [F64]>),
    ByteArray(Cow<'a, [ByteCow<'a, [i8]>]>),
    String(Cow<'a, [Cow<'a, Mutf8Str>]>),
    List(Cow<'a, [NbtListItem<'a>]>),
    Compound(Cow<'a, [NbtMap<'a>]>),
    IntArray(Cow<'a, [ByteCow<'a, [I32]>]>),
    LongArray(Cow<'a, [ByteCow<'a, [I64]>]>),
}

impl NbtListItem<'_> {
    /// Attempts to get the inner data as a byte slice.
    ///
    /// Returns `None` if the item is a [`NbtListItem::List`],
    /// [`NbtListItem::Compound`], or nested array.
    #[must_use]
    pub const fn try_as_slice(&self) -> Option<&[u8]> {
        match self {
            NbtListItem::Empty => Some(&[]),
            NbtListItem::Byte(b) => Some(b.as_slice()),
            NbtListItem::Short(b) => Some(b.as_slice()),
            NbtListItem::Int(b) => Some(b.as_slice()),
            NbtListItem::Long(b) => Some(b.as_slice()),
            NbtListItem::Float(b) => Some(b.as_slice()),
            NbtListItem::Double(b) => Some(b.as_slice()),
            _ => None,
        }
    }

    /// Convert this [`NbtListItem`] into an owned copy, cloning any borrowed
    /// data in the process.
    #[must_use]
    pub fn into_owned(self) -> NbtListItem<'static> {
        match self {
            NbtListItem::Empty => NbtListItem::Empty,
            NbtListItem::Byte(b) => NbtListItem::Byte(b.into_owned()),
            NbtListItem::Short(b) => NbtListItem::Short(b.into_owned()),
            NbtListItem::Int(b) => NbtListItem::Int(b.into_owned()),
            NbtListItem::Long(b) => NbtListItem::Long(b.into_owned()),
            NbtListItem::Float(b) => NbtListItem::Float(b.into_owned()),
            NbtListItem::Double(b) => NbtListItem::Double(b.into_owned()),
            NbtListItem::ByteArray(b) => NbtListItem::ByteArray(match b {
                Cow::Borrowed(v) => Cow::Owned(v.iter().map(ByteCow::to_owned).collect()),
                Cow::Owned(v) => Cow::Owned(v.into_iter().map(ByteCow::into_owned).collect()),
            }),
            NbtListItem::String(b) => NbtListItem::String(match b {
                Cow::Borrowed(v) => {
                    Cow::Owned(v.iter().map(|s| Cow::Owned(s.clone().into_owned())).collect())
                }
                Cow::Owned(v) => {
                    Cow::Owned(v.into_iter().map(|s| Cow::Owned(s.into_owned())).collect())
                }
            }),
            NbtListItem::List(b) => NbtListItem::List(match b {
                Cow::Borrowed(v) => Cow::Owned(v.iter().map(|b| b.clone().into_owned()).collect()),
                Cow::Owned(v) => Cow::Owned(v.into_iter().map(NbtListItem::into_owned).collect()),
            }),
            NbtListItem::Compound(b) => NbtListItem::Compound(match b {
                Cow::Borrowed(v) => Cow::Owned(v.iter().map(|b| b.clone().into_owned()).collect()),
                Cow::Owned(v) => Cow::Owned(v.into_iter().map(NbtMap::into_owned).collect()),
            }),
            NbtListItem::IntArray(b) => NbtListItem::IntArray(match b {
                Cow::Borrowed(v) => Cow::Owned(v.iter().map(ByteCow::to_owned).collect()),
                Cow::Owned(v) => Cow::Owned(v.into_iter().map(ByteCow::into_owned).collect()),
            }),
            NbtListItem::LongArray(b) => NbtListItem::LongArray(match b {
                Cow::Borrowed(v) => Cow::Owned(v.iter().map(ByteCow::to_owned).collect()),
                Cow::Owned(v) => Cow::Owned(v.into_iter().map(ByteCow::into_owned).collect()),
            }),
        }
    }
}

impl From<Vec<i8>> for NbtListItem<'static> {
    fn from(value: Vec<i8>) -> Self { NbtListItem::Byte(ByteCow::new(&value)) }
}
impl<'a> From<&'a [i8]> for NbtListItem<'a> {
    fn from(value: &'a [i8]) -> Self { NbtListItem::Byte(ByteCow::new_ref(value)) }
}

impl From<Vec<i16>> for NbtListItem<'static> {
    fn from(value: Vec<i16>) -> Self { value.into_iter().map(I16::from).collect::<Vec<_>>().into() }
}
impl From<Vec<I16>> for NbtListItem<'static> {
    fn from(value: Vec<I16>) -> Self { NbtListItem::Short(ByteCow::new(&value)) }
}
impl From<&[i16]> for NbtListItem<'static> {
    fn from(value: &[i16]) -> Self {
        value.iter().copied().map(I16::from).collect::<Vec<_>>().into()
    }
}
impl<'a> From<&'a [I16]> for NbtListItem<'a> {
    fn from(value: &'a [I16]) -> Self { NbtListItem::Short(ByteCow::new_ref(value)) }
}

impl From<Vec<i32>> for NbtListItem<'static> {
    fn from(value: Vec<i32>) -> Self { value.into_iter().map(I32::from).collect::<Vec<_>>().into() }
}
impl From<Vec<I32>> for NbtListItem<'static> {
    fn from(value: Vec<I32>) -> Self { NbtListItem::Int(ByteCow::new(&value)) }
}
impl From<&[i32]> for NbtListItem<'static> {
    fn from(value: &[i32]) -> Self {
        value.iter().copied().map(I32::from).collect::<Vec<_>>().into()
    }
}
impl<'a> From<&'a [I32]> for NbtListItem<'a> {
    fn from(value: &'a [I32]) -> Self { NbtListItem::Int(ByteCow::new_ref(value)) }
}

impl From<Vec<i64>> for NbtListItem<'static> {
    fn from(value: Vec<i64>) -> Self { value.into_iter().map(I64::from).collect::<Vec<_>>().into() }
}
impl From<Vec<I64>> for NbtListItem<'static> {
    fn from(value: Vec<I64>) -> Self { NbtListItem::Long(ByteCow::new(&value)) }
}
impl From<&[i64]> for NbtListItem<'static> {
    fn from(value: &[i64]) -> Self {
        value.iter().copied().map(I64::from).collect::<Vec<_>>().into()
    }
}
impl<'a> From<&'a [I64]> for NbtListItem<'a> {
    fn from(value: &'a [I64]) -> Self { NbtListItem::Long(ByteCow::new_ref(value)) }
}

impl From<Vec<f32>> for NbtListItem<'static> {
    fn from(value: Vec<f32>) -> Self { value.into_iter().map(F32::from).collect::<Vec<_>>().into() }
}
impl From<Vec<F32>> for NbtListItem<'static> {
    fn from(value: Vec<F32>) -> Self { NbtListItem::Float(ByteCow::new(&value)) }
}
impl From<&[f32]> for NbtListItem<'static> {
    fn from(value: &[f32]) -> Self {
        value.iter().copied().map(F32::from).collect::<Vec<_>>().into()
    }
}
impl<'a> From<&'a [F32]> for NbtListItem<'a> {
    fn from(value: &'a [F32]) -> Self { NbtListItem::Float(ByteCow::new_ref(value)) }
}

impl From<Vec<f64>> for NbtListItem<'static> {
    fn from(value: Vec<f64>) -> Self { value.into_iter().map(F64::from).collect::<Vec<_>>().into() }
}
impl From<Vec<F64>> for NbtListItem<'static> {
    fn from(value: Vec<F64>) -> Self { NbtListItem::Double(ByteCow::new(&value)) }
}
impl From<&[f64]> for NbtListItem<'static> {
    fn from(value: &[f64]) -> Self {
        value.iter().copied().map(F64::from).collect::<Vec<_>>().into()
    }
}
impl<'a> From<&'a [F64]> for NbtListItem<'a> {
    fn from(value: &'a [F64]) -> Self { NbtListItem::Double(ByteCow::new_ref(value)) }
}

impl From<Vec<ByteCow<'static, [i8]>>> for NbtListItem<'static> {
    fn from(value: Vec<ByteCow<'static, [i8]>>) -> Self {
        NbtListItem::ByteArray(Cow::Owned(value))
    }
}
impl<'a> From<&'a [ByteCow<'a, [i8]>]> for NbtListItem<'a> {
    fn from(value: &'a [ByteCow<'a, [i8]>]) -> Self { NbtListItem::ByteArray(Cow::Borrowed(value)) }
}
impl<'a> From<&'a [&'a [i8]]> for NbtListItem<'a> {
    fn from(value: &'a [&'a [i8]]) -> Self {
        NbtListItem::ByteArray(Cow::Owned(value.iter().map(|s| ByteCow::new_ref(*s)).collect()))
    }
}

impl From<Vec<Cow<'static, Mutf8Str>>> for NbtListItem<'static> {
    fn from(value: Vec<Cow<'static, Mutf8Str>>) -> Self { NbtListItem::String(Cow::Owned(value)) }
}
impl<'a> From<&'a [Cow<'a, Mutf8Str>]> for NbtListItem<'a> {
    fn from(value: &'a [Cow<'a, Mutf8Str>]) -> Self { NbtListItem::String(Cow::Borrowed(value)) }
}
impl<'a> From<&'a [&'a Mutf8Str]> for NbtListItem<'a> {
    fn from(value: &'a [&'a Mutf8Str]) -> Self {
        NbtListItem::String(Cow::Owned(value.iter().map(|s| Cow::Borrowed(*s)).collect()))
    }
}

impl From<Vec<NbtListItem<'static>>> for NbtListItem<'static> {
    fn from(value: Vec<NbtListItem<'static>>) -> Self { NbtListItem::List(Cow::Owned(value)) }
}
impl<'a> From<&'a [NbtListItem<'a>]> for NbtListItem<'a> {
    fn from(value: &'a [NbtListItem<'a>]) -> Self { NbtListItem::List(Cow::Borrowed(value)) }
}

impl From<Vec<NbtMap<'static>>> for NbtListItem<'static> {
    fn from(value: Vec<NbtMap<'static>>) -> Self { NbtListItem::Compound(Cow::Owned(value)) }
}
impl<'a> From<&'a [NbtMap<'a>]> for NbtListItem<'a> {
    fn from(value: &'a [NbtMap<'a>]) -> Self { NbtListItem::Compound(Cow::Borrowed(value)) }
}

impl From<Vec<Vec<i32>>> for NbtListItem<'static> {
    fn from(value: Vec<Vec<i32>>) -> Self {
        value
            .into_iter()
            .map(|v| v.into_iter().map(I32::from).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .into()
    }
}
impl From<Vec<Vec<I32>>> for NbtListItem<'static> {
    fn from(value: Vec<Vec<I32>>) -> Self {
        NbtListItem::IntArray(Cow::Owned(
            value.into_iter().map(|v| ByteCow::new(v.as_slice())).collect(),
        ))
    }
}
impl From<&[&[i32]]> for NbtListItem<'static> {
    fn from(value: &[&[i32]]) -> Self {
        NbtListItem::IntArray(Cow::Owned(
            value
                .iter()
                .map(|v| {
                    ByteCow::new(v.iter().copied().map(I32::from).collect::<Vec<_>>().as_slice())
                })
                .collect(),
        ))
    }
}
impl<'a> From<&'a [&'a [I32]]> for NbtListItem<'a> {
    fn from(value: &'a [&'a [I32]]) -> Self {
        NbtListItem::IntArray(Cow::Owned(value.iter().map(|v| ByteCow::new_ref(*v)).collect()))
    }
}

impl From<Vec<Vec<i64>>> for NbtListItem<'static> {
    fn from(value: Vec<Vec<i64>>) -> Self {
        value
            .into_iter()
            .map(|v| v.into_iter().map(I64::from).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .into()
    }
}
impl From<Vec<Vec<I64>>> for NbtListItem<'static> {
    fn from(value: Vec<Vec<I64>>) -> Self {
        NbtListItem::LongArray(Cow::Owned(
            value.into_iter().map(|v| ByteCow::new(v.as_slice())).collect(),
        ))
    }
}
impl From<&[&[i64]]> for NbtListItem<'static> {
    fn from(value: &[&[i64]]) -> Self {
        NbtListItem::LongArray(Cow::Owned(
            value
                .iter()
                .map(|v| {
                    ByteCow::new(v.iter().copied().map(I64::from).collect::<Vec<_>>().as_slice())
                })
                .collect(),
        ))
    }
}
impl<'a> From<&'a [&'a [I64]]> for NbtListItem<'a> {
    fn from(value: &'a [&'a [I64]]) -> Self {
        NbtListItem::LongArray(Cow::Owned(value.iter().map(|v| ByteCow::new_ref(*v)).collect()))
    }
}
