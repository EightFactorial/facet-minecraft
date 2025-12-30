//! Provided methods to parse various data types.

use alloc::borrow::Cow;

use facet_format::{ScalarTypeHint, ScalarValue};

use crate::deserialize::{DeserializeError, DeserializeErrorKind};

pub(crate) fn parse_input(
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
        (ScalarTypeHint::String, false) => todo!(),
        (ScalarTypeHint::Bytes, false) => todo!(),
        // Variable-length types
        (ScalarTypeHint::U8, true) => todo!(),
        (ScalarTypeHint::U16, true) => todo!(),
        (ScalarTypeHint::U32, true) => todo!(),
        (ScalarTypeHint::U64, true) => todo!(),
        (ScalarTypeHint::U128, true) => todo!(),
        (ScalarTypeHint::Usize, true) => todo!(),
        (ScalarTypeHint::I8, true) => todo!(),
        (ScalarTypeHint::I16, true) => todo!(),
        (ScalarTypeHint::I32, true) => todo!(),
        (ScalarTypeHint::I64, true) => todo!(),
        (ScalarTypeHint::I128, true) => todo!(),
        (ScalarTypeHint::Isize, true) => todo!(),
        // Unsupported variable-length types
        (ScalarTypeHint::Bool, true) => todo!(),
        (ScalarTypeHint::F32, true) => todo!(),
        (ScalarTypeHint::F64, true) => todo!(),
        (ScalarTypeHint::String, true) => todo!(),
        (ScalarTypeHint::Bytes, true) => todo!(),
        // Unsupported types
        (ScalarTypeHint::Char, _) => todo!(),
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper over [`parse_input`] that returns owned data.
#[allow(dead_code, reason = "May not be used if no async features are enabled")]
pub(crate) fn parse_input_owned(
    input: &[u8],
    hint: ScalarTypeHint,
    variable: bool,
) -> Result<(ScalarValue<'static>, usize), DeserializeError> {
    let (value, size) = parse_input(input, hint, variable)?;
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
