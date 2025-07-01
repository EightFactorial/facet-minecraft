#![allow(clippy::inline_always, clippy::missing_errors_doc)]

use facet::Facet;

use crate::{DeserializeError, McDeserializer, deserialize::error::ErrorReason};

/// A deserializer for Minecraft protocol data.
pub trait Deserializer {
    /// Deserialize a unit value `()`.
    fn deserialize_unit<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<((), &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize an unsigned 8-bit integer.
    fn deserialize_u8<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u8, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize an unsigned 16-bit integer.
    fn deserialize_u16<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize an unsigned 32-bit integer.
    fn deserialize_u32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize an unsigned 64-bit integer.
    fn deserialize_u64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize an unsigned 128-bit integer.
    fn deserialize_u128<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a `usize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_usize<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(usize, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_u64(input).map(|(val, rem)| (val as usize, rem))
    }

    /// Deserialize a signed 8-bit integer.
    fn deserialize_i8<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i8, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a signed 16-bit integer.
    fn deserialize_i16<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a signed 32-bit integer.
    fn deserialize_i32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a signed 64-bit integer.
    fn deserialize_i64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a signed 128-bit integer.
    fn deserialize_i128<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize an `isize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_isize<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(isize, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_i64(input).map(|(val, rem)| (val as isize, rem))
    }

    /// Deserialize a single-precision floating-point value.
    fn deserialize_f32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f32, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a double-precision floating-point value.
    fn deserialize_f64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f64, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a boolean value.
    #[inline(always)]
    fn deserialize_bool<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(bool, &'input [u8]), DeserializeError<'input, 'shape>> {
        match input.split_first() {
            Some((0u8, remainder)) => Ok((false, remainder)),
            Some((1u8, remainder)) => Ok((true, remainder)),
            Some((&b, ..)) => {
                Err(DeserializeError::new(input, bool::SHAPE, ErrorReason::InvalidBool(b)))
            }
            None => Err(DeserializeError::new(input, bool::SHAPE, ErrorReason::EndOfInput)),
        }
    }

    /// Deserialize a UTF-8 string slice.
    fn deserialize_str<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(&'input str, &'input [u8]), DeserializeError<'input, 'shape>>;
}

/// An extension trait for [`Deserializer`] that provides
/// variable-length deserialization methods.
pub trait DeserializerExt: Deserializer {
    /// Deserialize a variable-length unsigned 16-bit integer.
    fn deserialize_var_u16<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'shape>>;
    /// Deserialize a variable-length unsigned 32-bit integer.
    fn deserialize_var_u32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'shape>>;
    /// Deserialize a variable-length unsigned 64-bit integer.
    fn deserialize_var_u64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'shape>>;
    /// Deserialize a variable-length unsigned 128-bit integer.
    fn deserialize_var_u128<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'shape>>;

    /// Deserialize a variable-length `usize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_var_usize<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(usize, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_var_u64(input).map(|(val, rem)| (val as usize, rem))
    }

    /// Deserialize a variable-length signed 16-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i16<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_var_u16(input).map(|(val, rem)| (val as i16, rem))
    }
    /// Deserialize a variable-length signed 32-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_var_u32(input).map(|(val, rem)| (val as i32, rem))
    }
    /// Deserialize a variable-length signed 64-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_var_u64(input).map(|(val, rem)| (val as i64, rem))
    }
    /// Deserialize a variable-length signed 128-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i128<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_var_u128(input).map(|(val, rem)| (val as i128, rem))
    }

    /// Deserialize a variable-length signed `isize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_var_isize<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(isize, &'input [u8]), DeserializeError<'input, 'shape>> {
        self.deserialize_var_i64(input).map(|(val, rem)| (val as isize, rem))
    }
}

// -------------------------------------------------------------------------------------------------

impl Deserializer for McDeserializer {
    fn deserialize_unit<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<((), &'input [u8]), DeserializeError<'input, 'shape>> {
        Ok(((), input))
    }

    fn deserialize_u8<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u8, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&byte, remainder)) = input.split_first() {
            Ok((byte, remainder))
        } else {
            Err(DeserializeError::new(input, u8::SHAPE, ErrorReason::EndOfInput).with_length(1))
        }
    }

    fn deserialize_u16<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<2>() {
            Ok((u16::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, u16::SHAPE, ErrorReason::EndOfInput).with_length(2))
        }
    }

    fn deserialize_u32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<4>() {
            Ok((u32::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, u32::SHAPE, ErrorReason::EndOfInput).with_length(4))
        }
    }

    fn deserialize_u64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<8>() {
            Ok((u64::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, u64::SHAPE, ErrorReason::EndOfInput).with_length(8))
        }
    }

    fn deserialize_u128<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<16>() {
            Ok((u128::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, u128::SHAPE, ErrorReason::EndOfInput).with_length(16))
        }
    }

    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_i8<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i8, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&byte, remainder)) = input.split_first() {
            Ok((byte as i8, remainder))
        } else {
            Err(DeserializeError::new(input, i8::SHAPE, ErrorReason::EndOfInput).with_length(1))
        }
    }

    fn deserialize_i16<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<2>() {
            Ok((i16::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, i16::SHAPE, ErrorReason::EndOfInput).with_length(2))
        }
    }

    fn deserialize_i32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<4>() {
            Ok((i32::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, i32::SHAPE, ErrorReason::EndOfInput).with_length(4))
        }
    }

    fn deserialize_i64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<8>() {
            Ok((i64::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, i64::SHAPE, ErrorReason::EndOfInput).with_length(8))
        }
    }

    fn deserialize_i128<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<16>() {
            Ok((i128::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, i128::SHAPE, ErrorReason::EndOfInput).with_length(16))
        }
    }

    fn deserialize_f32<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f32, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<4>() {
            Ok((f32::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, f32::SHAPE, ErrorReason::EndOfInput).with_length(4))
        }
    }

    fn deserialize_f64<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f64, &'input [u8]), DeserializeError<'input, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<8>() {
            Ok((f64::from_le_bytes(chunk), remainder))
        } else {
            Err(DeserializeError::new(input, f64::SHAPE, ErrorReason::EndOfInput).with_length(8))
        }
    }

    fn deserialize_str<'input, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(&'input str, &'input [u8]), DeserializeError<'input, 'shape>> {
        let (len, remainder) = self.deserialize_var_usize(input)?;
        if let Some((str_bytes, rem_bytes)) = remainder.split_at_checked(len) {
            match core::str::from_utf8(str_bytes) {
                Ok(s) => Ok((s, rem_bytes)),
                Err(err) => Err(DeserializeError::new(
                    remainder,
                    str::SHAPE,
                    ErrorReason::InvalidUtf8(err.valid_up_to()),
                )
                .with_length(len - err.valid_up_to())),
            }
        } else {
            Err(DeserializeError::new(remainder, str::SHAPE, ErrorReason::EndOfInput)
                .with_length(len))
        }
    }
}

impl DeserializerExt for McDeserializer {
    fn deserialize_var_u16<'input, 'shape>(
        &mut self,
        original: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'shape>> {
        let mut input = original;
        let mut number: u16 = 0;

        for i in 0..3 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u16::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserializeError::new(original, u16::SHAPE, ErrorReason::EndOfInput)
                    .with_length(i + 1))?;
            }
        }
        Ok((number, input))
    }

    fn deserialize_var_u32<'input, 'shape>(
        &mut self,
        original: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'shape>> {
        let mut input = original;
        let mut number: u32 = 0;

        for i in 0..5 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u32::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserializeError::new(original, u32::SHAPE, ErrorReason::EndOfInput)
                    .with_length(i + 1))?;
            }
        }
        Ok((number, input))
    }

    fn deserialize_var_u64<'input, 'shape>(
        &mut self,
        original: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'shape>> {
        let mut input = original;
        let mut number: u64 = 0;

        for i in 0..10 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u64::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserializeError::new(original, u64::SHAPE, ErrorReason::EndOfInput)
                    .with_length(i + 1))?;
            }
        }
        Ok((number, input))
    }

    fn deserialize_var_u128<'input, 'shape>(
        &mut self,
        original: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'shape>> {
        let mut input = original;
        let mut number: u128 = 0;

        for i in 0..19 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u128::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserializeError::new(original, u128::SHAPE, ErrorReason::EndOfInput)
                    .with_length(i + 1))?;
            }
        }
        Ok((number, input))
    }
}
