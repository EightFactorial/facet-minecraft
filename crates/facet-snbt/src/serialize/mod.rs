//! TODO

use facet_nbt::prelude::*;
use facet_serialize::Serializer;

use crate::{
    format::{Modern, SnbtFormat},
    snbt::Snbt,
};

mod error;
pub use error::SerializeError;

mod serializer;
use serializer::SnbtSerializer;

mod writer;
use writer::SnbtWriter;

/// Serialize an [`Nbt`] into [`Snbt`] with the given [`SnbtFormat`].
///
/// # Errors
/// Returns an error if the serialization fails.
pub fn serialize<'input, F: SnbtFormat<'input>>(
    nbt: &'input Nbt,
    buffer: F::Inner,
) -> Result<Snbt<'input, F>, <SnbtSerializer<'input, F> as Serializer>::Error>
where
    F::Inner: SnbtWriter,
    SnbtSerializer<'input, F>: Serializer,
{
    let mut serializer = SnbtSerializer::<F>::new(buffer);
    facet_serialize::serialize_iterative(facet_reflect::Peek::new(nbt), &mut serializer)?;
    Ok(Snbt::new_unchecked(serializer.buffer))
}

/// Serialize a [`BorrowedNbt`] into [`Snbt`] with the given [`SnbtFormat`].
///
/// # Errors
/// Returns an error if the serialization fails.
pub fn serialize_borrowed<'input, F: SnbtFormat<'input>>(
    nbt: &BorrowedNbt<'input>,
    buffer: F::Inner,
) -> Result<Snbt<'input, F>, <SnbtSerializer<'input, F> as Serializer>::Error>
where
    F::Inner: SnbtWriter,
    SnbtSerializer<'input, F>: Serializer,
{
    let mut serializer = SnbtSerializer::<F>::new(buffer);
    facet_serialize::serialize_iterative(facet_reflect::Peek::new(nbt), &mut serializer)?;
    Ok(Snbt::new_unchecked(serializer.buffer))
}

// -------------------------------------------------------------------------------------------------

#[expect(unused_variables)]
impl Serializer for SnbtSerializer<'_, Modern> {
    type Error = SerializeError;

    #[expect(clippy::match_bool)]
    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        match value {
            true => self.write_str("true").unwrap(),
            false => self.write_str("false").unwrap(),
        }
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    fn serialize_u8(&mut self, value: u8) -> Result<(), Self::Error> {
        self.serialize_i8(value as i8)
    }

    #[expect(clippy::cast_possible_wrap)]
    fn serialize_u16(&mut self, value: u16) -> Result<(), Self::Error> {
        self.serialize_i16(value as i16)
    }

    #[expect(clippy::cast_possible_wrap)]
    fn serialize_u32(&mut self, value: u32) -> Result<(), Self::Error> {
        self.serialize_i32(value as i32)
    }

    #[expect(clippy::cast_possible_wrap)]
    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        self.serialize_i64(value as i64)
    }

    fn serialize_u128(&mut self, value: u128) -> Result<(), Self::Error> {
        unreachable!("`u128` is not a valid NBT type")
    }

    #[expect(clippy::cast_possible_wrap)]
    fn serialize_usize(&mut self, value: usize) -> Result<(), Self::Error> {
        self.serialize_isize(value as isize)
    }

    fn serialize_i8(&mut self, value: i8) -> Result<(), Self::Error> {
        self.write_array_prefix('B').unwrap();
        self.write_separator().unwrap();

        self.write_int(value).unwrap();
        self.write_char('B').unwrap();
        Ok(())
    }

    fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        self.write_separator().unwrap();

        self.write_int(value).unwrap();
        self.write_char('S').unwrap();
        Ok(())
    }

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        self.write_array_prefix('I').unwrap();
        self.write_separator().unwrap();

        self.write_int(value).unwrap();
        self.write_char('I').unwrap();
        Ok(())
    }

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        self.write_array_prefix('L').unwrap();
        self.write_separator().unwrap();

        self.write_int(value).unwrap();
        self.write_char('L').unwrap();
        Ok(())
    }

    fn serialize_i128(&mut self, value: i128) -> Result<(), Self::Error> {
        unreachable!("`i128` is not a valid NBT type")
    }

    fn serialize_isize(&mut self, value: isize) -> Result<(), Self::Error> {
        self.write_array_prefix('I').unwrap();
        self.write_separator().unwrap();

        self.write_int(value).unwrap();
        self.write_char('I').unwrap();
        Ok(())
    }

    // Note: `Legacy` does not support scientific notation for floats
    // which `ryu` uses. `ToString`, however, never scientific notation.
    fn serialize_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        self.write_array_prefix('F').unwrap();
        self.write_separator().unwrap();

        let string = alloc::string::ToString::to_string(&value);
        self.write_str(&string).unwrap();
        Ok(())
    }

    // Note: `Legacy` does not support scientific notation for floats
    // which `ryu` uses. `ToString`, however, never scientific notation.
    fn serialize_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        self.write_array_prefix('D').unwrap();
        self.write_separator().unwrap();

        let string = alloc::string::ToString::to_string(&value);
        self.write_str(&string).unwrap();
        Ok(())
    }

    fn serialize_char(&mut self, value: char) -> Result<(), Self::Error> {
        self.write_separator().unwrap();

        self.write_char(value).unwrap();
        Ok(())
    }

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
