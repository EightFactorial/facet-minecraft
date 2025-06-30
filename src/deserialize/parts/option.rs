use facet_reflect::Partial;

use crate::{
    DeserializeError, DeserializerExt,
    deserialize::{DeserializerState, StepType},
};

pub(crate) fn deserialize_option<'input, 'partial, 'facet: 'shape, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    let (is_some, remaining) =
        de.deserialize_bool(input).map_err(|err| state.handle_deserialize_error(err))?;

    state.steps.push(StepType::ValueHolder);

    if is_some {
        // Begin deserializing `Some`
        let some = current.begin_some().map_err(|err| state.handle_reflect_error(err))?;

        // Deserialize the value inside the `Some`
        Ok((some, remaining))
    } else {
        // Return `None`
        // TODO: Is this the best way to handle `None`?
        let none = current.set_default().map_err(|err| state.handle_reflect_error(err))?;

        state.update_state(none, remaining)
    }
}
