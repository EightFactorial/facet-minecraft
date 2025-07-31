//! TODO

use core::convert::Infallible;

use facet_nbt::prelude::*;
use facet_serialize::Serializer;

use crate::{
    SerializeError,
    format::{Legacy, Modern, SnbtFormat},
    snbt::Snbt,
};

/// Serialize a [`BorrowedNbt`] into [`Snbt`] with the given [`SnbtFormat`].
///
/// # Errors
/// Returns an error if the serialization fails.
#[expect(private_bounds, private_interfaces)]
pub fn serialize<'input, F: SnbtFormat<'input>>(
    nbt: &BorrowedNbt<'input>,
    buffer: F::Inner,
) -> Result<Snbt<'input, F>, <SnbtSerializer<'input, F> as Serializer>::Error>
where
    F::Inner: SnbtWriter,
    SnbtSerializer<'input, F>: Serializer,
{
    let mut serializer = SnbtSerializer::<F>(buffer, SerState::Start);
    facet_serialize::serialize_iterative(facet_reflect::Peek::new(nbt), &mut serializer)?;
    Ok(Snbt::new_unchecked(serializer.0))
}

struct SnbtSerializer<'input, F: SnbtFormat<'input>>(F::Inner, SerState)
where F::Inner: SnbtWriter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SerState {
    Start,
}

// -------------------------------------------------------------------------------------------------

#[expect(unused_variables)]
impl Serializer for SnbtSerializer<'_, Legacy> {
    type Error = SerializeError;

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> { todo!() }

    fn serialize_u8(&mut self, value: u8) -> Result<(), Self::Error> { todo!() }

    fn serialize_u16(&mut self, value: u16) -> Result<(), Self::Error> { todo!() }

    fn serialize_u32(&mut self, value: u32) -> Result<(), Self::Error> { todo!() }

    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error> { todo!() }

    fn serialize_u128(&mut self, value: u128) -> Result<(), Self::Error> { todo!() }

    fn serialize_usize(&mut self, value: usize) -> Result<(), Self::Error> { todo!() }

    fn serialize_i8(&mut self, value: i8) -> Result<(), Self::Error> { todo!() }

    fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error> { todo!() }

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> { todo!() }

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> { todo!() }

    fn serialize_i128(&mut self, value: i128) -> Result<(), Self::Error> { todo!() }

    fn serialize_isize(&mut self, value: isize) -> Result<(), Self::Error> { todo!() }

    fn serialize_f32(&mut self, value: f32) -> Result<(), Self::Error> { todo!() }

    fn serialize_f64(&mut self, value: f64) -> Result<(), Self::Error> { todo!() }

    fn serialize_char(&mut self, value: char) -> Result<(), Self::Error> { todo!() }

    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> { todo!() }

    fn serialize_bytes(&mut self, value: &[u8]) -> Result<(), Self::Error> { todo!() }

    fn start_some(&mut self) -> Result<(), Self::Error> { todo!() }

    fn serialize_none(&mut self) -> Result<(), Self::Error> { todo!() }

    fn serialize_unit(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_enum_variant(&mut self, disc: u64) -> Result<(), Self::Error> { todo!() }

    fn serialize_unit_variant(&mut self, idx: usize, name: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_field_name(&mut self, name: &'static str) -> Result<(), Self::Error> { todo!() }

    fn end_field(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_object(&mut self, len: Option<usize>) -> Result<(), Self::Error> { todo!() }

    fn end_object(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_array(&mut self, len: Option<usize>) -> Result<(), Self::Error> { todo!() }

    fn end_array(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_map(&mut self, len: Option<usize>) -> Result<(), Self::Error> { todo!() }

    fn end_map(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_map_key(&mut self) -> Result<(), Self::Error> { todo!() }

    fn end_map_key(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_map_value(&mut self) -> Result<(), Self::Error> { todo!() }

    fn end_map_value(&mut self) -> Result<(), Self::Error> { todo!() }
}

// -------------------------------------------------------------------------------------------------

#[expect(unused_variables)]
impl Serializer for SnbtSerializer<'_, Modern> {
    type Error = SerializeError;

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> { todo!() }

    fn serialize_u8(&mut self, value: u8) -> Result<(), Self::Error> { todo!() }

    fn serialize_u16(&mut self, value: u16) -> Result<(), Self::Error> { todo!() }

    fn serialize_u32(&mut self, value: u32) -> Result<(), Self::Error> { todo!() }

    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error> { todo!() }

    fn serialize_u128(&mut self, value: u128) -> Result<(), Self::Error> { todo!() }

    fn serialize_usize(&mut self, value: usize) -> Result<(), Self::Error> { todo!() }

    fn serialize_i8(&mut self, value: i8) -> Result<(), Self::Error> { todo!() }

    fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error> { todo!() }

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> { todo!() }

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> { todo!() }

    fn serialize_i128(&mut self, value: i128) -> Result<(), Self::Error> { todo!() }

    fn serialize_isize(&mut self, value: isize) -> Result<(), Self::Error> { todo!() }

    fn serialize_f32(&mut self, value: f32) -> Result<(), Self::Error> { todo!() }

    fn serialize_f64(&mut self, value: f64) -> Result<(), Self::Error> { todo!() }

    fn serialize_char(&mut self, value: char) -> Result<(), Self::Error> { todo!() }

    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> { todo!() }

    fn serialize_bytes(&mut self, value: &[u8]) -> Result<(), Self::Error> { todo!() }

    fn start_some(&mut self) -> Result<(), Self::Error> { todo!() }

    fn serialize_none(&mut self) -> Result<(), Self::Error> { todo!() }

    fn serialize_unit(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_enum_variant(&mut self, disc: u64) -> Result<(), Self::Error> { todo!() }

    fn serialize_unit_variant(&mut self, idx: usize, name: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_field_name(&mut self, name: &'static str) -> Result<(), Self::Error> { todo!() }

    fn end_field(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_object(&mut self, len: Option<usize>) -> Result<(), Self::Error> { todo!() }

    fn end_object(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_array(&mut self, len: Option<usize>) -> Result<(), Self::Error> { todo!() }

    fn end_array(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_map(&mut self, len: Option<usize>) -> Result<(), Self::Error> { todo!() }

    fn end_map(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_map_key(&mut self) -> Result<(), Self::Error> { todo!() }

    fn end_map_key(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_map_value(&mut self) -> Result<(), Self::Error> { todo!() }

    fn end_map_value(&mut self) -> Result<(), Self::Error> { todo!() }
}

// -------------------------------------------------------------------------------------------------

#[expect(missing_docs, clippy::missing_errors_doc)]
pub trait SnbtWriter {
    type WriteError;

    fn write_str(&mut self, value: &str) -> Result<(), Self::WriteError>;
    fn write_char(&mut self, value: char) -> Result<(), Self::WriteError>;
    fn reserve(&mut self, additional: usize) -> Result<(), Self::WriteError>;
}

#[expect(clippy::unit_arg)]
impl SnbtWriter for alloc::string::String {
    type WriteError = Infallible;

    fn write_str(&mut self, value: &str) -> Result<(), Self::WriteError> {
        Ok(self.push_str(value))
    }

    fn write_char(&mut self, value: char) -> Result<(), Self::WriteError> { Ok(self.push(value)) }

    fn reserve(&mut self, value: usize) -> Result<(), Self::WriteError> { Ok(self.reserve(value)) }
}

impl SnbtWriter for alloc::borrow::Cow<'_, str> {
    type WriteError = Infallible;

    fn write_str(&mut self, value: &str) -> Result<(), Self::WriteError> {
        SnbtWriter::write_str(self.to_mut(), value)
    }

    fn write_char(&mut self, value: char) -> Result<(), Self::WriteError> {
        SnbtWriter::write_char(self.to_mut(), value)
    }

    fn reserve(&mut self, value: usize) -> Result<(), Self::WriteError> {
        SnbtWriter::reserve(self.to_mut(), value)
    }
}
