use alloc::{borrow::Cow, string::ToString};
use core::ops::SubAssign;
use std::collections::{HashMap, hash_map::Entry};

#[cfg(feature = "custom")]
use facet::ShapeAttribute;
use facet::{ArrayType, Def, FieldAttribute, SequenceType, Shape, SliceType, Type};
use facet_reflect::{HeapValue, Partial, ScalarType};

use crate::assert::AssertProtocol;
#[cfg(feature = "custom")]
use crate::custom::FacetOverride;

mod error;
pub use error::DeserializeError;

mod traits;
pub use traits::{Deserializer, DeserializerExt};

/// Deserialize a type from the given byte slice.
///
/// This is a wrapper around [`deserialize_iterative`],
/// using [`McDeserializer`] as the deserializer.
///
/// # Errors
/// Returns an error if the deserialization fails.
#[inline(always)]
#[expect(clippy::inline_always)]
pub fn deserialize<'input: 'facet, 'facet, 'shape, T: AssertProtocol<'facet>>(
    input: &'input [u8],
) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    McDeserializer::deserialize::<T>(input)
}

// -------------------------------------------------------------------------------------------------

/// A deserializer for Minecraft protocol data.
#[derive(Debug, Default, Clone, Copy)]
pub struct McDeserializer;

impl McDeserializer {
    /// Deserialize a type from the given byte slice.
    ///
    /// This is a wrapper around [`deserialize_iterative`],
    /// using [`McDeserializer`] as the deserializer.
    ///
    /// # Errors
    /// Returns an error if the deserialization fails.
    #[inline(always)]
    #[expect(clippy::inline_always)]
    pub fn deserialize<'input: 'facet, 'facet, 'shape, T: AssertProtocol<'facet>>(
        input: &'input [u8],
    ) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        let () = const { <T as AssertProtocol<'facet>>::ASSERT };

        deserialize_iterative::<T, McDeserializer>(input, T::SHAPE, McDeserializer)
    }
}

// -------------------------------------------------------------------------------------------------

/// Iteratively deserialize a type from the given bytes.
///
/// Avoids recursion to prevent depth issues with large structures.
///
/// # Errors
/// Returns an error if the deserialization fails.
pub fn deserialize_iterative<
    'input: 'facet,
    'facet,
    'shape,
    T: AssertProtocol<'facet>,
    D: DeserializerExt,
>(
    input: &'input [u8],
    shape: &'shape Shape<'shape>,
    mut de: D,
) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    let partial = match Partial::alloc_shape(shape) {
        Ok(partial) => partial,
        Err(_err) => todo!(),
    };

    let (heap, rem) = match deserialize_value::<D>(input, partial, &mut de) {
        Ok(value) => value,
        Err(_err) => todo!(),
    };

    match heap.materialize::<T>() {
        Ok(value) => Ok((value, rem)),
        Err(_err) => todo!(),
    }
}

#[expect(clippy::too_many_lines, unused_variables)]
fn deserialize_value<'input: 'facet, 'facet, 'shape, D: DeserializerExt>(
    mut input: &'input [u8],
    mut partial: Partial<'facet, 'shape>,
    de: &mut D,
) -> Result<(HeapValue<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    static _VAR: &FieldAttribute = &FieldAttribute::Arbitrary("var");
    #[cfg(feature = "json")]
    static _JSON: &FieldAttribute = &FieldAttribute::Arbitrary("json");

    #[cfg(feature = "custom")]
    static CUSTOM: &ShapeAttribute = &ShapeAttribute::Arbitrary("custom");
    #[cfg(feature = "custom")]
    let overrides = FacetOverride::global();

    let mut current = &mut partial;
    let mut lists = HashMap::new();

    loop {
        // Use the inner type if the shape has the `transparent` attribute.
        if current.shape().attributes.contains(&ShapeAttribute::Transparent) {
            current = current.begin_inner().unwrap();
        }

        // If the shape has a `custom` attribute,
        // check for a custom deserialization function.
        #[cfg(feature = "custom")]
        if current.shape().attributes.contains(CUSTOM)
            && let Some(custom) = overrides.iter().find(|o| o.id == current.shape().id)
            && let Some(de) = custom.deserialize
        {
            match de(current, input) {
                Ok((part, rem)) => {
                    current = part;
                    input = rem;
                    continue;
                }
                Err(_err) => todo!(),
            }
        }

        // Deserialize the value
        match current.shape().def {
            Def::Scalar => match current.shape().ty {
                Type::Primitive(..) => {
                    match deserialize_primitive(current, &mut lists, input, de) {
                        Ok((part, rem)) => {
                            current = part;
                            input = rem;
                        }
                        Err(_err) => todo!(),
                    }
                }
                Type::Sequence(ty) => {
                    match deserialize_sequence(current, &mut lists, input, ty, de) {
                        Ok((part, rem)) => {
                            current = part;
                            input = rem;
                        }
                        Err(_err) => todo!(),
                    }
                }
                Type::Pointer(ty) => todo!(),
                Type::User(ty) => todo!(),
            },
            Def::Slice(def) => {
                let ty = SequenceType::Slice(SliceType { t: def.t() });
                match deserialize_sequence(current, &mut lists, input, ty, de) {
                    Ok((part, rem)) => {
                        current = part;
                        input = rem;
                    }
                    Err(_err) => todo!(),
                }
            }
            Def::List(def) => {
                let ty = SequenceType::Slice(SliceType { t: def.t() });
                match deserialize_sequence(current, &mut lists, input, ty, de) {
                    Ok((part, rem)) => {
                        current = part;
                        input = rem;
                    }
                    Err(_err) => todo!(),
                }
            }
            Def::Array(def) => {
                let ty = SequenceType::Array(ArrayType { t: def.t, n: def.n });
                match deserialize_sequence(current, &mut lists, input, ty, de) {
                    Ok((part, rem)) => {
                        current = part;
                        input = rem;
                    }
                    Err(_err) => todo!(),
                }
            }
            Def::Map(def) => todo!(),
            Def::Set(def) => todo!(),
            Def::Option(def) => todo!(),
            Def::SmartPointer(def) => todo!(),
            Def::Undefined => todo!(),
        }

        // If we've finished the last frame, break the loop.
        if current.frame_count() == 1 {
            break;
        }
    }

    // Build the deserialized value.
    match partial.build() {
        Ok(heap) => Ok((heap, input)),
        Err(_err) => todo!(),
    }
}

// -------------------------------------------------------------------------------------------------

fn deserialize_primitive<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    lists: &mut HashMap<usize, usize>,
    input: &'input [u8],
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
>
where
    'input: 'partial + 'facet,
{
    macro_rules! deserialize_scalar {
        ($deserialize_fn:ident) => {
            match de.$deserialize_fn(input) {
                Ok((val, rem)) => match current.set(val) {
                    Ok(partial) => handle_list_result(partial, rem, lists),
                    Err(_err) => todo!(),
                },
                Err(_err) => todo!(),
            }
        };
        ($deserialize_fn:ident, $($map_fn:tt)+) => {
            match de.$deserialize_fn(input).map($($map_fn)+) {
                Ok((val, rem)) => match current.set(val) {
                    Ok(partial) => handle_list_result(partial, rem, lists),
                    Err(_err) => todo!(),
                },
                Err(_err) => todo!(),
            }
        };
    }

    fn handle_list_result<'input, 'partial, 'facet, 'shape>(
        mut partial: &'partial mut Partial<'facet, 'shape>,
        input: &'input [u8],
        lists: &mut HashMap<usize, usize>,
    ) -> Result<
        (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
        DeserializeError<'input, 'facet, 'shape>,
    > {
        let frame_count = partial.frame_count().saturating_sub(1);
        if let Some(n) = lists.get_mut(&frame_count) {
            *n = n.saturating_sub(1);

            match partial.end() {
                Ok(part) => partial = part,
                Err(_err) => todo!(),
            }

            if *n == 0 {
                lists.remove(&frame_count);

                Ok((partial, input))
            } else {
                match partial.begin_list_item() {
                    Ok(part) => Ok((part, input)),
                    Err(_err) => todo!(),
                }
            }
        } else {
            Ok((partial, input))
        }
    }

    #[cfg_attr(rustfmt, rustfmt::skip)]
    match ScalarType::try_from_shape(current.shape()) {
        Some(ScalarType::Unit) => deserialize_scalar!(deserialize_unit),
        Some(ScalarType::Bool) => deserialize_scalar!(deserialize_bool),
        Some(ScalarType::U8) => deserialize_scalar!(deserialize_u8),
        Some(ScalarType::U16) => deserialize_scalar!(deserialize_u16),
        Some(ScalarType::U32) => deserialize_scalar!(deserialize_u32),
        Some(ScalarType::U64) => deserialize_scalar!(deserialize_u64),
        Some(ScalarType::U128) => deserialize_scalar!(deserialize_u128),
        Some(ScalarType::USize) => deserialize_scalar!(deserialize_usize),
        Some(ScalarType::I8) => deserialize_scalar!(deserialize_i8),
        Some(ScalarType::I16) => deserialize_scalar!(deserialize_i16),
        Some(ScalarType::I32) => deserialize_scalar!(deserialize_i32),
        Some(ScalarType::I64) => deserialize_scalar!(deserialize_i64),
        Some(ScalarType::I128) => deserialize_scalar!(deserialize_i128),
        Some(ScalarType::ISize) => deserialize_scalar!(deserialize_isize),
        Some(ScalarType::F32) => deserialize_scalar!(deserialize_f32),
        Some(ScalarType::F64) => deserialize_scalar!(deserialize_f64),
        Some(ScalarType::Str) => deserialize_scalar!(deserialize_str),
        Some(ScalarType::String) => deserialize_scalar!(deserialize_str, |(s, r)| (s.to_string(), r)),
        Some(ScalarType::CowStr) => deserialize_scalar!(deserialize_str, |(s, r)| (Cow::Borrowed(s), r)),
        Some(..) => todo!(),
        None => todo!(),
    }
}

#[expect(dead_code)]
fn deserialize_var_primitive<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    lists: &mut HashMap<usize, usize>,
    input: &'input [u8],
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
>
where
    'input: 'partial + 'facet,
{
    macro_rules! var_deserialize_scalar {
        ($deserialize_fn:ident) => {
            match de.$deserialize_fn(input) {
                Ok((val, rem)) => match current.set(val) {
                    Ok(partial) => handle_list_result(partial, rem, lists),
                    Err(_err) => todo!(),
                },
                Err(_err) => todo!(),
            }
        };
    }

    fn handle_list_result<'input, 'partial, 'facet, 'shape>(
        mut partial: &'partial mut Partial<'facet, 'shape>,
        input: &'input [u8],
        lists: &mut HashMap<usize, usize>,
    ) -> Result<
        (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
        DeserializeError<'input, 'facet, 'shape>,
    > {
        let frame_count = partial.frame_count().saturating_sub(1);
        if let Some(n) = lists.get_mut(&frame_count) {
            *n = n.saturating_sub(1);

            match partial.end() {
                Ok(part) => partial = part,
                Err(_err) => todo!(),
            }

            if *n == 0 {
                lists.remove(&frame_count);

                Ok((partial, input))
            } else {
                match partial.begin_list_item() {
                    Ok(part) => Ok((part, input)),
                    Err(_err) => todo!(),
                }
            }
        } else {
            Ok((partial, input))
        }
    }

    #[cfg_attr(rustfmt, rustfmt::skip)]
    match ScalarType::try_from_shape(current.shape()) {
        Some(ScalarType::U16) => var_deserialize_scalar!(deserialize_var_u16),
        Some(ScalarType::U32) => var_deserialize_scalar!(deserialize_var_u32),
        Some(ScalarType::U64) => var_deserialize_scalar!(deserialize_var_u64),
        Some(ScalarType::U128) => var_deserialize_scalar!(deserialize_var_u128),
        Some(ScalarType::USize) => var_deserialize_scalar!(deserialize_var_usize),
        Some(ScalarType::I16) => var_deserialize_scalar!(deserialize_var_i16),
        Some(ScalarType::I32) => var_deserialize_scalar!(deserialize_var_i32),
        Some(ScalarType::I64) => var_deserialize_scalar!(deserialize_var_i64),
        Some(ScalarType::I128) => var_deserialize_scalar!(deserialize_var_i128),
        Some(ScalarType::ISize) => var_deserialize_scalar!(deserialize_var_isize),
        Some(..) => todo!(),
        None => todo!(),
    }
}

// -------------------------------------------------------------------------------------------------

fn deserialize_sequence<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    lists: &mut HashMap<usize, usize>,
    mut input: &'input [u8],
    ty: SequenceType,
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
>
where
    'input: 'partial + 'facet,
{
    match lists.entry(current.frame_count()) {
        Entry::Occupied(mut entry) => {
            match *entry.get() {
                // Remove the entry and finish the list.
                0 => {
                    // Remove the list from the map.
                    entry.remove();

                    // Finish the list.
                    match current.end() {
                        Ok(part) => Ok((part, input)),
                        Err(_err) => todo!(),
                    }
                }
                _ => {
                    // Decrement the remaining item count.
                    entry.get_mut().sub_assign(1);

                    // Begin the next item.
                    match current.begin_list_item() {
                        Ok(part) => Ok((part, input)),
                        Err(_err) => todo!(),
                    }
                }
            }
        }
        Entry::Vacant(entry) => {
            // Get the list length.
            let len = match ty {
                // Use the given item count.
                SequenceType::Array(ty) => ty.n,
                // Read the item count.
                SequenceType::Slice(..) => match de.deserialize_var_usize(input) {
                    Ok((len, rem)) => {
                        input = rem;
                        len
                    }
                    Err(_err) => todo!(),
                },
            };

            // Begin the list.
            let Ok(part) = current.begin_list() else { todo!() };

            match len {
                // Return the empty list.
                0 => Ok((part, input)),
                // Begin the first item.
                other => {
                    // Keep track of the number of remaining items.
                    entry.insert(other);

                    match part.begin_list_item() {
                        Ok(part) => Ok((part, input)),
                        Err(_err) => todo!(),
                    }
                }
            }
        }
    }
}
