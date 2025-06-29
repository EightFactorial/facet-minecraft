#![allow(clippy::inline_always, clippy::missing_errors_doc)]

use crate::{DeserializeError, McDeserializer};

/// A deserializer for Minecraft protocol data.
pub trait Deserializer {
    /// Deserialize a unit value `()`.
    fn deserialize_unit<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<((), &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize an unsigned 8-bit integer.
    fn deserialize_u8<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u8, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize an unsigned 16-bit integer.
    fn deserialize_u16<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize an unsigned 32-bit integer.
    fn deserialize_u32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize an unsigned 64-bit integer.
    fn deserialize_u64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize an unsigned 128-bit integer.
    fn deserialize_u128<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a `usize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_usize<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(usize, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_u64(input).map(|(val, rem)| (val as usize, rem))
    }

    /// Deserialize a signed 8-bit integer.
    fn deserialize_i8<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i8, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a signed 16-bit integer.
    fn deserialize_i16<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a signed 32-bit integer.
    fn deserialize_i32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a signed 64-bit integer.
    fn deserialize_i64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a signed 128-bit integer.
    fn deserialize_i128<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize an `isize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_isize<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(isize, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_i64(input).map(|(val, rem)| (val as isize, rem))
    }

    /// Deserialize a single-precision floating-point value.
    fn deserialize_f32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a double-precision floating-point value.
    fn deserialize_f64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a boolean value.
    #[inline(always)]
    fn deserialize_bool<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(bool, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        match input.split_first() {
            Some((0u8, remainder)) => Ok((false, remainder)),
            Some((1u8, remainder)) => Ok((true, remainder)),
            Some(..) => todo!("Return an error gracefully"),
            None => todo!("Return an error gracefully"),
        }
    }

    /// Deserialize a UTF-8 string slice.
    fn deserialize_str<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(&'input str, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;
}

/// An extension trait for [`Deserializer`] that provides
/// variable-length deserialization methods.
pub trait DeserializerExt: Deserializer {
    /// Deserialize a variable-length unsigned 16-bit integer.
    fn deserialize_var_u16<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;
    /// Deserialize a variable-length unsigned 32-bit integer.
    fn deserialize_var_u32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;
    /// Deserialize a variable-length unsigned 64-bit integer.
    fn deserialize_var_u64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;
    /// Deserialize a variable-length unsigned 128-bit integer.
    fn deserialize_var_u128<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'facet, 'shape>>;

    /// Deserialize a variable-length `usize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_var_usize<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(usize, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_var_u64(input).map(|(val, rem)| (val as usize, rem))
    }

    /// Deserialize a variable-length signed 16-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i16<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_var_u16(input).map(|(val, rem)| (val as i16, rem))
    }
    /// Deserialize a variable-length signed 32-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_var_u32(input).map(|(val, rem)| (val as i32, rem))
    }
    /// Deserialize a variable-length signed 64-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_var_u64(input).map(|(val, rem)| (val as i64, rem))
    }
    /// Deserialize a variable-length signed 128-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_var_i128<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_var_u128(input).map(|(val, rem)| (val as i128, rem))
    }

    /// Deserialize a variable-length signed `isize` integer.
    #[inline(always)]
    #[expect(clippy::cast_possible_truncation)]
    fn deserialize_var_isize<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(isize, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        self.deserialize_var_i64(input).map(|(val, rem)| (val as isize, rem))
    }
}

// -------------------------------------------------------------------------------------------------

impl Deserializer for McDeserializer {
    fn deserialize_unit<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<((), &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        Ok(((), input))
    }

    fn deserialize_u8<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u8, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&byte, remainder)) = input.split_first() {
            Ok((byte, remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_u16<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<2>() {
            Ok((u16::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_u32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<4>() {
            Ok((u32::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_u64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<8>() {
            Ok((u64::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_u128<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<16>() {
            Ok((u128::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    #[expect(clippy::cast_possible_wrap)]
    fn deserialize_i8<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i8, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&byte, remainder)) = input.split_first() {
            Ok((byte as i8, remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_i16<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<2>() {
            Ok((i16::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_i32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<4>() {
            Ok((i32::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_i64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<8>() {
            Ok((i64::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_i128<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<16>() {
            Ok((i128::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_f32<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<4>() {
            Ok((f32::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_f64<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        if let Some((&chunk, remainder)) = input.split_first_chunk::<8>() {
            Ok((f64::from_le_bytes(chunk), remainder))
        } else {
            todo!("Return an error gracefully")
        }
    }

    fn deserialize_str<'input, 'facet, 'shape>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(&'input str, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        let (len, remainder) = self.deserialize_var_usize(input)?;
        if let Some((str_bytes, remainder)) = remainder.split_at_checked(len) {
            match core::str::from_utf8(str_bytes) {
                Ok(s) => Ok((s, remainder)),
                Err(_err) => todo!("Return an error gracefully"),
            }
        } else {
            todo!("Return an error gracefully")
        }
    }
}

impl DeserializerExt for McDeserializer {
    fn deserialize_var_u16<'input, 'facet, 'shape>(
        &mut self,
        mut input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        let mut number: u16 = 0;
        for i in 0..3 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u16::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                todo!("Return an error gracefully");
            }
        }
        Ok((number, input))
    }

    fn deserialize_var_u32<'input, 'facet, 'shape>(
        &mut self,
        mut input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        let mut number: u32 = 0;
        for i in 0..5 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u32::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                todo!("Return an error gracefully");
            }
        }
        Ok((number, input))
    }

    fn deserialize_var_u64<'input, 'facet, 'shape>(
        &mut self,
        mut input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        let mut number: u64 = 0;
        for i in 0..10 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u64::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                todo!("Return an error gracefully");
            }
        }
        Ok((number, input))
    }

    fn deserialize_var_u128<'input, 'facet, 'shape>(
        &mut self,
        mut input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        let mut number: u128 = 0;
        for i in 0..19 {
            if let Some((&byte, remainder)) = input.split_first() {
                input = remainder;
                number |= u128::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                todo!("Return an error gracefully");
            }
        }
        Ok((number, input))
    }
}
