//! Provided methods to parse various data types.

use alloc::borrow::Cow;

use facet_format::{ScalarTypeHint, ScalarValue};

use crate::deserialize::{DeserializeError, DeserializeErrorKind};

/// A wrapper over [`parse_input`] that returns owned data.
#[allow(dead_code, reason = "May not be used if no async features are enabled")]
pub(crate) fn parse_owned_scalar(
    input: &[u8],
    hint: ScalarTypeHint,
    variable: bool,
) -> Result<(ScalarValue<'static>, usize), DeserializeError> {
    let (value, size) = parse_scalar(input, hint, variable)?;
    match value {
        ScalarValue::Null => Ok((ScalarValue::Null, size)),
        ScalarValue::Bool(v) => Ok((ScalarValue::Bool(v), size)),
        ScalarValue::I64(v) => Ok((ScalarValue::I64(v), size)),
        ScalarValue::U64(v) => Ok((ScalarValue::U64(v), size)),
        ScalarValue::I128(v) => Ok((ScalarValue::I128(v), size)),
        ScalarValue::U128(v) => Ok((ScalarValue::U128(v), size)),
        ScalarValue::F64(v) => Ok((ScalarValue::F64(v), size)),
        ScalarValue::Str(cow) => Ok((ScalarValue::Str(Cow::Owned(cow.into_owned())), size)),
        ScalarValue::Bytes(cow) => Ok((ScalarValue::Bytes(Cow::Owned(cow.into_owned())), size)),
    }
}

/// Parse a [`ScalarValue`] from the given input and [`ScalarTypeHint`].
#[expect(clippy::too_many_lines, reason = "Complex deserializer for many types")]
pub(crate) fn parse_scalar(
    input: &[u8],
    hint: ScalarTypeHint,
    variable: bool,
) -> Result<(ScalarValue<'_>, usize), DeserializeError> {
    macro_rules! as_chunk {
        ($N:expr) => {{
            if let Some(chunk) = input.first_chunk::<$N>() {
                chunk
            } else {
                return Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                    expected: $N,
                    found: input.len(),
                }));
            }
        }};
    }

    match (hint, variable) {
        // Fixed-length types
        (ScalarTypeHint::Bool, false) => match as_chunk!(1)[0] {
            0 => Ok((ScalarValue::Bool(false), 1)),
            1 => Ok((ScalarValue::Bool(true), 1)),
            other => Err(DeserializeError::new(DeserializeErrorKind::InvalidBool(other))),
        },
        (ScalarTypeHint::U8, false) => {
            let value = as_chunk!(1)[0];
            Ok((ScalarValue::U64(u64::from(value)), 1))
        }
        (ScalarTypeHint::U16, false) => {
            let value = u16::from_be_bytes(*as_chunk!(2));
            Ok((ScalarValue::U64(u64::from(value)), 2))
        }
        (ScalarTypeHint::U32 | ScalarTypeHint::Usize, false) => {
            let value = u32::from_be_bytes(*as_chunk!(4));
            Ok((ScalarValue::U64(u64::from(value)), 4))
        }
        (ScalarTypeHint::U64, false) => {
            let value = u64::from_be_bytes(*as_chunk!(8));
            Ok((ScalarValue::U64(value), 8))
        }
        (ScalarTypeHint::U128, false) => {
            let value = u128::from_be_bytes(*as_chunk!(16));
            Ok((ScalarValue::U128(value), 16))
        }
        #[expect(clippy::cast_possible_wrap, reason = "This is desired behavior")]
        (ScalarTypeHint::I8, false) => {
            let value = as_chunk!(1)[0] as i8;
            Ok((ScalarValue::I64(i64::from(value)), 1))
        }
        (ScalarTypeHint::I16, false) => {
            let value = i16::from_be_bytes(*as_chunk!(2));
            Ok((ScalarValue::I64(i64::from(value)), 2))
        }
        (ScalarTypeHint::I32 | ScalarTypeHint::Isize, false) => {
            let value = i32::from_be_bytes(*as_chunk!(4));
            Ok((ScalarValue::I64(i64::from(value)), 4))
        }
        (ScalarTypeHint::I64, false) => {
            let value = i64::from_be_bytes(*as_chunk!(8));
            Ok((ScalarValue::I64(value), 8))
        }
        (ScalarTypeHint::I128, false) => {
            let value = i128::from_be_bytes(*as_chunk!(16));
            Ok((ScalarValue::I128(value), 16))
        }
        (ScalarTypeHint::F32, false) => {
            let value = f32::from_be_bytes(*as_chunk!(4));
            Ok((ScalarValue::F64(f64::from(value)), 4))
        }
        (ScalarTypeHint::F64, false) => {
            let value = f64::from_be_bytes(*as_chunk!(8));
            Ok((ScalarValue::F64(value), 8))
        }

        // Strings and Bytes
        (ScalarTypeHint::String, false) => {
            match parse_scalar(input, ScalarTypeHint::Bytes, false) {
                Ok((ScalarValue::Bytes(Cow::Borrowed(bytes)), size)) => {
                    let content = core::str::from_utf8(bytes)
                        .map_err(|_| DeserializeError::new(DeserializeErrorKind::InvalidUtf8))?;
                    Ok((ScalarValue::Str(Cow::Borrowed(content)), size))
                }
                Ok(_) => unreachable!(),
                Err(err) => Err(err),
            }
        }
        (ScalarTypeHint::Bytes, false) => {
            let (len, len_size) = parse_scalar(input, ScalarTypeHint::Usize, true)?;
            let len = match len {
                #[expect(clippy::cast_possible_truncation, reason = "")]
                ScalarValue::U64(v) => v as usize,
                ScalarValue::U128(v) => v as usize,
                _ => unreachable!(),
            };

            let expected = len_size + len;
            if let Some(slice) = input.get(len_size..expected) {
                Ok((ScalarValue::Bytes(Cow::Borrowed(slice)), expected))
            } else {
                Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                    expected,
                    found: input.len(),
                }))
            }
        }

        // Variable-length types
        (ScalarTypeHint::U16, true) => {
            var_u16(input).map(|(val, size)| (ScalarValue::U64(u64::from(val)), size))
        }
        (ScalarTypeHint::U32 | ScalarTypeHint::Usize, true) => {
            var_u32(input).map(|(val, size)| (ScalarValue::U64(u64::from(val)), size))
        }
        (ScalarTypeHint::U64, true) => {
            var_u64(input).map(|(val, size)| (ScalarValue::U64(val), size))
        }
        (ScalarTypeHint::U128, true) => {
            var_u128(input).map(|(val, size)| (ScalarValue::U128(val), size))
        }
        #[expect(clippy::cast_possible_wrap, reason = "This is desired behavior")]
        (ScalarTypeHint::I16, true) => {
            var_u16(input).map(|(val, size)| (ScalarValue::I64(i64::from(val as i16)), size))
        }
        #[expect(clippy::cast_possible_wrap, reason = "This is desired behavior")]
        (ScalarTypeHint::I32 | ScalarTypeHint::Isize, true) => {
            var_u32(input).map(|(val, size)| (ScalarValue::I64(i64::from(val as i32)), size))
        }
        #[expect(clippy::cast_possible_wrap, reason = "This is desired behavior")]
        (ScalarTypeHint::I64, true) => {
            var_u64(input).map(|(val, size)| (ScalarValue::I64(val as i64), size))
        }
        #[expect(clippy::cast_possible_wrap, reason = "This is desired behavior")]
        (ScalarTypeHint::I128, true) => {
            var_u128(input).map(|(v, s)| (ScalarValue::I128(v as i128), s))
        }

        // Unsupported variable-length types
        (ScalarTypeHint::Bool, true) => todo!(),
        (ScalarTypeHint::U8, true) => todo!(),
        (ScalarTypeHint::I8, true) => todo!(),
        (ScalarTypeHint::F32, true) => todo!(),
        (ScalarTypeHint::F64, true) => todo!(),
        (ScalarTypeHint::String, true) => todo!(),
        (ScalarTypeHint::Bytes, true) => todo!(),

        // Unsupported types
        (ScalarTypeHint::Char, _) => todo!(),
    }
}

// -------------------------------------------------------------------------------------------------

fn var_u16(input: &[u8]) -> Result<(u16, usize), DeserializeError> {
    let mut number: u16 = 0;
    let mut index = 0;

    while index < 3 {
        if let Some(&byte) = input.get(index) {
            number |= u16::from(byte & 0b0111_1111) << (7 * index);
            index += 1;
            if byte & 0b1000_0000 == 0 {
                break;
            }
        } else {
            Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                expected: 1,
                found: 0,
            }))?;
        }
    }

    Ok((number, index))
}

fn var_u32(input: &[u8]) -> Result<(u32, usize), DeserializeError> {
    let mut number: u32 = 0;
    let mut index = 0;

    while index < 5 {
        if let Some(&byte) = input.get(index) {
            number |= u32::from(byte & 0b0111_1111) << (7 * index);
            index += 1;
            if byte & 0b1000_0000 == 0 {
                break;
            }
        } else {
            Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                expected: 1,
                found: 0,
            }))?;
        }
    }

    Ok((number, index))
}

fn var_u64(input: &[u8]) -> Result<(u64, usize), DeserializeError> {
    let mut number: u64 = 0;
    let mut index = 0;

    while index < 10 {
        if let Some(&byte) = input.get(index) {
            number |= u64::from(byte & 0b0111_1111) << (7 * index);
            index += 1;
            if byte & 0b1000_0000 == 0 {
                break;
            }
        } else {
            Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                expected: 1,
                found: 0,
            }))?;
        }
    }

    Ok((number, index))
}

fn var_u128(input: &[u8]) -> Result<(u128, usize), DeserializeError> {
    let mut number: u128 = 0;
    let mut index = 0;

    while index < 19 {
        if let Some(&byte) = input.get(index) {
            number |= u128::from(byte & 0b0111_1111) << (7 * index);
            index += 1;
            if byte & 0b1000_0000 == 0 {
                break;
            }
        } else {
            Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                expected: 1,
                found: 0,
            }))?;
        }
    }

    Ok((number, index))
}
