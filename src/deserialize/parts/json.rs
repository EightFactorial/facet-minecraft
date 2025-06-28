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
    todo!()
}
