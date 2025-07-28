use indexmap::IndexMap;

use super::NbtTag;
use crate::mutf8::Mutf8String;

pub struct NbtCompound(IndexMap<Mutf8String, NbtTag>);
