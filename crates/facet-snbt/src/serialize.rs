//! TODO

use core::convert::Infallible;

use facet_nbt::prelude::*;
use facet_serialize::Serializer;

use crate::{
    SerializeError,
    format::{Legacy, Modern, SnbtFormat},
    snbt::Snbt,
};

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
    Ok(Snbt::new_unchecked(serializer.into_inner()))
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
    Ok(Snbt::new_unchecked(serializer.into_inner()))
}

// -------------------------------------------------------------------------------------------------

use sealed::SnbtSerializer;
mod sealed {
    use itoa::Buffer as IntBuffer;
    use ryu::Buffer as FltBuffer;

    use super::{SerState, SnbtFormat, SnbtWriter};

    pub struct SnbtSerializer<'input, F: SnbtFormat<'input>>
    where F::Inner: SnbtWriter
    {
        buffer: F::Inner,
        int_buffer: IntBuffer,
        flt_buffer: FltBuffer,

        state: SerState,
    }

    #[expect(dead_code)]
    impl<'input, F: SnbtFormat<'input>> SnbtSerializer<'input, F>
    where F::Inner: SnbtWriter
    {
        #[must_use]
        pub(super) fn new(buffer: F::Inner) -> Self {
            Self {
                buffer,
                int_buffer: IntBuffer::new(),
                flt_buffer: FltBuffer::new(),
                state: SerState::Start,
            }
        }

        #[must_use]
        pub(super) const fn state(&self) -> SerState { self.state }

        #[must_use]
        pub(super) const fn state_mut(&mut self) -> &mut SerState { &mut self.state }

        #[must_use]
        pub(super) fn into_inner(self) -> F::Inner { self.buffer }

        pub(super) fn write_int<Integer: itoa::Integer>(
            &mut self,
            integer: Integer,
        ) -> Result<(), <F::Inner as SnbtWriter>::WriteError> {
            let str = self.int_buffer.format(integer);
            self.buffer.write_str(str)
        }

        pub(super) fn write_float<Float: ryu::Float>(
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SerState {
    Start,
    Compound(bool, bool, bool, usize), // (is_key, array_prefix, separator, depth)
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
        if let SerState::Compound(_, array, separator, _) = self.state_mut() {
            if *array {
                *array = false;
                self.write_str("B;").unwrap();
            } else if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('B').unwrap();
        Ok(())
    }

    fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        if let SerState::Compound(_, array, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('S').unwrap();
        Ok(())
    }

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        if let SerState::Compound(_, array, separator, _) = self.state_mut() {
            if *array {
                *array = false;
                self.write_str("I;").unwrap();
            } else if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('I').unwrap();
        Ok(())
    }

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        if let SerState::Compound(_, array, separator, _) = self.state_mut() {
            if *array {
                *array = false;
                self.write_str("L;").unwrap();
            } else if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('L').unwrap();
        Ok(())
    }

    fn serialize_i128(&mut self, value: i128) -> Result<(), Self::Error> {
        unreachable!("`i128` is not a valid NBT type")
    }

    fn serialize_isize(&mut self, value: isize) -> Result<(), Self::Error> {
        if let SerState::Compound(_, array, separator, _) = self.state_mut() {
            if *array {
                *array = false;
                self.write_str("I;").unwrap();
            } else if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('I').unwrap();
        Ok(())
    }

    // Note: `Legacy` does not support scientific notation for floats
    // which `ryu` uses. `ToString`, however, never scientific notation.
    fn serialize_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        let string = alloc::string::ToString::to_string(&value);
        self.write_str(&string).unwrap();
        Ok(())
    }

    // Note: `Legacy` does not support scientific notation for floats
    // which `ryu` uses. `ToString`, however, never scientific notation.
    fn serialize_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        let string = alloc::string::ToString::to_string(&value);
        self.write_str(&string).unwrap();
        Ok(())
    }

    fn serialize_char(&mut self, value: char) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

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

        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        if VALID_CHARS
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
        match self.state_mut() {
            SerState::Start => Ok(()),
            SerState::Compound(name, _, separator, _) if *name => {
                self.serialize_str(&simd_cesu8::decode_lossy(value))
            }
            SerState::Compound(..) => value.iter().try_for_each(|v| self.serialize_u8(*v)),
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
        if let SerState::Compound(str, list, ..) = self.state_mut() {
            match name {
                "ByteArray" | "IntArray" | "LongArray" => *list = true,
                "String" => *str = true,
                _ => {}
            }
        }
        Ok(())
    }

    fn end_field(&mut self) -> Result<(), Self::Error> { todo!() }

    fn start_object(&mut self, len: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn end_object(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn start_array(&mut self, len: Option<usize>) -> Result<(), Self::Error> {
        match self.state_mut() {
            SerState::Start => Ok(()),
            SerState::Compound(_, _, separator, _) => {
                *separator = false;
                self.write_char('[').unwrap();
                Ok(())
            }
        }
    }

    fn end_array(&mut self) -> Result<(), Self::Error> {
        match self.state_mut() {
            SerState::Start => Ok(()),
            SerState::Compound(_, array, ..) => {
                *array = false;
                self.write_char(']').unwrap();
                Ok(())
            }
        }
    }

    fn start_map(&mut self, len: Option<usize>) -> Result<(), Self::Error> {
        match self.state_mut() {
            SerState::Start => *self.state_mut() = SerState::Compound(false, false, false, 0),
            SerState::Compound(_, _, separator, n) => {
                let write = *separator;
                *separator = false;
                *n += 1;

                if write {
                    self.write_char(',').unwrap();
                }
            }
        }

        self.write_char('{').unwrap();
        Ok(())
    }

    fn end_map(&mut self) -> Result<(), Self::Error> {
        match self.state_mut() {
            SerState::Start => todo!(),
            SerState::Compound(.., 0) => *self.state_mut() = SerState::Start,
            SerState::Compound(.., str, depth) => {
                *str = true;
                *depth -= 1;
            }
        }

        self.write_char('}').unwrap();
        Ok(())
    }

    fn begin_map_key(&mut self) -> Result<(), Self::Error> {
        if let SerState::Compound(key, ..) = self.state_mut() {
            *key = true;
        }

        Ok(())
    }

    fn end_map_key(&mut self) -> Result<(), Self::Error> {
        if let SerState::Compound(key, _, separator, _) = self.state_mut() {
            *key = false;
            *separator = false;
        }

        self.write_char(':').unwrap();
        Ok(())
    }

    fn begin_map_value(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn end_map_value(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

// -------------------------------------------------------------------------------------------------

#[expect(unused_variables)]
impl Serializer for SnbtSerializer<'_, Modern> {
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
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('B').unwrap();
        Ok(())
    }

    fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('S').unwrap();
        Ok(())
    }

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('I').unwrap();
        Ok(())
    }

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('L').unwrap();
        Ok(())
    }

    fn serialize_i128(&mut self, value: i128) -> Result<(), Self::Error> {
        unreachable!("`i128` is not a valid NBT type")
    }

    fn serialize_isize(&mut self, value: isize) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_int(value).unwrap();
        self.write_char('I').unwrap();
        Ok(())
    }

    fn serialize_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_float(value).unwrap();
        self.write_char('F').unwrap();
        Ok(())
    }

    fn serialize_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

        self.write_float(value).unwrap();
        self.write_char('D').unwrap();
        Ok(())
    }

    fn serialize_char(&mut self, value: char) -> Result<(), Self::Error> {
        if let SerState::Compound(_, _, separator, _) = self.state_mut() {
            if *separator {
                self.write_char(',').unwrap();
            } else {
                *separator = true;
            }
        }

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
