use alloc::{borrow::Cow, string::ToString, vec::Vec};
use core::ops::AddAssign;

#[cfg(feature = "custom")]
use facet::ShapeAttribute;
use facet::{ArrayType, Def, FieldAttribute, SequenceType, Shape, SliceType, Type, UserType};
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

static VAR: &FieldAttribute = &FieldAttribute::Arbitrary("var");
#[cfg(feature = "json")]
static JSON: &FieldAttribute = &FieldAttribute::Arbitrary("json");

#[cfg(feature = "custom")]
static CUSTOM: &ShapeAttribute = &ShapeAttribute::Arbitrary("custom");

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

#[expect(unused_variables)]
fn deserialize_value<'input: 'facet, 'facet, 'shape, D: DeserializerExt>(
    mut input: &'input [u8],
    mut partial: Partial<'facet, 'shape>,
    de: &mut D,
) -> Result<(HeapValue<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    #[cfg(feature = "custom")]
    let overrides = FacetOverride::global();

    let mut current = &mut partial;
    let mut counters = Vec::<(PartialType, usize)>::new();

    let (mut var, mut json) = (false, false);

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

                    if current.frame_count() == 1 {
                        // If we've finished the last frame, break the loop.
                        break;
                    } else {
                        // Otherwise continue to the next iteration.
                        continue;
                    }
                }
                Err(_err) => todo!(),
            }
        }

        #[cfg(feature = "json")]
        if json {
            todo!()
        }

        // Deserialize the value
        match current.shape().def {
            Def::Scalar | Def::Undefined => match current.shape().ty {
                Type::Primitive(..) => {
                    if var {
                        let flags = (&mut var, &mut json);
                        match deserialize_var_primitive(current, &mut counters, flags, input, de) {
                            Ok((part, rem)) => {
                                current = part;
                                input = rem;
                            }
                            Err(_err) => todo!(),
                        }
                    } else {
                        let flags = (&mut var, &mut json);
                        match deserialize_primitive(current, &mut counters, flags, input, de) {
                            Ok((part, rem)) => {
                                current = part;
                                input = rem;
                            }
                            Err(_err) => todo!(),
                        }
                    }
                }
                Type::Sequence(ty) => {
                    match deserialize_sequence(current, &mut counters, input, ty, de) {
                        Ok((part, rem)) => {
                            current = part;
                            input = rem;
                        }
                        Err(_err) => todo!(),
                    }
                }
                Type::Pointer(ty) => todo!(),
                Type::User(UserType::Struct(ty)) => match ty.fields.len() {
                    0 => todo!(),
                    other => {
                        // Mark the field to be variably deserialized.
                        if ty.fields[0].attributes.contains(VAR) {
                            var = true;
                        }

                        // Mark the field to be deserialized as JSON.
                        #[cfg(feature = "json")]
                        if ty.fields[0].attributes.contains(JSON) {
                            json = true;
                        }

                        counters.push((PartialType::StructField(1), other));
                        match current.begin_nth_field(0) {
                            Ok(part) => current = part,
                            Err(_err) => todo!(),
                        }
                    }
                },
                Type::User(UserType::Enum(ty)) => {
                    let Ok((variant, rem)) = de.deserialize_var_usize(input) else { todo!() };
                    input = rem;

                    if let Some(variant) = ty.variants.get(variant) {
                        match variant.data.fields.len() {
                            0 => todo!(),
                            other => {
                                // Mark the field to be variably deserialized.
                                if variant.data.fields[0].attributes.contains(VAR) {
                                    var = true;
                                }

                                // Mark the field to be deserialized as JSON.
                                #[cfg(feature = "json")]
                                if variant.data.fields[0].attributes.contains(JSON) {
                                    json = true;
                                }

                                counters.push((PartialType::EnumField(other, 1), other));
                                match current.begin_nth_enum_field(0) {
                                    Ok(part) => current = part,
                                    Err(_err) => todo!(),
                                }
                            }
                        }
                    } else {
                        todo!();
                    }
                }
                Type::User(..) => todo!(),
            },
            Def::Slice(def) => {
                let ty = SequenceType::Slice(SliceType { t: def.t() });
                match deserialize_sequence(current, &mut counters, input, ty, de) {
                    Ok((part, rem)) => {
                        current = part;
                        input = rem;
                    }
                    Err(_err) => todo!(),
                }
            }
            Def::List(def) => {
                let ty = SequenceType::Slice(SliceType { t: def.t() });
                match deserialize_sequence(current, &mut counters, input, ty, de) {
                    Ok((part, rem)) => {
                        current = part;
                        input = rem;
                    }
                    Err(_err) => todo!(),
                }
            }
            Def::Array(def) => {
                let ty = SequenceType::Array(ArrayType { t: def.t, n: def.n });
                match deserialize_sequence(current, &mut counters, input, ty, de) {
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
        }

        // If we've finished the last frame, break the loop.
        if current.frame_count() == 1 {
            break;
        }
    }

    // Build the deserialized value.
    match partial.build() {
        Ok(heap) => Ok((heap, input)),
        Err(err) => todo!("Failed to build: {err}"),
    }
}

#[derive(Debug)]
enum PartialType {
    Sequence,
    StructField(usize),
    EnumField(usize, usize),
}

// -------------------------------------------------------------------------------------------------

fn deserialize_primitive<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    counters: &mut Vec<(PartialType, usize)>,
    flags: (&mut bool, &mut bool),
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
                    Ok(partial) => handle_item(partial, rem, flags, counters),
                    Err(_err) => todo!(),
                },
                Err(_err) => todo!(),
            }
        };
        ($deserialize_fn:ident, $($map_fn:tt)+) => {
            match de.$deserialize_fn(input).map($($map_fn)+) {
                Ok((val, rem)) => match current.set(val) {
                    Ok(partial) => handle_item(partial, rem, flags, counters),
                    Err(_err) => todo!(),
                },
                Err(_err) => todo!(),
            }
        };
    }

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
        Some(ScalarType::String) => {
            deserialize_scalar!(deserialize_str, |(s, r)| (s.to_string(), r))
        }
        Some(ScalarType::CowStr) => {
            deserialize_scalar!(deserialize_str, |(s, r)| (Cow::Borrowed(s), r))
        }
        Some(..) => todo!(),
        None => todo!(),
    }
}

fn deserialize_var_primitive<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    counters: &mut Vec<(PartialType, usize)>,
    flags: (&mut bool, &mut bool),
    input: &'input [u8],
    de: &mut D,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
>
where
    'input: 'partial + 'facet,
{
    macro_rules! deserialize_var_scalar {
        ($deserialize_fn:ident) => {
            match de.$deserialize_fn(input) {
                Ok((val, rem)) => match current.set(val) {
                    Ok(partial) => handle_item(partial, rem, flags, counters),
                    Err(_err) => todo!(),
                },
                Err(_err) => todo!(),
            }
        };
    }

    match ScalarType::try_from_shape(current.shape()) {
        Some(ScalarType::U16) => deserialize_var_scalar!(deserialize_var_u16),
        Some(ScalarType::U32) => deserialize_var_scalar!(deserialize_var_u32),
        Some(ScalarType::U64) => deserialize_var_scalar!(deserialize_var_u64),
        Some(ScalarType::U128) => deserialize_var_scalar!(deserialize_var_u128),
        Some(ScalarType::USize) => deserialize_var_scalar!(deserialize_var_usize),
        Some(ScalarType::I16) => deserialize_var_scalar!(deserialize_var_i16),
        Some(ScalarType::I32) => deserialize_var_scalar!(deserialize_var_i32),
        Some(ScalarType::I64) => deserialize_var_scalar!(deserialize_var_i64),
        Some(ScalarType::I128) => deserialize_var_scalar!(deserialize_var_i128),
        Some(ScalarType::ISize) => deserialize_var_scalar!(deserialize_var_isize),
        Some(..) => todo!(),
        None => todo!(),
    }
}

// -------------------------------------------------------------------------------------------------

fn deserialize_sequence<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    current: &'partial mut Partial<'facet, 'shape>,
    counters: &mut Vec<(PartialType, usize)>,
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
            counters.push((PartialType::Sequence, other));

            match part.begin_list_item() {
                Ok(part) => Ok((part, input)),
                Err(_err) => todo!(),
            }
        }
    }
}

/// Handle an item that may be part of a list.
///
/// If the item is from a list, decrement the remaining item count.
/// If the count reaches zero, remove the list from the map.
fn handle_item<'input, 'partial, 'facet, 'shape>(
    mut partial: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    flags: (&mut bool, &mut bool),
    counters: &mut Vec<(PartialType, usize)>,
) -> Result<
    (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
    DeserializeError<'input, 'facet, 'shape>,
> {
    // Reset the field flags.
    *flags.0 = false;
    *flags.1 = false;

    if let Some((ty, n)) = counters.last_mut() {
        *n = n.saturating_sub(1);

        match partial.end() {
            Ok(part) => partial = part,
            Err(_err) => todo!(),
        }

        if *n == 0 {
            counters.pop();

            Ok((partial, input))
        } else {
            match ty {
                PartialType::Sequence => match partial.begin_list_item() {
                    Ok(part) => Ok((part, input)),
                    Err(_err) => todo!(),
                },
                PartialType::StructField(field_n) => {
                    let Type::User(UserType::Struct(ty)) = partial.shape().ty else { todo!() };
                    if let Some(field) = ty.fields.get(*field_n) {
                        // Mark the field to be variably deserialized.
                        if field.attributes.contains(VAR) {
                            *flags.0 = true;
                        }

                        // Mark the field to be deserialized as JSON.
                        #[cfg(feature = "json")]
                        if field.attributes.contains(JSON) {
                            *flags.1 = true;
                        }

                        match partial.begin_nth_field(*field_n) {
                            Ok(part) => {
                                field_n.add_assign(1);
                                Ok((part, input))
                            }
                            Err(_err) => todo!(),
                        }
                    } else {
                        todo!();
                    }
                }
                PartialType::EnumField(variant_n, field_n) => {
                    let Type::User(UserType::Enum(ty)) = partial.shape().ty else { todo!() };
                    if let Some(variant) = ty.variants.get(*variant_n)
                        && let Some(field) = variant.data.fields.get(*field_n)
                    {
                        // Mark the field to be variably deserialized.
                        if field.attributes.contains(VAR) {
                            *flags.0 = true;
                        }

                        // Mark the field to be deserialized as JSON.
                        #[cfg(feature = "json")]
                        if field.attributes.contains(JSON) {
                            *flags.1 = true;
                        }

                        match partial.begin_nth_enum_field(*field_n) {
                            Ok(part) => {
                                field_n.add_assign(1);
                                Ok((part, input))
                            }
                            Err(_err) => todo!(),
                        }
                    } else {
                        todo!()
                    }
                }
            }
        }
    } else {
        Ok((partial, input))
    }
}
