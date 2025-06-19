use alloc::vec::Vec;

#[cfg(feature = "custom")]
use facet::ShapeAttribute;
use facet::{FieldAttribute, Shape};
use facet_reflect::{HeapValue, Partial};

use crate::assert::AssertProtocol;
#[cfg(feature = "custom")]
use crate::custom::FacetOverride;

mod error;
pub use error::DeserializeError;

mod traits;
pub use traits::{Deserializer, DeserializerExt};

/// A deserializer for Minecraft protocol data.
#[derive(Debug, Default, Clone, Copy)]
pub struct McDeserializer;

/// Deserialize a type from the given byte slice.
///
/// # Errors
/// Returns an error if the deserialization fails.
#[inline]
pub fn deserialize<'input, 'facet, 'shape, T: AssertProtocol<'facet>>(
    input: &'input [u8],
) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    <T as AssertProtocol<'facet>>::assert();

    deserialize_iterative::<T, McDeserializer>(input, T::SHAPE, McDeserializer)
}

// -------------------------------------------------------------------------------------------------

/// Iteratively deserialize a type from the given bytes.
///
/// Avoids recursion to prevent depth issues with large structures.
///
/// # Errors
/// Returns an error if the deserialization fails.
pub fn deserialize_iterative<
    'input,
    'facet,
    'shape,
    T: AssertProtocol<'facet>,
    D: DeserializerExt,
>(
    input: &'input [u8],
    shape: &'shape Shape<'shape>,
    mut des: D,
) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    let partial = match Partial::alloc_shape(shape) {
        Ok(partial) => partial,
        Err(_err) => todo!(),
    };

    let (heap, rem) = match deserialize_value::<D>(input, partial, shape, &mut des) {
        Ok(value) => value,
        Err(_err) => todo!(),
    };

    match heap.materialize::<T>() {
        Ok(value) => Ok((value, rem)),
        Err(_err) => todo!(),
    }
}

#[expect(clippy::vec_init_then_push, unused_mut, unused_variables)]
fn deserialize_value<'input, 'facet, 'shape, D: DeserializerExt>(
    mut input: &'input [u8],
    mut partial: Partial<'facet, 'shape>,
    shape: &'shape Shape<'shape>,
    _des: &mut D,
) -> Result<(HeapValue<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    static _VAR: &FieldAttribute = &FieldAttribute::Arbitrary("var");
    #[cfg(feature = "custom")]
    static _CUSTOM: &ShapeAttribute = &ShapeAttribute::Arbitrary("custom");
    #[cfg(feature = "json")]
    static _JSON: &FieldAttribute = &FieldAttribute::Arbitrary("json");

    #[cfg(feature = "custom")]
    let _overrides = FacetOverride::global();

    let mut stack = Vec::with_capacity(1);
    stack.push(DeserializationTask::Placeholder(core::marker::PhantomData));

    todo!();

    #[expect(unreachable_code)]
    match partial.build() {
        Ok(heap) => Ok((heap, input)),
        Err(_err) => todo!(),
    }
}

/// A task to be performed during deserialization.
#[expect(missing_docs)]
pub enum DeserializationTask<'input, 'facet, 'shape> {
    Placeholder(core::marker::PhantomData<(&'input (), &'facet (), &'shape ())>),
}
