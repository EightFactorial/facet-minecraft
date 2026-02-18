//! Custom [`facet`](::facet) attributes for supporting the Minecraft protocol.
//!
//! TODO: Make `Serialize` and `Deserialize` *require* a function pointer.
//!
//! Until then, this can be forced via trait bounds and compiler errors.
#![allow(unpredictable_function_pointer_comparisons, reason = "Correct!")]

use crate::{deserialize::fns::DeserializeFn, serialize::fns::SerializeFn};

facet::define_attr_grammar! {
    ns "mc";
    crate_path ::facet_minecraft::attribute;

    /// Attributes used by the Minecraft protocol.
    pub enum Attr {
        /// Marks a field as variably-sized.
        Variable,
        /// Specifies custom serialization function for a field.
        Serialize(fn_ptr SerializeFn),
        /// Specifies custom deserialization function for a field.
        Deserialize(fn_ptr DeserializeFn),
    }
}
