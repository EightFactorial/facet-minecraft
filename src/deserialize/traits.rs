use crate::McDeserializer;

/// A deserializer for Minecraft protocol data.
pub trait Deserializer {}

/// An extension trait for [`Deserializer`] that provides
/// variable-length deserialization methods.
pub trait DeserializerExt: Deserializer {}

// -------------------------------------------------------------------------------------------------

impl Deserializer for McDeserializer {}

impl DeserializerExt for McDeserializer {}
