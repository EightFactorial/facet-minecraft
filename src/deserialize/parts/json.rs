use facet_reflect::Partial;

use crate::{DeserializeError, DeserializerExt, deserialize::DeserializerState};

pub(crate) fn deserialize_json<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    _current: &'partial mut Partial<'facet, 'shape>,
    _input: &'input [u8],
    _state: &mut DeserializerState<'shape>,
    _de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
>
where
    'input: 'partial + 'facet,
{
    // let owned = Partial::alloc_shape(current.shape())
    //     .map_err(|err| todo!("TODO: Handle JSON error: {err}"))?;

    // let value = facet_deserialize::deserialize_wip(owned, input,
    // facet_json::Json)     .map_err(|err| todo!("TODO: Handle JSON error:
    // {err}"))?;

    todo!()
}
