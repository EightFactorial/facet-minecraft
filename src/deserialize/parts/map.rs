use facet::{MapDef, SetDef};
use facet_reflect::Partial;

use crate::{
    DeserializeError, DeserializerExt,
    deserialize::{DeserializerState, StepType},
};

pub(crate) fn deserialize_map<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    _def: MapDef<'shape>,
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'shape>,
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
> {
    let (length, remaining) =
        de.deserialize_var_usize(input).map_err(|err| state.handle_deserialize_error(err))?;

    // Push the step for the map.
    state.steps.push(StepType::Map(length, 0));

    let map = current.begin_map().map_err(|err| state.handle_reflect_error(err))?;

    if length != 0 {
        // Begin the first key in the map.
        let key = map.begin_key().map_err(|err| state.handle_reflect_error(err))?;

        Ok((key, remaining))
    } else {
        // Otherwise return the empty map.
        Ok((map, remaining))
    }
}

// -------------------------------------------------------------------------------------------------

pub(crate) fn deserialize_set<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    _def: SetDef<'shape>,
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'shape>,
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
> {
    let (length, remaining) =
        de.deserialize_var_usize(input).map_err(|err| state.handle_deserialize_error(err))?;

    // Push the step for the map.
    state.steps.push(StepType::Set(length, 0));

    // TODO: Use the correct `begin_set` method when available.
    let map = current.begin_map().map_err(|err| state.handle_reflect_error(err))?;

    if length != 0 {
        // Begin the first item in the set.
        // TODO: Use the correct `begin_set_item` method when available.
        let key = map.begin_key().map_err(|err| state.handle_reflect_error(err))?;

        Ok((key, remaining))
    } else {
        // Otherwise return the empty map.
        Ok((map, remaining))
    }
}
