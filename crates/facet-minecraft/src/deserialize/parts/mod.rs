#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub(super) use json::deserialize_json;

mod map;
pub(super) use map::{deserialize_map, deserialize_set};

mod option;
pub(super) use option::deserialize_option;

mod pointer;
#[expect(unused_imports)]
pub(super) use pointer::{deserialize_pointer, deserialize_smartpointer};

mod primitive;
pub(super) use primitive::deserialize_primitive;

mod sequence;
pub(super) use sequence::deserialize_sequence;

mod user;
pub(super) use user::deserialize_user;
