use alloc::{borrow::Cow, boxed::Box, string::String, vec, vec::Vec};
use core::ops::{Deref, DerefMut};

use facet::{Def, FieldAttribute, ShapeAttribute, StructKind, Type, UserType};
use facet_reflect::{
    FieldsForSerializeIter, HasFields, Peek, PeekListLikeIter, PeekMapIter, ScalarType,
};

#[cfg(feature = "custom")]
use crate::custom::FacetOverride;
use crate::{adapter::WriteAdapter, assert::AssertProtocol};

mod traits;
pub use traits::{OwnedPeek, Serializer, SerializerExt};

mod error;
pub use error::SerializeError;

/// Serialize a type to the given writer.
///
/// This is a wrapper around [`serialize_iterative`],
/// using [`McSerializer`] as the serializer.
///
/// # Errors
/// Returns an error if the serialization fails.
#[inline(always)]
#[expect(clippy::inline_always)]
pub fn serialize<'mem, 'facet, T, W>(
    value: &'mem T,
    writer: W,
) -> Result<(), SerializeError<'mem, 'facet, W::Error>>
where
    T: AssertProtocol<'facet>,
    W: WriteAdapter,
{
    McSerializer::<W>::serialize_into::<T>(value, writer)
}

// -------------------------------------------------------------------------------------------------

/// A serializer for Minecraft protocol data.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub struct McSerializer<W: WriteAdapter>(pub W);

impl<W: WriteAdapter> McSerializer<W> {
    /// Serialize a type to the given writer.
    ///
    /// This is a wrapper around [`serialize_iterative`],
    /// using [`McSerializer`] as the serializer.
    ///
    /// # Errors
    /// Returns an error if the serialization fails.
    #[inline(always)]
    #[expect(clippy::inline_always)]
    pub fn serialize_into<'mem, 'facet, T: AssertProtocol<'facet>>(
        value: &'mem T,
        writer: W,
    ) -> Result<(), SerializeError<'mem, 'facet, W::Error>> {
        Self::serialize::<T>(&mut Self(writer), value)
    }

    /// Serialize a type into this writer.
    ///
    /// This is a wrapper around [`serialize_iterative`],
    /// using [`McSerializer`] as the serializer.
    ///
    /// # Errors
    /// Returns an error if the serialization fails.
    #[inline(always)]
    #[expect(clippy::inline_always)]
    pub fn serialize<'mem, 'facet, T: AssertProtocol<'facet>>(
        &mut self,
        value: &'mem T,
    ) -> Result<(), SerializeError<'mem, 'facet, W::Error>> {
        let () = const { <T as AssertProtocol<'facet>>::ASSERT };

        serialize_iterative::<Self>(Peek::new(value), self)
    }
}

// -------------------------------------------------------------------------------------------------

/// Iteratively serialize a type to the given writer.
///
/// Avoids recursion to prevent depth issues with large structures.
///
/// # Errors
/// Returns an error if the serialization fails.
#[expect(clippy::missing_panics_doc, clippy::too_many_lines)]
pub fn serialize_iterative<'mem, 'facet, W: SerializerExt>(
    peek: Peek<'mem, 'facet>,
    writer: &mut W,
) -> Result<(), SerializeError<'mem, 'facet, W::Error>> {
    static VAR: &FieldAttribute = &FieldAttribute::Arbitrary("var");
    #[cfg(feature = "json")]
    static JSON: &FieldAttribute = &FieldAttribute::Arbitrary("json");

    #[cfg(feature = "custom")]
    static CUSTOM: &ShapeAttribute = &ShapeAttribute::Arbitrary("custom");
    #[cfg(feature = "custom")]
    let overrides = FacetOverride::global();

    // Initialize the stack with the initial value to serialize.
    let mut stack = vec![SerializationTask::Value(peek)];

    while let Some(task) = stack.pop() {
        match task {
            SerializationTask::Value(mut peek) => {
                // Use the inner type if the shape has the `transparent` attribute.
                if peek.shape().attributes.contains(&ShapeAttribute::Transparent) {
                    let inner = peek.into_struct().unwrap();
                    peek = inner.field(0).unwrap();
                }

                // If the shape has a `custom` attribute,
                // check for a custom serialization function.
                #[cfg(feature = "custom")]
                if peek.shape().attributes.contains(CUSTOM)
                    && let Some(custom) = overrides.iter().find(|o| o.id == peek.shape().id)
                    && let Some(ser) = custom.serialize
                {
                    ser(peek, &mut stack);
                    continue;
                }

                // Serialize the value based on its definition.
                match peek.shape().def {
                    Def::Scalar => match peek.scalar_type() {
                        Some(ScalarType::Unit) => writer.serialize_unit()?,
                        Some(ScalarType::Bool) => {
                            writer.serialize_bool(*peek.get::<bool>().unwrap())?;
                        }
                        Some(ScalarType::Str) => {
                            writer.serialize_str(peek.get::<&str>().unwrap())?;
                        }
                        Some(ScalarType::String) => {
                            writer.serialize_str(peek.get::<String>().unwrap())?;
                        }
                        Some(ScalarType::CowStr) => {
                            writer.serialize_str(peek.get::<Cow<'_, str>>().unwrap())?;
                        }
                        Some(ScalarType::F32) => {
                            writer.serialize_f32(*peek.get::<f32>().unwrap())?;
                        }
                        Some(ScalarType::F64) => {
                            writer.serialize_f64(*peek.get::<f64>().unwrap())?;
                        }
                        Some(ScalarType::U8) => writer.serialize_u8(*peek.get::<u8>().unwrap())?,
                        Some(ScalarType::U16) => {
                            writer.serialize_u16(*peek.get::<u16>().unwrap())?;
                        }
                        Some(ScalarType::U32) => {
                            writer.serialize_u32(*peek.get::<u32>().unwrap())?;
                        }
                        Some(ScalarType::U64) => {
                            writer.serialize_u64(*peek.get::<u64>().unwrap())?;
                        }
                        Some(ScalarType::U128) => {
                            writer.serialize_u128(*peek.get::<u128>().unwrap())?;
                        }
                        Some(ScalarType::USize) => {
                            writer.serialize_usize(*peek.get::<usize>().unwrap())?;
                        }
                        Some(ScalarType::I8) => writer.serialize_i8(*peek.get::<i8>().unwrap())?,
                        Some(ScalarType::I16) => {
                            writer.serialize_i16(*peek.get::<i16>().unwrap())?;
                        }
                        Some(ScalarType::I32) => {
                            writer.serialize_i32(*peek.get::<i32>().unwrap())?;
                        }
                        Some(ScalarType::I64) => {
                            writer.serialize_i64(*peek.get::<i64>().unwrap())?;
                        }
                        Some(ScalarType::I128) => {
                            writer.serialize_i128(*peek.get::<i128>().unwrap())?;
                        }
                        Some(ScalarType::ISize) => {
                            writer.serialize_isize(*peek.get::<isize>().unwrap())?;
                        }
                        _ => {
                            return Err(SerializeError::new(peek, "unsupported type"));
                        }
                    },
                    Def::Map(..) => {
                        let peek = peek.into_map().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(SerializationTask::Map(peek.iter(), false));
                    }
                    Def::List(..) | Def::Slice(..) => {
                        let peek = peek.into_list_like().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(SerializationTask::List(peek.iter(), false));
                    }
                    Def::Array(..) => {
                        let peek = peek.into_list_like().unwrap();
                        stack.push(SerializationTask::Array(peek.iter(), false));
                    }
                    Def::Option(..) => {
                        if let Some(value) = peek.into_option().unwrap().value() {
                            writer.serialize_bool(true)?;
                            stack.push(SerializationTask::Value(value));
                        } else {
                            writer.serialize_bool(false)?;
                        }
                    }
                    Def::Set(..) => {
                        return Err(SerializeError::new(peek, "sets are not supported yet"));
                    }
                    Def::Pointer(..) => {
                        let peek = peek.into_pointer().unwrap();
                        if let Some(inner) = peek.borrow_inner() {
                            stack.push(SerializationTask::Value(inner));
                        } else {
                            return Err(SerializeError::new_reason(
                                "smart pointer is not initialized",
                            ));
                        }
                    }
                    Def::Undefined => match peek.shape().ty {
                        #[expect(clippy::single_match_else)]
                        Type::User(UserType::Struct(ty)) => match ty.kind {
                            StructKind::Unit => writer.serialize_unit()?,
                            _ => {
                                let peek = peek.into_struct().unwrap();
                                stack.push(SerializationTask::Object(peek.fields_for_serialize()));
                            }
                        },
                        Type::User(UserType::Enum(..)) => {
                            let peek = peek.into_enum().unwrap();
                            let variant = peek.active_variant().unwrap();

                            let discriminant =
                                variant.discriminant.unwrap_or_else(|| peek.discriminant());
                            writer.serialize_var_i64(discriminant)?;

                            #[expect(clippy::single_match_else)]
                            match variant.data.kind {
                                StructKind::Unit => writer.serialize_unit()?,
                                _ => {
                                    // Serialize the fields in reverse order
                                    let fields: Vec<_> = peek.fields_for_serialize().collect();
                                    for (field, peek) in fields.into_iter().rev() {
                                        // Check if the field has the `var` attribute
                                        if field.attributes.contains(VAR) {
                                            stack.push(SerializationTask::ValueVariable(peek));
                                            continue;
                                        }

                                        // Check if the field has the `json` attribute
                                        #[cfg(feature = "json")]
                                        if field.attributes.contains(JSON) {
                                            stack.push(SerializationTask::ValueJson(peek));
                                            continue;
                                        }

                                        stack.push(SerializationTask::Value(peek));
                                    }
                                }
                            }
                        }
                        Type::Pointer(..) => {
                            if let Some(str) = peek.as_str() {
                                writer.serialize_str(str)?;
                            } else if let Some(bytes) = peek.as_bytes() {
                                writer.serialize_bytes(bytes)?;
                            }
                        }
                        _ => {}
                    },
                }
            }
            SerializationTask::ValueVariable(mut peek) => {
                // Use the inner type if the shape has the `transparent` attribute.
                if peek.shape().attributes.contains(&ShapeAttribute::Transparent) {
                    let inner = peek.into_struct().unwrap();
                    peek = inner.field(0).unwrap();
                }

                // Serialize the value based on its definition.
                match peek.shape().def {
                    Def::Scalar => match peek.scalar_type() {
                        Some(ScalarType::U16) => {
                            writer.serialize_var_u16(*peek.get::<u16>().unwrap())?;
                        }
                        Some(ScalarType::U32) => {
                            writer.serialize_var_u32(*peek.get::<u32>().unwrap())?;
                        }
                        Some(ScalarType::U64) => {
                            writer.serialize_var_u64(*peek.get::<u64>().unwrap())?;
                        }
                        Some(ScalarType::U128) => {
                            writer.serialize_var_u128(*peek.get::<u128>().unwrap())?;
                        }
                        Some(ScalarType::USize) => {
                            writer.serialize_var_usize(*peek.get::<usize>().unwrap())?;
                        }
                        Some(ScalarType::I16) => {
                            writer.serialize_var_i16(*peek.get::<i16>().unwrap())?;
                        }
                        Some(ScalarType::I32) => {
                            writer.serialize_var_i32(*peek.get::<i32>().unwrap())?;
                        }
                        Some(ScalarType::I64) => {
                            writer.serialize_var_i64(*peek.get::<i64>().unwrap())?;
                        }
                        Some(ScalarType::I128) => {
                            writer.serialize_var_i128(*peek.get::<i128>().unwrap())?;
                        }
                        Some(ScalarType::ISize) => {
                            writer.serialize_var_isize(*peek.get::<isize>().unwrap())?;
                        }
                        _ => {
                            return Err(SerializeError::new(
                                peek,
                                "type does not support variable-length serialization",
                            ));
                        }
                    },
                    Def::Option(..) => {
                        if let Some(value) = peek.into_option().unwrap().value() {
                            writer.serialize_bool(true)?;
                            stack.push(SerializationTask::ValueVariable(value));
                        } else {
                            writer.serialize_bool(false)?;
                        }
                    }
                    Def::Map(..) => {
                        let peek = peek.into_map().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(SerializationTask::Map(peek.iter(), true));
                    }
                    Def::List(..) | Def::Slice(..) => {
                        let peek = peek.into_list_like().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(SerializationTask::List(peek.iter(), true));
                    }
                    Def::Array(..) => {
                        let peek = peek.into_list_like().unwrap();
                        stack.push(SerializationTask::Array(peek.iter(), true));
                    }
                    _ => {
                        return Err(SerializeError::new(
                            peek,
                            "type does not support variable-length serialization",
                        ));
                    }
                }
            }
            // TODO: Avoid recursion here if possible.
            SerializationTask::ValueOwned(owned) => {
                if let Err(err) = serialize_iterative(owned.peek(), writer) {
                    return Err(err.into_owned());
                }
            }
            #[cfg(feature = "json")]
            SerializationTask::ValueJson(peek) => {
                writer.serialize_str(&facet_json::peek_to_string(peek))?;
            }
            SerializationTask::Object(mut peek) => {
                let Some((field, value)) = peek.next() else { continue };
                stack.push(SerializationTask::Object(peek));

                // Check if the field has the `var` attribute
                if field.attributes.contains(VAR) {
                    stack.push(SerializationTask::ValueVariable(value));
                    continue;
                }

                // Check if the field has the `json` attribute
                #[cfg(feature = "json")]
                if field.attributes.contains(JSON) {
                    stack.push(SerializationTask::ValueJson(value));
                    continue;
                }

                stack.push(SerializationTask::Value(value));
            }
            SerializationTask::List(mut peek, var) => {
                let Some(entry) = peek.next() else { continue };
                stack.push(SerializationTask::List(peek, var));
                if var {
                    stack.push(SerializationTask::ValueVariable(entry));
                } else {
                    stack.push(SerializationTask::Value(entry));
                }
            }
            SerializationTask::Array(mut peek, var) => {
                let Some(entry) = peek.next() else { continue };
                stack.push(SerializationTask::Array(peek, var));
                if var {
                    stack.push(SerializationTask::ValueVariable(entry));
                } else {
                    stack.push(SerializationTask::Value(entry));
                }
            }
            SerializationTask::Map(mut peek, var) => {
                let Some((key, value)) = peek.next() else { continue };
                stack.push(SerializationTask::Map(peek, var));
                if var {
                    stack.push(SerializationTask::ValueVariable(value));
                } else {
                    stack.push(SerializationTask::Value(value));
                }
                stack.push(SerializationTask::Value(key));
            }
        }
    }

    Ok(())
}

/// A task to be performed during serialization.
#[expect(missing_docs)]
pub enum SerializationTask<'mem, 'facet> {
    Value(Peek<'mem, 'facet>),
    ValueOwned(Box<dyn OwnedPeek<'facet>>),
    ValueVariable(Peek<'mem, 'facet>),
    #[cfg(feature = "json")]
    ValueJson(Peek<'mem, 'facet>),
    Object(FieldsForSerializeIter<'mem, 'facet>),
    Array(PeekListLikeIter<'mem, 'facet>, bool),
    List(PeekListLikeIter<'mem, 'facet>, bool),
    Map(PeekMapIter<'mem, 'facet>, bool),
}

// -------------------------------------------------------------------------------------------------

impl<W: WriteAdapter> Deref for McSerializer<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<W: WriteAdapter> DerefMut for McSerializer<W> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
