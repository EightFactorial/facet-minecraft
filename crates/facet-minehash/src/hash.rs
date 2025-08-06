//! TODO

use core::{convert::Infallible, hash::Hasher};

use crc32c::Crc32cHasher;
use facet_reflect::Peek;
use facet_serialize::Serializer;

use crate::assert::AssertHashable;

/// TODO
pub fn hash<'facet, T: AssertHashable<'facet>>(input: &T) -> u32 {
    let () = const { <T as AssertHashable<'facet>>::ASSERT };

    let mut serializer = MineHasher(Crc32cHasher::new(0));
    match facet_serialize::serialize_iterative(Peek::new(input), &mut serializer) {
        #[expect(clippy::cast_possible_truncation, reason = "Crc32cHasher internally uses u32")]
        Ok(()) => serializer.0.finish() as u32,
        Err(..) => unreachable!("Error is `Infallible` and can never happen"),
    }
}

// -------------------------------------------------------------------------------------------------

struct MineHasher(Crc32cHasher);

impl core::ops::Deref for MineHasher {
    type Target = Crc32cHasher;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl core::ops::DerefMut for MineHasher {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

impl Serializer for MineHasher {
    type Error = Infallible;

    fn serialize_bool(&mut self, _value: bool) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_u8(&mut self, _value: u8) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_u16(&mut self, _value: u16) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_u32(&mut self, _value: u32) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_u64(&mut self, _value: u64) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_u128(&mut self, _value: u128) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_usize(&mut self, _value: usize) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_i8(&mut self, _value: i8) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_i16(&mut self, _value: i16) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_i32(&mut self, _value: i32) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_i64(&mut self, _value: i64) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_i128(&mut self, _value: i128) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_isize(&mut self, _value: isize) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_f32(&mut self, _value: f32) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_f64(&mut self, _value: f64) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_char(&mut self, _value: char) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_str(&mut self, _value: &str) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_bytes(&mut self, _value: &[u8]) -> Result<(), Self::Error> { Ok(()) }

    fn start_some(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_none(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_unit(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn start_enum_variant(&mut self, _disc: u64) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_unit_variant(&mut self, _idx: usize, _name: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn serialize_field_name(&mut self, _name: &'static str) -> Result<(), Self::Error> { Ok(()) }

    fn end_field(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn start_object(&mut self, _len: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn end_object(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn start_array(&mut self, _len: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn end_array(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn start_map(&mut self, _len: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn end_map(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn begin_map_key(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn end_map_key(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn begin_map_value(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn end_map_value(&mut self) -> Result<(), Self::Error> { Ok(()) }
}
