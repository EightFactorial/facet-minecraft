use alloc::{borrow::Cow, string::ToString, vec};

#[cfg(feature = "custom")]
use facet::ShapeAttribute;
use facet::{Def, FieldAttribute, Shape, Type};
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

    let mut stack = vec![DeserializationTask::Object(&mut partial)];

    while let Some(task) = stack.pop() {
        match task {
            DeserializationTask::Object(mut partial) => {
                // Use the inner type if the shape has the `transparent` attribute.
                if partial.shape().attributes.contains(&ShapeAttribute::Transparent) {
                    partial = partial.begin_inner().unwrap();
                }

                // If the shape has a `custom` attribute,
                // check for a custom serialization function.
                #[cfg(feature = "custom")]
                #[allow(clippy::collapsible_if)]
                if partial.shape().attributes.contains(CUSTOM) {
                    if let Some(custom) = overrides.iter().find(|o| o.id == partial.shape().id) {
                        if let Some(de) = custom.deserialize {
                            de(input, &mut stack);
                            continue;
                        }
                    }
                }

                match partial.shape().def {
                    Def::Scalar => match ScalarType::try_from_shape(partial.shape()) {
                        Some(ScalarType::Unit) => match de.deserialize_unit(input) {
                            Ok(((), remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(()) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::Bool) => match de.deserialize_bool(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::Str) => match de.deserialize_str(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::String) => match de.deserialize_str(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value.to_string()) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::CowStr) => match de.deserialize_str(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(Cow::Borrowed(value)) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::F32) => match de.deserialize_f32(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::F64) => match de.deserialize_f64(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::U8) => match de.deserialize_u8(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::U16) => match de.deserialize_u16(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::U32) => match de.deserialize_u32(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::U64) => match de.deserialize_u64(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::U128) => match de.deserialize_u128(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::USize) => match de.deserialize_usize(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::I8) => match de.deserialize_i8(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::I16) => match de.deserialize_i16(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::I32) => match de.deserialize_i32(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::I64) => match de.deserialize_i64(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::I128) => match de.deserialize_i128(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(ScalarType::ISize) => match de.deserialize_isize(input) {
                            Ok((value, remainder)) => {
                                input = remainder;
                                if let Err(err) = partial.set(value) {
                                    todo!();
                                }
                            }
                            Err(err) => todo!(),
                        },
                        Some(..) => todo!(),
                        None => todo!(),
                    },
                    Def::Map(_def) => todo!(),
                    Def::Set(_def) => todo!(),
                    Def::List(_def) => todo!(),
                    Def::Array(_def) => todo!(),
                    Def::Slice(_def) => todo!(),
                    Def::Option(_def) => todo!(),
                    Def::SmartPointer(_def) => todo!(),
                    Def::Undefined => todo!(),
                }
            }
            DeserializationTask::FieldVariable(mut partial) => {
                // Use the inner type if the shape has the `transparent` attribute.
                if partial.shape().attributes.contains(&ShapeAttribute::Transparent) {
                    partial = partial.begin_inner().unwrap();
                }

                match partial.shape().ty {
                    Type::Primitive(ty) => todo!(),
                    Type::Sequence(ty) => todo!(),
                    Type::User(ty) => todo!(),
                    Type::Pointer(ty) => todo!(),
                }
            }
            DeserializationTask::Skip(len) => {
                if let Some((_, remainder)) = input.split_at_checked(len) {
                    input = remainder;
                } else {
                    todo!();
                }
            }
        }
    }

    match partial.build() {
        Ok(heap) => Ok((heap, input)),
        Err(_err) => todo!(),
    }
}

/// A task to be performed during deserialization.
#[expect(missing_docs)]
pub enum DeserializationTask<'stack, 'facet, 'shape> {
    Object(&'stack mut Partial<'facet, 'shape>),
    FieldVariable(&'stack mut Partial<'facet, 'shape>),
    Skip(usize),
}
