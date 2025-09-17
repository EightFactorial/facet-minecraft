//! A serializer for types implementing [`Facet`].
#![allow(clippy::result_large_err, reason = "Error is large if rich diagnostics are enabled")]

use alloc::{borrow::Cow, boxed::Box, vec};
use core::fmt::Debug;

#[cfg(feature = "custom")]
use facet_core::ShapeAttribute;
use facet_core::{
    Def, Facet, FieldAttribute, PointerType, PrimitiveType, SequenceType, Type, UserType,
};
use facet_reflect::{
    FieldsForSerializeIter, HasFields, Peek, PeekListLikeIter, PeekMapIter, PeekSetIter, ScalarType,
};

mod error;
pub use error::{SerError, SerErrorKind};

mod traits;
pub use traits::{Peekable, Serializer, SliceCursor, SliceFullError, Writer};

use crate::Standard;

/// Serialize a value of type `T` into a series of bytes.
///
/// # Errors
///
/// If a field of `T` cannot be serialized,
/// or if the provided writer returns an error.
#[cfg_attr(feature = "rich-diagnostics", track_caller)]
pub fn serialize<'input, T: Facet<'input>, W: Writer>(
    value: &'input T,
    writer: &mut W,
) -> Result<(), SerError<'input, W>> {
    #[cfg(feature = "rich-diagnostics")]
    let location = core::panic::Location::caller();

    #[allow(clippy::map_identity, unused_mut, reason = "Used when rich diagnostics are enabled")]
    serialize_value::<T, Standard, W>(value, &mut Standard, writer).map_err(|mut err| {
        #[cfg(feature = "rich-diagnostics")]
        {
            err = err.with_location(location);
        }
        err
    })
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value of type `T` into a series of bytes using the given
/// serializer.
///
/// # Errors
///
/// If a field of `T` cannot be serialized,
/// or if the provided writer returns an error.
#[cfg_attr(feature = "rich-diagnostics", track_caller)]
pub fn serialize_value<'input, T: Facet<'input>, S: Serializer, W: Writer>(
    value: &'input T,
    serializer: &mut S,
    writer: &mut W,
) -> Result<(), SerError<'input, W>> {
    #[cfg(feature = "rich-diagnostics")]
    let location = core::panic::Location::caller();

    let peek = Peek::new(value);
    #[allow(clippy::map_identity, unused_mut, reason = "Used when rich diagnostics are enabled")]
    serialize_iterative(peek, serializer, writer).map_err(|mut err| {
        #[cfg(feature = "rich-diagnostics")]
        {
            err = err.with_location(location);
        }
        err
    })?;

    Ok(())
}

/// A step in the iterative deserialization process.
pub enum SerStep<'input, 'facet> {
    /// A byte array to append directly.
    Append(Cow<'input, [u8]>),
    /// Return an error directly.
    Error(Box<dyn core::error::Error + Send + Sync>),

    /// A value, with (value, is_variable)
    Value(Peek<'input, 'facet>, bool),
    /// A boxed value
    ValueBoxed(Box<dyn Peekable<'static>>),
    /// An object field iterator
    Object(FieldsForSerializeIter<'input, 'facet>),
    /// A list iterator, with (iter, is_variable)
    List(PeekListLikeIter<'input, 'facet>, bool),
    /// A map iterator, with (iter, is_variable)
    Map(PeekMapIter<'input, 'facet>, bool),
    /// A set iterator, with (iter, is_variable)
    Set(PeekSetIter<'input, 'facet>, bool),
}

impl Debug for SerStep<'_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Append(arg0) => f.debug_tuple("Append").field(arg0).finish(),
            Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::Value(arg0, arg1) => f.debug_tuple("Value").field(arg0).field(arg1).finish(),
            Self::ValueBoxed(_) => f.debug_tuple("ValueBoxed").finish_non_exhaustive(),
            Self::Object(_) => f.debug_tuple("Object").finish_non_exhaustive(),
            Self::List(_, arg1) => f.debug_tuple("List").field(arg1).finish_non_exhaustive(),
            Self::Map(_, arg1) => f.debug_tuple("Map").field(arg1).finish_non_exhaustive(),
            Self::Set(_, arg1) => f.debug_tuple("Set").field(arg1).finish_non_exhaustive(),
        }
    }
}

#[allow(clippy::too_many_lines, reason = "Complicated, iterative serializer")]
fn serialize_iterative<'input, S: Serializer, W: Writer>(
    peek: Peek<'input, '_>,
    ser: &mut S,
    writer: &mut W,
) -> Result<(), SerError<'input, W>> {
    #[cfg(feature = "trace")]
    let mut step_count = 0usize;
    let mut instructions = vec![SerStep::Value(peek, false)]; // TODO: is_variable

    while let Some(step) = instructions.pop() {
        #[cfg(feature = "trace")]
        if !matches!(step, SerStep::Value(..) | SerStep::ValueBoxed(..)) {
            tracing::trace!("Serialize: {step:?} ({step_count})");
            step_count += 1;
        }

        match step {
            SerStep::Append(bytes) => writer.write(&bytes).unwrap(), // TODO: Handle errors
            SerStep::Error(err) => {
                // TODO: Handle errors
                let type_identifier = peek.shape().type_identifier;
                return Err(SerError::new_using(
                    &[],
                    type_identifier,
                    SerErrorKind::Other(err),
                    0..0,
                ));
            }

            SerStep::ValueBoxed(peekable) => {
                #[cfg(feature = "trace")]
                {
                    tracing::trace!(
                        "Serialize `Box<{}>`: ValueBoxed(..) ({step_count})",
                        peekable.peek().shape().type_identifier
                    );
                    step_count += 1;
                }

                serialize_iterative::<S, W>(peekable.peek(), ser, writer)
                    .map_err(SerError::into_owned)?;
            }

            SerStep::Value(peek, is_variable) => {
                #[cfg(feature = "trace")]
                {
                    tracing::trace!(
                        "Serialize `{}`: Value(..) ({step_count})",
                        peek.shape().type_identifier,
                    );
                    step_count += 1;
                }

                #[cfg(feature = "custom")]
                if peek.shape().attributes.contains(&ShapeAttribute::Arbitrary("custom"))
                    && let Some(custom) = crate::custom::SerializerFn::for_type_id(peek.shape().id)
                {
                    custom.run(peek, is_variable, &mut instructions);
                    continue;
                }

                if let Some(scalar_type) = ScalarType::try_from_shape(peek.shape()) {
                    // Handle scalar types directly.
                    match scalar_type {
                        ScalarType::Unit => {
                            ser.serialize_unit((), writer)?;
                        }
                        ScalarType::Bool => {
                            let val = *peek.get::<bool>().unwrap(); // TODO: Handle errors
                            ser.serialize_bool(val, writer)?;
                        }
                        ScalarType::Str | ScalarType::String | ScalarType::CowStr => {
                            let str = peek.as_str().expect("`str`-like scalar should be `str`!?");
                            ser.serialize_str(str, writer)?;
                        }
                        ScalarType::F32 => {
                            let val = *peek.get::<f32>().unwrap(); // TODO: Handle errors
                            ser.serialize_f32(val, writer)?;
                        }
                        ScalarType::F64 => {
                            let val = *peek.get::<f64>().unwrap(); // TODO: Handle errors
                            ser.serialize_f64(val, writer)?;
                        }
                        ScalarType::U8 => {
                            let val = *peek.get::<u8>().unwrap(); // TODO: Handle errors
                            ser.serialize_u8(val, writer)?;
                        }
                        ScalarType::U16 => {
                            let val = *peek.get::<u16>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_u16(val, writer)?;
                            } else {
                                ser.serialize_u16(val, writer)?;
                            }
                        }
                        ScalarType::U32 => {
                            let val = *peek.get::<u32>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_u32(val, writer)?;
                            } else {
                                ser.serialize_u32(val, writer)?;
                            }
                        }
                        ScalarType::U64 => {
                            let val = *peek.get::<u64>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_u64(val, writer)?;
                            } else {
                                ser.serialize_u64(val, writer)?;
                            }
                        }
                        ScalarType::U128 => {
                            let val = *peek.get::<u128>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_u128(val, writer)?;
                            } else {
                                ser.serialize_u128(val, writer)?;
                            }
                        }
                        ScalarType::USize => {
                            let val = *peek.get::<usize>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_usize(val, writer)?;
                            } else {
                                ser.serialize_usize(val, writer)?;
                            }
                        }
                        ScalarType::I8 => {
                            let val = *peek.get::<i8>().unwrap(); // TODO: Handle errors
                            ser.serialize_i8(val, writer)?;
                        }
                        ScalarType::I16 => {
                            let val = *peek.get::<i16>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_i16(val, writer)?;
                            } else {
                                ser.serialize_i16(val, writer)?;
                            }
                        }
                        ScalarType::I32 => {
                            let val = *peek.get::<i32>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_i32(val, writer)?;
                            } else {
                                ser.serialize_i32(val, writer)?;
                            }
                        }
                        ScalarType::I64 => {
                            let val = *peek.get::<i64>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_i64(val, writer)?;
                            } else {
                                ser.serialize_i64(val, writer)?;
                            }
                        }
                        ScalarType::I128 => {
                            let val = *peek.get::<i128>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_i128(val, writer)?;
                            } else {
                                ser.serialize_i128(val, writer)?;
                            }
                        }
                        ScalarType::ISize => {
                            let val = *peek.get::<isize>().unwrap(); // TODO: Handle errors
                            if is_variable {
                                ser.serialize_var_isize(val, writer)?;
                            } else {
                                ser.serialize_isize(val, writer)?;
                            }
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
                    match peek.shape().def {
                        Def::Array(_) => {
                            let list_like =
                                peek.into_list_like().expect("Array should be list-like");
                            instructions.push(SerStep::List(list_like.iter(), is_variable));
                        }
                        Def::List(_) | Def::Slice(_) => {
                            let list_like =
                                peek.into_list_like().expect("List/Slice should be list-like");
                            ser.serialize_var_usize(list_like.len(), writer)?;
                            instructions.push(SerStep::List(list_like.iter(), is_variable));
                        }
                        Def::Map(_) => {
                            let map = peek.into_map().expect("Map should be map-like");
                            ser.serialize_var_usize(map.len(), writer)?;
                            instructions.push(SerStep::Map(map.iter(), is_variable));
                        }
                        Def::Set(_) => {
                            let set = peek.into_set().expect("Set should be set-like");
                            ser.serialize_var_usize(set.len(), writer)?;
                            instructions.push(SerStep::Set(set.iter(), is_variable));
                        }
                        Def::Option(_option) => {
                            let option = peek.into_option().expect("Option should be option-like");
                            if let Some(value) = option.value() {
                                ser.serialize_bool(true, writer)?;
                                instructions.push(SerStep::Value(value, is_variable));
                            } else {
                                ser.serialize_bool(false, writer)?;
                            }
                        }

                        Def::Scalar =>
                        {
                            #[cfg(feature = "uuid")]
                            if let Ok(uuid) = peek.get::<uuid::Uuid>() {
                                ser.serialize_u128(uuid.as_u128(), writer)?;
                            }
                        }

                        // If the definition is undefined, determine based on the shape type.
                        Def::Undefined => match peek.shape().ty {
                            Type::Sequence(SequenceType::Array(_)) => {
                                let list_like =
                                    peek.into_list_like().expect("Array should be list-like");
                                instructions.push(SerStep::List(list_like.iter(), is_variable));
                            }
                            Type::Sequence(SequenceType::Slice(_)) => {
                                let list_like =
                                    peek.into_list_like().expect("Slice should be list-like");
                                ser.serialize_var_usize(list_like.len(), writer)?;
                                instructions.push(SerStep::List(list_like.iter(), is_variable));
                            }
                            Type::Pointer(PointerType::Reference(_reference)) => todo!(),
                            Type::User(UserType::Struct(_)) => {
                                let struct_like =
                                    peek.into_struct().expect("Struct should be struct-like");
                                instructions
                                    .push(SerStep::Object(struct_like.fields_for_serialize()));
                            }
                            Type::User(UserType::Enum(_)) => {
                                let enum_like = peek.into_enum().expect("Enum should be enum-like");
                                ser.serialize_var_i64(enum_like.discriminant(), writer)?;
                                instructions
                                    .push(SerStep::Object(enum_like.fields_for_serialize()));
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
                            todo!("Return error on `Pointer` ({ptr:?})")
                        }
                    }

                    // Otherwise, determine the next steps based on the type.
                }
            }

            SerStep::Object(mut iter) => {
                let Some((field, field_value)) = iter.next() else { continue };
                let is_variable = field.attributes.contains(&FieldAttribute::Arbitrary("var"));
                instructions.push(SerStep::Object(iter));
                instructions.push(SerStep::Value(field_value, is_variable));
            }
            SerStep::List(mut iter, is_variable) => {
                let Some(value) = iter.next() else { continue };
                instructions.push(SerStep::List(iter, is_variable));
                instructions.push(SerStep::Value(value, is_variable));
            }
            SerStep::Map(mut iter, is_variable) => {
                let Some((key, value)) = iter.next() else { continue };
                instructions.push(SerStep::Map(iter, is_variable));
                instructions.push(SerStep::Value(value, is_variable));
                // Note: The key is never variably-sized.
                instructions.push(SerStep::Value(key, false));
            }
            SerStep::Set(mut iter, is_variable) => {
                let Some(value) = iter.next() else { continue };
                instructions.push(SerStep::Set(iter, is_variable));
                instructions.push(SerStep::Value(value, is_variable));
            }
        }
    }

    Ok(())
}
