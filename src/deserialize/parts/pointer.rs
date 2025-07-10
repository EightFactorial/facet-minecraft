use facet::PointerType;
use facet_reflect::Partial;

use crate::{
    DeserializeError, DeserializerExt,
    deserialize::{DeserializerState, StepType},
};

/// # TODO: FIX SOUNDNESS ISSUE
///
/// It is currently possible to deserialize a value into a static reference.
/// This is fine for static inputs, but not for non-static inputs!
#[expect(dead_code, unreachable_code, unused_variables)]
pub(crate) fn deserialize_pointer<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    ty: PointerType<'shape>,
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    match ty {
        PointerType::Function(..) | PointerType::Raw(..) => todo!(),
        PointerType::Reference(..) => {
            if current.shape().is_type::<&str>() {
                // Deserialize a string from the input.
                let (content, remaining) =
                    de.deserialize_str(input).map_err(|err| state.handle_deserialize_error(err))?;
                todo!("Set the value in the current partial");
                // current = current.set(content).map_err(|err|
                // state.handle_reflect_error(err))?;

                state.update_state(current, remaining)
            } else if current.shape().is_type::<&[u8]>() {
                // Deserialize the length of the byte slice.
                let (length, rem) = de
                    .deserialize_var_usize(input)
                    .map_err(|err| state.handle_deserialize_error(err))?;

                // Take the byte slice from the input.
                if let Some((content, remaining)) = rem.split_at_checked(length) {
                    todo!("Set the value in the current partial");
                    current =
                        current.set(content).map_err(|err| state.handle_reflect_error(err))?;

                    state.update_state(current, remaining)
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub(crate) fn deserialize_smartpointer<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    _de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    state.steps.push(StepType::ValueHolder);

    // Begin the smart pointer.
    let pointer = current.begin_smart_ptr().map_err(|err| state.handle_reflect_error(err))?;

    Ok((pointer, input))
}
