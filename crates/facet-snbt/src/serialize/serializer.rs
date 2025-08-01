#![allow(clippy::struct_excessive_bools)]

use facet_serialize::Serializer;
use itoa::Buffer as IntBuffer;
use ryu::Buffer as FltBuffer;

use super::{SerializeError, SnbtWriter};
use crate::format::{Legacy, SnbtFormat};

pub struct SnbtSerializer<'input, F: SnbtFormat<'input>>
where F::Inner: SnbtWriter
{
    pub buffer: F::Inner,
    int_buffer: IntBuffer,
    flt_buffer: FltBuffer,

    array_prefix: bool,
    bytes_as_string: bool,
    depth: usize,
    is_key: bool,
    list_depth: usize,
    separator: bool,
}

impl<'input, F: SnbtFormat<'input>> SnbtSerializer<'input, F>
where F::Inner: SnbtWriter
{
    #[must_use]
    pub fn new(buffer: F::Inner) -> Self {
        Self {
            buffer,
            int_buffer: IntBuffer::new(),
            flt_buffer: FltBuffer::new(),

            array_prefix: false,
            bytes_as_string: false,
            depth: 0,
            is_key: false,
            list_depth: 0,
            separator: false,
        }
    }

    pub const fn start_list(&mut self) { self.list_depth |= 1 << (self.depth.saturating_sub(1)); }

    pub const fn end_list(&mut self) { self.list_depth &= !(1 << (self.depth.saturating_sub(1))); }

    #[must_use]
    pub const fn is_list(&self) -> bool {
        self.list_depth & (1 << (self.depth.saturating_sub(1))) != 0
    }

    pub fn write_separator(&mut self) -> Result<(), <F::Inner as SnbtWriter>::WriteError> {
        if self.separator {
            self.write_char(',')
        } else {
            self.separator = true;
            Ok(())
        }
    }

    pub fn write_array_prefix(
        &mut self,
        prefix: char,
    ) -> Result<(), <F::Inner as SnbtWriter>::WriteError> {
        if self.array_prefix {
            self.array_prefix = false;
            self.write_char(prefix)?;
            self.write_char(';')
        } else {
            Ok(())
        }
    }

    pub fn write_int<Integer: itoa::Integer>(
        &mut self,
        integer: Integer,
    ) -> Result<(), <F::Inner as SnbtWriter>::WriteError> {
        let str = self.int_buffer.format(integer);
        self.buffer.write_str(str)
    }

    pub fn write_float<Float: ryu::Float>(
        &mut self,
        float: Float,
    ) -> Result<(), <F::Inner as SnbtWriter>::WriteError> {
        let str = self.flt_buffer.format(float);
        self.buffer.write_str(str)
    }
}

impl<'input, F: SnbtFormat<'input>> core::ops::Deref for SnbtSerializer<'input, F>
where F::Inner: SnbtWriter
{
    type Target = F::Inner;

    fn deref(&self) -> &Self::Target { &self.buffer }
}
impl<'input, F: SnbtFormat<'input>> core::ops::DerefMut for SnbtSerializer<'input, F>
where F::Inner: SnbtWriter
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.buffer }
}

// -------------------------------------------------------------------------------------------------

#[expect(unused_variables)]
impl Serializer for SnbtSerializer<'_, Legacy> {
    type Error = SerializeError;

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        self.serialize_i8(i8::from(value))
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

    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> {
        #[cfg(feature = "std")]
        static VALID_CHARS: std::sync::OnceLock<regex_lite::Regex> = std::sync::OnceLock::new();
        #[cfg(all(not(feature = "std"), feature = "once_cell"))]
        static VALID_CHARS: once_cell::sync::OnceCell<regex_lite::Regex> =
            once_cell::sync::OnceCell::new();
        #[cfg(not(any(feature = "std", feature = "once_cell")))]
        compile_error!("It is required to enable the `once_cell` feature on `no_std` platforms!");

        self.write_separator().unwrap();

        if self.is_key
            && VALID_CHARS
                .get_or_init(|| regex_lite::Regex::new(r"^[a-zA-Z0-9_\-\.\+]+$").unwrap())
                .is_match(value)
        {
            self.write_str(value).unwrap();
        } else {
            self.write_char('"').unwrap();
            self.write_str(&value.replace('"', "\\\"")).unwrap();
            self.write_char('"').unwrap();
        }

        Ok(())
    }

    fn serialize_bytes(&mut self, value: &[u8]) -> Result<(), Self::Error> {
        if self.depth == 0 {
            Ok(())
        } else if self.bytes_as_string {
            self.serialize_str(&simd_cesu8::decode_lossy(value))
        } else {
            value.iter().try_for_each(|&v| self.serialize_u8(v))
        }
    }

    fn start_some(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_none(&mut self) -> Result<(), Self::Error> { todo!() }

    fn serialize_unit(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn start_enum_variant(&mut self, disc: u64) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_unit_variant(&mut self, idx: usize, name: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_field_name(&mut self, name: &'static str) -> Result<(), Self::Error> {
        if self.depth > 0 {
            std::println!("Field \"{}\" @ {}", name, self.depth);
            match name {
                "ByteArray" | "IntArray" | "LongArray" => {
                    self.start_list();
                    self.array_prefix = true;
                }
                "List" => self.start_list(),
                "String" => self.bytes_as_string = true,
                _ => {}
            }
        }

        Ok(())
    }

    fn end_field(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_object(&mut self, len: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn end_object(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn start_array(&mut self, len: Option<usize>) -> Result<(), Self::Error> {
        if !self.is_key && !self.bytes_as_string && self.is_list() {
            std::println!("Start Array @ {}", self.depth);
            self.write_char('[').unwrap();
            self.separator = false;
        }

        Ok(())
    }

    fn end_array(&mut self) -> Result<(), Self::Error> {
        if !self.is_key && !self.bytes_as_string && self.is_list() {
            std::println!("End Array @ {}", self.depth);
            self.write_char(']').unwrap();
            self.array_prefix = false;
            self.end_list();
        }

        Ok(())
    }

    fn start_map(&mut self, len: Option<usize>) -> Result<(), Self::Error> {
        self.write_char('{').unwrap();
        self.separator = false;
        self.depth += 1;

        Ok(())
    }

    fn end_map(&mut self) -> Result<(), Self::Error> {
        self.write_char('}').unwrap();
        self.depth -= 1;

        Ok(())
    }

    fn begin_map_key(&mut self) -> Result<(), Self::Error> {
        self.is_key = true;
        self.bytes_as_string = true;

        Ok(())
    }

    fn end_map_key(&mut self) -> Result<(), Self::Error> {
        self.write_char(':').unwrap();
        self.is_key = false;
        self.bytes_as_string = false;
        self.separator = false;

        Ok(())
    }

    fn begin_map_value(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn end_map_value(&mut self) -> Result<(), Self::Error> { Ok(()) }
}
