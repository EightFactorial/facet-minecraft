use facet::SequenceType;
use facet_reflect::Partial;

use crate::{DeserializeError, DeserializerExt, deserialize::DeserializerState};

pub(crate) fn deserialize_sequence<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    ty: SequenceType,
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'shape>,
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
>
where
    'input: 'partial + 'facet,
{
    match ty {
        SequenceType::Array(ty) => array(current, input, ty.n, state, de),
        SequenceType::Slice(..) => list(current, input, state, de),
    }
}

// -------------------------------------------------------------------------------------------------

fn array<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    _current: &'partial mut Partial<'facet, 'shape>,
    _input: &'input [u8],
    _size: usize,
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

fn list<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
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
