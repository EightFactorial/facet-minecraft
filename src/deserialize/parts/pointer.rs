use facet_reflect::Partial;

use crate::{
    DeserializeError, DeserializerExt,
    deserialize::{DeserializerState, StepType},
};

pub(crate) fn deserialize_smartpointer<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'shape>,
    _de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
> {
    state.steps.push(StepType::SmartPointer);

    // Begin the smart pointer.
    let pointer = current.begin_smart_ptr().map_err(|err| state.handle_reflect_error(err))?;

    Ok((pointer, input))
}
