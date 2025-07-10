use alloc::string::ToString;

use facet_reflect::{Partial, ScalarType};

use crate::{DeserializeError, DeserializerExt, deserialize::DeserializerState};

pub(crate) fn deserialize_primitive<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    if state.variable() {
        var_primitive(current, input, state, de)
    } else {
        primitive(current, input, state, de)
    }
}

// -------------------------------------------------------------------------------------------------

fn primitive<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    macro_rules! deserialize_scalar {
        ($deserialize_fn:ident) => {{
            let (val, rem) = de.$deserialize_fn(input).map_err(|err| state.handle_deserialize_error(err))?;
            match current.set(val) {
                Ok(partial) => state.update_state(partial, rem),
                Err(err) => Err(state.handle_reflect_error(err)),
            }
        }};
        ($deserialize_fn:ident, $($map_fn:tt)+) => {{
            let (val, rem) = de.$deserialize_fn(input).map_or_else(|err| Err(state.handle_deserialize_error(err)), $($map_fn)+)?;
            match current.set(val) {
                Ok(partial) => state.update_state(partial, rem),
                Err(err) => Err(state.handle_reflect_error(err)),
            }
        }};
    }

    match ScalarType::try_from_shape(current.shape()) {
        Some(ScalarType::Unit) => deserialize_scalar!(deserialize_unit),
        Some(ScalarType::Bool) => deserialize_scalar!(deserialize_bool),
        Some(ScalarType::U8) => deserialize_scalar!(deserialize_u8),
        Some(ScalarType::U16) => deserialize_scalar!(deserialize_u16),
        Some(ScalarType::U32) => deserialize_scalar!(deserialize_u32),
        Some(ScalarType::U64) => deserialize_scalar!(deserialize_u64),
        Some(ScalarType::U128) => deserialize_scalar!(deserialize_u128),
        Some(ScalarType::USize) => deserialize_scalar!(deserialize_usize),
        Some(ScalarType::I8) => deserialize_scalar!(deserialize_i8),
        Some(ScalarType::I16) => deserialize_scalar!(deserialize_i16),
        Some(ScalarType::I32) => deserialize_scalar!(deserialize_i32),
        Some(ScalarType::I64) => deserialize_scalar!(deserialize_i64),
        Some(ScalarType::I128) => deserialize_scalar!(deserialize_i128),
        Some(ScalarType::ISize) => deserialize_scalar!(deserialize_isize),
        Some(ScalarType::F32) => deserialize_scalar!(deserialize_f32),
        Some(ScalarType::F64) => deserialize_scalar!(deserialize_f64),
        Some(ScalarType::String) => {
            deserialize_scalar!(deserialize_str, |(s, r)| Ok((s.to_string(), r)))
        }
        Some(..) => todo!(),
        None => todo!(),
    }
}

// -------------------------------------------------------------------------------------------------

fn var_primitive<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    macro_rules! deserialize_var_scalar {
        ($deserialize_fn:ident) => {{
            let (val, rem) =
                de.$deserialize_fn(input).map_err(|err| state.handle_deserialize_error(err))?;
            match current.set(val) {
                Ok(partial) => state.update_state(partial, rem),
                Err(err) => Err(state.handle_reflect_error(err)),
            }
        }};
    }

    match ScalarType::try_from_shape(current.shape()) {
        Some(ScalarType::U16) => deserialize_var_scalar!(deserialize_var_u16),
        Some(ScalarType::U32) => deserialize_var_scalar!(deserialize_var_u32),
        Some(ScalarType::U64) => deserialize_var_scalar!(deserialize_var_u64),
        Some(ScalarType::U128) => deserialize_var_scalar!(deserialize_var_u128),
        Some(ScalarType::USize) => deserialize_var_scalar!(deserialize_var_usize),
        Some(ScalarType::I16) => deserialize_var_scalar!(deserialize_var_i16),
        Some(ScalarType::I32) => deserialize_var_scalar!(deserialize_var_i32),
        Some(ScalarType::I64) => deserialize_var_scalar!(deserialize_var_i64),
        Some(ScalarType::I128) => deserialize_var_scalar!(deserialize_var_i128),
        Some(ScalarType::ISize) => deserialize_var_scalar!(deserialize_var_isize),
        Some(..) => todo!(),
        None => todo!(),
    }
}
