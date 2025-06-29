use facet_reflect::Partial;

use crate::{DeserializeError, DeserializerExt, deserialize::DeserializerState};

#[rustfmt::skip]
pub(crate) fn deserialize_json<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'shape>,
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
> {
    // Deserialize the string from the input.
    let (_content, _remaining) =
        de.deserialize_str(input).map_err(|err| state.handle_deserialize_error(err))?;

    // Create a new owned `Partial`.
    let _owned =
        Partial::alloc_shape(current.shape()).map_err(|err| state.handle_reflect_error(err))?;

    todo!()

    // TODO: Currently `facet_json::Json` is not public, so we cannot use it directly.
    //
    // // Deserialize the type from the string content.
    // let value = facet_deserialize::deserialize_wip(owned, content, facet_json::Json)
    //     .map_err(|err| todo!("TODO: Handle JSON error: {err}"))?;
    //
    // current.set_from_peek(&value.peek()).map_err(|err| state.handle_reflect_error(err))?;
    //
    // Ok((current, remaining))
}
