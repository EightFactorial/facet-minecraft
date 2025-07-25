use facet::SequenceType;
use facet_reflect::Partial;

use crate::{
    DeserializeError, DeserializerExt,
    deserialize::{DeserializerState, StepType},
};

pub(crate) fn deserialize_sequence<'input, 'partial, 'facet, D: DeserializerExt>(
    ty: SequenceType,
    current: &'partial mut Partial<'facet>,
    input: &'input [u8],
    state: &mut DeserializerState<'input>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet>, &'input [u8]), DeserializeError<'input>> {
    match ty {
        SequenceType::Array(ty) => deserialize_array(current, input, ty.n, state, de),
        SequenceType::Slice(..) => deserialize_list(current, input, state, de),
    }
}

// -------------------------------------------------------------------------------------------------

fn deserialize_array<'input, 'partial, 'facet, D: DeserializerExt>(
    current: &'partial mut Partial<'facet>,
    input: &'input [u8],
    length: usize,
    state: &mut DeserializerState<'input>,
    _de: &mut D,
) -> Result<(&'partial mut Partial<'facet>, &'input [u8]), DeserializeError<'input>> {
    state.steps.push(StepType::Sequence(length, 0));

    // Begin the list.
    let list = current.begin_list().map_err(|err| state.handle_reflect_error(err))?;

    if length != 0 {
        // Begin the first item in the list.
        let item = list.begin_list_item().map_err(|err| state.handle_reflect_error(err))?;

        Ok((item, input))
    } else {
        // Otherwise return the empty list.
        Ok((list, input))
    }
}

fn deserialize_list<'input, 'partial, 'facet, D: DeserializerExt>(
    current: &'partial mut Partial<'facet>,
    input: &'input [u8],
    state: &mut DeserializerState<'input>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet>, &'input [u8]), DeserializeError<'input>> {
    // Deserialize the size of the list.
    let (length, remaining) =
        de.deserialize_var_usize(input).map_err(|err| state.handle_deserialize_error(err))?;

    // Then deserialize exactly like the array.
    deserialize_array(current, remaining, length, state, de)
}
