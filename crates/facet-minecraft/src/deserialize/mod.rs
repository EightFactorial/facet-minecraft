//! A deserializer for types implementing [`Facet`].
#![allow(clippy::result_large_err, reason = "Error is large if rich diagnostics are enabled")]

use alloc::{borrow::Cow, string::String, vec};
use core::fmt::Debug;

use facet_core::{
    Def, Facet, FieldAttribute, PointerType, PrimitiveType, SequenceType, StructType, Type,
    UserType,
};
use facet_reflect::{HeapValue, Partial, ReflectError, ScalarType};

mod error;
pub use error::{DeserError, DeserErrorKind};

mod traits;
pub use traits::Deserializer;

use crate::Standard;

/// Deserialize a value of type `T` from the input bytes.
///
/// # Errors
///
/// If the bytes do not represent a valid value of type `T`,
/// an error is returned.
#[cfg_attr(feature = "rich-diagnostics", track_caller)]
pub fn deserialize<'input: 'facet, 'facet, T: Facet<'facet>>(
    input: &'input [u8],
) -> Result<T, DeserError<'input>> {
    #[cfg(feature = "rich-diagnostics")]
    let location = core::panic::Location::caller();

    #[allow(unused_mut, reason = "Used when rich diagnostics are enabled")]
    deserialize_value::<T, Standard>(input, &mut Standard).map_or_else(
        |mut err| {
            #[cfg(feature = "rich-diagnostics")]
            {
                err = err.with_location(location);
            }
            Err(err)
        },
        |(value, _)| Ok(value),
    )
}

// -------------------------------------------------------------------------------------------------

/// Deserialize a value of type `T` from the input bytes using the given
/// deserializer, returning any remaining bytes after the value.
///
/// # Errors
///
/// If the bytes do not represent a valid value of type `T`,
/// an error is returned.
#[cfg_attr(feature = "rich-diagnostics", track_caller)]
pub fn deserialize_value<'input: 'facet, 'facet, T: Facet<'facet>, D: Deserializer>(
    input: &'input [u8],
    de: &mut D,
) -> Result<(T, &'input [u8]), DeserError<'input>> {
    #[cfg(feature = "rich-diagnostics")]
    let location = core::panic::Location::caller();

    // Allocate a partial value to hold the deserialized data.
    let mut partial = match Partial::alloc_shape(T::SHAPE) {
        Ok(partial) => partial,
        Err(err) => todo!("Cannot alloc shape: {err}"),
    };

    // Perform iterative deserialization.
    let (heap, rest) = deserialize_iterative::<D>(&mut partial, input, de).map_err(|mut err| {
        err = err.with_source(input);
        #[cfg(feature = "rich-diagnostics")]
        {
            err = err.with_location(location);
        }
        err
    })?;

    // Materialize the heap value.
    match heap.materialize() {
        Ok(value) => Ok((value, rest)),
        Err(err) => todo!("Cannot materialize: {err}"),
    }
}

/// A step in the iterative deserialization process.
#[derive(Clone, Copy)]
pub enum DeserStep {
    /// The root step.
    Root,
    /// A value, with (is_variable).
    Value(bool),
    /// An object, with (data, current).
    Object(StructType, usize),
    /// The end of the current object.
    ObjectEnd,
    /// An array, with (is_list, is_variable, max, current).
    Array(bool, bool, usize, usize),
    /// A map or set key, with (has_value, is_variable, max, current).
    MapKey(bool, bool, usize, usize),
    /// A map item, with (is_variable).
    ///
    /// Only meant to be used by [`DeserStep::MapKey`].
    MapItem(bool),
}

impl Debug for DeserStep {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Root => write!(f, "Root"),
            Self::Value(arg0) => f.debug_tuple("Value").field(arg0).finish(),
            Self::Object(arg0, arg1) => {
                f.debug_tuple("Object").field(&((*arg1)..arg0.fields.len())).finish()
            }
            Self::ObjectEnd => write!(f, "ObjectEnd"),
            Self::Array(arg0, arg1, arg2, arg3) => {
                f.debug_tuple("Array").field(arg0).field(arg1).field(arg2).field(arg3).finish()
            }
            Self::MapKey(arg0, arg1, arg2, arg3) => {
                f.debug_tuple("MapKey").field(arg0).field(arg1).field(arg2).field(arg3).finish()
            }
            Self::MapItem(arg0) => f.debug_tuple("MapItem").field(arg0).finish(),
        }
    }
}

#[expect(clippy::too_many_lines, reason = "Complicated, iterative deserializer")]
fn deserialize_iterative<'input, D: Deserializer>(
    mut partial: &mut Partial<'input>,
    input: &'input [u8],
    de: &mut D,
) -> Result<(HeapValue<'input>, &'input [u8]), DeserError<'input>> {
    let mut cursor = input;

    #[cfg(feature = "trace")]
    let mut step_count = 0usize;
    // Note: The outer shape can never be variably-sized, only fields.
    let mut instructions = vec![DeserStep::Root, DeserStep::Value(false)];

    loop {
        let Some(step) = instructions.pop() else {
            unreachable!("There should never be no instructions remaining");
        };

        #[cfg(feature = "trace")]
        {
            tracing::trace!(
                "Deserialize `{}`: {step:?} ({step_count}) ({} bytes remaining)",
                partial.shape().type_identifier,
                cursor.len()
            );
            step_count += 1;
        }

        match step {
            DeserStep::Root => break,
            DeserStep::Value(is_variable) => {
                #[cfg(feature = "custom")]
                if partial
                    .shape()
                    .attributes
                    .contains(&facet_core::ShapeAttribute::Arbitrary("custom"))
                    && let Some(custom) =
                        crate::custom::DeserializerFn::for_type_id(partial.shape().id)
                {
                    let (part, rest) = custom.run(partial, cursor, &mut instructions)?;
                    partial = part;
                    cursor = rest;
                    continue;
                }

                if let Some(scalar_type) = ScalarType::try_from_shape(partial.shape()) {
                    // Handle scalar types directly.
                    match scalar_type {
                        ScalarType::Unit => {
                            cursor = de.deserialize_unit(input)?;
                            let path = partial.path();
                            partial = partial
                                .set(())
                                .map_err(|err| map_deser::<()>(err, path, cursor, input))?;
                        }
                        ScalarType::Bool => {
                            let (val, rest) = de.deserialize_bool(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<bool>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::Str => {
                            let (val, rest) = de.deserialize_str(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<str>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::String => {
                            let (val, rest) = de.deserialize_str(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(String::from(val))
                                .map_err(|err| map_deser::<String>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::CowStr => {
                            let (val, rest) = de.deserialize_str(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(Cow::from(val))
                                .map_err(|err| map_deser::<Cow<str>>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::F32 => {
                            let (val, rest) = de.deserialize_f32(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<f32>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::F64 => {
                            let (val, rest) = de.deserialize_f64(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<f64>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::U8 => {
                            let (val, rest) = de.deserialize_u8(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<u8>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::U16 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_u16(cursor)
                            } else {
                                de.deserialize_u16(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<u16>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::U32 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_u32(cursor)
                            } else {
                                de.deserialize_u32(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<u32>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::U64 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_u64(cursor)
                            } else {
                                de.deserialize_u64(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<u64>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::U128 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_u128(cursor)
                            } else {
                                de.deserialize_u128(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<u128>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::USize => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_usize(cursor)
                            } else {
                                de.deserialize_usize(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<usize>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::I8 => {
                            let (val, rest) = de.deserialize_i8(cursor)?;
                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<i8>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::I16 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_i16(cursor)
                            } else {
                                de.deserialize_i16(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<i16>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::I32 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_i32(cursor)
                            } else {
                                de.deserialize_i32(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<i32>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::I64 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_i64(cursor)
                            } else {
                                de.deserialize_i64(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<i64>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::I128 => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_i128(cursor)
                            } else {
                                de.deserialize_i128(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<i128>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::ISize => {
                            let (val, rest) = if is_variable {
                                de.deserialize_var_isize(cursor)
                            } else {
                                de.deserialize_isize(cursor)
                            }?;

                            let path = partial.path();
                            partial = partial
                                .set(val)
                                .map_err(|err| map_deser::<isize>(err, path, cursor, input))?;
                            cursor = rest;
                        }
                        ScalarType::Char => todo!("Return error on `char`"),
                        ScalarType::SocketAddr => todo!("Return error on `SocketAddr`"),
                        ScalarType::IpAddr => todo!("Return error on `IpAddr`"),
                        ScalarType::Ipv4Addr => todo!("Return error on `Ipv4Addr`"),
                        ScalarType::Ipv6Addr => todo!("Return error on `Ipv6Addr`"),
                        ScalarType::ConstTypeId => todo!("Return error on `ConstTypeId`"),
                    }
                } else {
                    // Otherwise, determine the next steps based on the definition.
                    match partial.shape().def {
                        Def::Array(array) => {
                            instructions.push(DeserStep::Array(false, is_variable, array.n, 0));
                        }
                        Def::List(_) | Def::Slice(_) => {
                            let (length, rest) = de.deserialize_var_usize(cursor)?;
                            instructions.push(DeserStep::Array(true, is_variable, length, 0));
                            cursor = rest;
                        }
                        Def::Map(_) => {
                            let (length, rest) = de.deserialize_var_usize(cursor)?;
                            instructions.push(DeserStep::MapKey(true, is_variable, length, 0));
                            cursor = rest;
                        }
                        Def::Set(_) => {
                            let (length, rest) = de.deserialize_var_usize(cursor)?;
                            instructions.push(DeserStep::MapKey(false, is_variable, length, 0));
                            cursor = rest;
                        }
                        Def::Option(_) => {
                            let (is_some, rest) = de.deserialize_bool(cursor)?;
                            cursor = rest;

                            let type_identifier = partial.shape().type_identifier;
                            let path = partial.path();
                            if is_some {
                                // Begin the `Some` variant.
                                partial = partial.begin_some().map_err(|err| {
                                    map_deser_using(err, type_identifier, path, cursor, input)
                                })?;
                                instructions.push(DeserStep::ObjectEnd);
                                instructions.push(DeserStep::Value(is_variable));
                            } else {
                                // Set the `None` variant.
                                partial = partial.set_default().map_err(|err| {
                                    map_deser_using(err, type_identifier, path, cursor, input)
                                })?;
                            }
                        }

                        Def::Scalar =>
                        {
                            #[cfg(feature = "uuid")]
                            if partial.shape().is_type::<uuid::Uuid>() {
                                let (val, rest) = de
                                    .deserialize_u128(cursor)
                                    .map_err(DeserError::with_type_name::<uuid::Uuid>)?;

                                let path = partial.path();
                                partial =
                                    partial.set(uuid::Uuid::from_u128(val)).map_err(|err| {
                                        map_deser::<uuid::Uuid>(err, path, cursor, input)
                                    })?;
                                cursor = rest;
                            }
                        }

                        // If the definition is undefined, determine based on the shape type.
                        Def::Undefined => match partial.shape().ty {
                            Type::Sequence(SequenceType::Array(array)) => {
                                instructions.push(DeserStep::Array(false, is_variable, array.n, 0));
                            }
                            Type::Sequence(SequenceType::Slice(_)) => {
                                let (length, rest) = de.deserialize_var_usize(cursor)?;
                                instructions.push(DeserStep::Array(false, is_variable, length, 0));
                                cursor = rest;
                            }
                            Type::Pointer(PointerType::Reference(_reference)) => todo!(),
                            Type::User(UserType::Struct(struct_type)) => {
                                instructions.push(DeserStep::Object(struct_type, 0));
                            }
                            Type::User(UserType::Enum(enum_type)) => {
                                let (variant, rest) = de.deserialize_var_i64(cursor)?;

                                let type_identifier = partial.shape().type_identifier;
                                let path = partial.path();
                                partial = partial.select_variant(variant).map_err(|err| {
                                    map_deser_using(err, type_identifier, path, cursor, input)
                                })?;
                                cursor = rest;

                                let variant = enum_type
                                    .variants
                                    .iter()
                                    .find(|v| v.discriminant.is_some_and(|d| d == variant))
                                    .unwrap();

                                // Only push an object step if there are fields to deserialize.
                                if !variant.data.fields.is_empty() {
                                    instructions.push(DeserStep::Object(variant.data, 0));
                                }
                            }
                            Type::User(UserType::Opaque) => todo!("Return error on `Opaque`"),
                            Type::Primitive(PrimitiveType::Never) => {
                                todo!("Return error on `Never`")
                            }
                            Type::Pointer(PointerType::Function(_)) => {
                                todo!("Return error on `FnPtr`")
                            }
                            Type::Pointer(PointerType::Raw(_)) => todo!("Return error on `Ptr`"),
                            Type::User(UserType::Union(_)) => todo!("Return error on `Union`"),
                            _ => unreachable!("All scalar types should have been handled above"),
                        },
                        Def::Pointer(ptr) => {
                            todo!("Return error on pointer: {ptr:?}");
                        }
                    }
                }
            }

            DeserStep::Object(data, current) => {
                if current < data.fields.len() {
                    // Begin the next field.
                    let type_identifier = partial.shape().type_identifier;
                    let path = partial.path();
                    partial = partial.begin_nth_field(current).map_err(|err| {
                        map_deser_using(err, type_identifier, path, cursor, input)
                    })?;

                    let is_variable =
                        data.fields[current].attributes.contains(&FieldAttribute::Arbitrary("var"));

                    instructions.push(DeserStep::Object(data, current + 1));
                    instructions.push(DeserStep::ObjectEnd);
                    instructions.push(DeserStep::Value(is_variable));
                }
            }
            DeserStep::ObjectEnd => {
                let type_identifier = partial.shape().type_identifier;
                let path = partial.path();
                partial = partial
                    .end()
                    .map_err(|err| map_deser_using(err, type_identifier, path, cursor, input))?;
            }

            DeserStep::Array(is_list, is_variable, max, current) => {
                if is_list && current == 0 {
                    // Initialize the list.
                    let type_identifier = partial.shape().type_identifier;
                    let path = partial.path();
                    partial = partial.begin_list().map_err(|err| {
                        map_deser_using(err, type_identifier, path, cursor, input)
                    })?;
                }

                if current < max {
                    // Begin the next item.
                    let type_identifier = partial.shape().type_identifier;
                    let path = partial.path();
                    if is_list {
                        partial = partial.begin_list_item().map_err(|err| {
                            map_deser_using(err, type_identifier, path, cursor, input)
                        })?;
                    } else {
                        partial = partial.begin_nth_field(current).map_err(|err| {
                            map_deser_using(err, type_identifier, path, cursor, input)
                        })?;
                    }

                    instructions.push(DeserStep::Array(is_list, is_variable, max, current + 1));
                    instructions.push(DeserStep::ObjectEnd);
                    instructions.push(DeserStep::Value(is_variable));
                }
            }

            #[rustfmt::skip]
            DeserStep::MapKey(has_value, is_variable, max, current) => {

                // Begin the map.
                if current == 0 && has_value {
                    let type_identifier = partial.shape().type_identifier;
                    let path = partial.path();
                    partial = partial
                        .begin_map()
                        .map_err(|err| map_deser_using(err, type_identifier, path, cursor, input))?;
                }

                // If we've reached the end, don't begin another key-value pair.
                if current >= max { continue }


                // Begin the next key.
                let type_identifier = partial.shape().type_identifier;
                let path = partial.path();
                partial = partial
                    .begin_key()
                    .map_err(|err| map_deser_using(err, type_identifier, path, cursor, input))?;

                instructions.push(DeserStep::MapKey(has_value, is_variable, max, current + 1));
                if has_value {
                    instructions.push(DeserStep::MapItem(is_variable));
                }

                // Note: The key is never variably-sized.
                instructions.push(DeserStep::ObjectEnd);
                instructions.push(DeserStep::Value(false));
            }
            DeserStep::MapItem(is_variable) => {
                // Begin the value.
                let type_identifier = partial.shape().type_identifier;
                let path = partial.path();
                partial = partial
                    .begin_value()
                    .map_err(|err| map_deser_using(err, type_identifier, path, cursor, input))?;

                instructions.push(DeserStep::ObjectEnd);
                instructions.push(DeserStep::Value(is_variable));
            }
        }
    }

    // Ensure we are at the root partial.
    while partial.frame_count() > 1 {
        let type_identifier = partial.shape().type_identifier;
        let path = partial.path();
        partial = partial
            .end()
            .map_err(|err| map_deser_using(err, type_identifier, path, cursor, input))?;
    }

    // Attempt to build the final value.
    let path = partial.path();
    match partial.build() {
        Ok(value) => Ok((value, cursor)),
        Err(err) => Err(map_deser_using(err, partial.shape().type_identifier, path, cursor, input)),
    }
}

/// Map a [`ReflectError`] into a [`DeserError`].
#[inline]
fn map_deser<'input, T: ?Sized>(
    err: ReflectError,
    path: String,
    cursor: &'input [u8],
    input: &'input [u8],
) -> DeserError<'input> {
    map_deser_using(err, core::any::type_name::<T>(), path, cursor, input)
}

/// Map a [`ReflectError`] into a [`DeserError`].
#[allow(unused_mut, unused_variables, reason = "Used when rich diagnostics are enabled")]
fn map_deser_using<'input>(
    err: ReflectError,
    type_name: &'static str,
    path: String,
    cursor: &'input [u8],
    input: &'input [u8],
) -> DeserError<'input> {
    let mut error = DeserError::new_using(
        cursor,
        type_name,
        DeserErrorKind::Reflect(err),
        (input.len() - cursor.len()).saturating_sub(1)
            ..(input.len() - cursor.len()).saturating_sub(1),
    );

    #[cfg(feature = "rich-diagnostics")]
    {
        error = error.with_help(alloc::format!("Failed while deserializing field: `{path}`"));
    }

    error
}
