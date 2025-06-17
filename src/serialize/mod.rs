use alloc::{borrow::Cow, string::String, vec::Vec};
use core::ops::{Deref, DerefMut};

use facet::{Def, FieldAttribute, ShapeAttribute, StructKind, Type, UserType};
use facet_reflect::{
    FieldsForSerializeIter, HasFields, Peek, PeekListLikeIter, PeekMapIter, ScalarType,
};

use crate::{adapter::WriteAdapter, assert::AssertProtocol};

mod traits;
pub use traits::{Serializer, SerializerExt};

/// A serializer for Minecraft protocol data.
#[derive(Debug, Default, Clone, Copy)]
pub struct McSerializer<W: WriteAdapter>(pub W);

/// Serialize a type to the given writer.
///
/// # Errors
/// Returns an error if the serialization fails.
#[inline]
pub fn serialize<'mem, 'facet, T, W>(value: &'mem T, writer: W) -> Result<(), W::Error>
where
    'mem: 'facet,
    'facet: 'mem,
    T: AssertProtocol<'facet>,
    W: WriteAdapter,
{
    <T as AssertProtocol<'facet>>::assert();
    serialize_iterative(Peek::new(value), McSerializer(writer))
}

// -------------------------------------------------------------------------------------------------

/// Iteratively serialize a type to the given writer.
///
/// Avoids recursion to prevent depth issues with large structures.
///
/// # Errors
/// Returns an error if the serialization fails.
#[expect(clippy::missing_panics_doc, clippy::too_many_lines)]
pub fn serialize_iterative<'mem, 'facet, 'shape, W: SerializerExt<'shape>>(
    peek: Peek<'mem, 'facet, 'shape>,
    mut writer: W,
) -> Result<(), W::Error> {
    static VAR: &FieldAttribute = &FieldAttribute::Arbitrary("var");

    enum Task<'mem, 'facet, 'shape> {
        Value(Peek<'mem, 'facet, 'shape>),
        ValueVariable(Peek<'mem, 'facet, 'shape>),
        Object(FieldsForSerializeIter<'mem, 'facet, 'shape>),
        Array(PeekListLikeIter<'mem, 'facet, 'shape>, bool),
        List(PeekListLikeIter<'mem, 'facet, 'shape>, bool),
        Map(PeekMapIter<'mem, 'facet, 'shape>, bool),
    }

    let mut stack = Vec::new();
    stack.push(Task::Value(peek));

    while let Some(task) = stack.pop() {
        match task {
            Task::Value(mut peek) => {
                if peek.shape().attributes.contains(&ShapeAttribute::Transparent) {
                    let inner = peek.into_struct().unwrap();
                    peek = inner.field(0).unwrap();
                }

                match peek.shape().def {
                    Def::Scalar(..) => match peek.scalar_type() {
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
                        _ => todo!("TODO: Support other scalar types"),
                    },
                    Def::Map(..) => {
                        let peek = peek.into_map().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(Task::Map(peek.iter(), false));
                    }
                    Def::List(..) | Def::Slice(..) => {
                        let peek = peek.into_list_like().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(Task::List(peek.iter(), false));
                    }
                    Def::Array(..) => {
                        let peek = peek.into_list_like().unwrap();
                        stack.push(Task::Array(peek.iter(), false));
                    }
                    Def::Option(..) => {
                        if let Some(value) = peek.into_option().unwrap().value() {
                            writer.serialize_bool(true)?;
                            stack.push(Task::Value(value));
                        } else {
                            writer.serialize_bool(false)?;
                        }
                    }
                    Def::Set(..) => todo!("Push `Task::Set`"),
                    Def::SmartPointer(..) => {
                        let peek = peek.into_smart_pointer().unwrap();
                        if let Some(inner) = peek.borrow_inner() {
                            stack.push(Task::Value(inner));
                        } else {
                            panic!("Attempted to serialize a smart pointer with no inner value!");
                        }
                    }
                    Def::Undefined => match peek.shape().ty {
                        #[expect(clippy::single_match_else)]
                        Type::User(UserType::Struct(ty)) => match ty.kind {
                            StructKind::Unit => writer.serialize_unit()?,
                            _ => {
                                let peek = peek.into_struct().unwrap();
                                stack.push(Task::Object(peek.fields_for_serialize()));
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
                                        if field.attributes.contains(VAR) {
                                            stack.push(Task::ValueVariable(peek));
                                        } else {
                                            stack.push(Task::Value(peek));
                                        }
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
                    _ => {}
                }
            }
            Task::ValueVariable(mut peek) => {
                if peek.shape().attributes.contains(&ShapeAttribute::Transparent) {
                    let inner = peek.into_struct().unwrap();
                    peek = inner.field(0).unwrap();
                }

                match peek.shape().def {
                    Def::Scalar(..) => match peek.scalar_type() {
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
                        other => {
                            panic!(
                                "Attempted to serialize a non-scalar type `{other:?}` as variable-length"
                            )
                        }
                    },
                    Def::Option(..) => {
                        if let Some(value) = peek.into_option().unwrap().value() {
                            writer.serialize_bool(true)?;
                            stack.push(Task::ValueVariable(value));
                        } else {
                            writer.serialize_bool(false)?;
                        }
                    }
                    Def::Map(..) => {
                        let peek = peek.into_map().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(Task::Map(peek.iter(), true));
                    }
                    Def::List(..) | Def::Slice(..) => {
                        let peek = peek.into_list_like().unwrap();
                        writer.serialize_var_usize(peek.len())?;
                        stack.push(Task::List(peek.iter(), true));
                    }
                    Def::Array(..) => {
                        let peek = peek.into_list_like().unwrap();
                        stack.push(Task::Array(peek.iter(), true));
                    }
                    other => {
                        panic!(
                            "Attempted to serialize a non-scalar type `{other:?}` as variable-length"
                        )
                    }
                }
            }
            Task::Object(mut peek) => {
                let Some((field, value)) = peek.next() else { continue };
                stack.push(Task::Object(peek));

                if field.attributes.contains(VAR) {
                    stack.push(Task::ValueVariable(value));
                } else {
                    stack.push(Task::Value(value));
                }
            }
            Task::List(mut peek, var) => {
                let Some(entry) = peek.next() else { continue };
                stack.push(Task::List(peek, var));
                if var {
                    stack.push(Task::ValueVariable(entry));
                } else {
                    stack.push(Task::Value(entry));
                }
            }
            Task::Array(mut peek, var) => {
                let Some(entry) = peek.next() else { continue };
                stack.push(Task::Array(peek, var));
                if var {
                    stack.push(Task::ValueVariable(entry));
                } else {
                    stack.push(Task::Value(entry));
                }
            }
            Task::Map(mut peek, var) => {
                let Some((key, value)) = peek.next() else { continue };
                stack.push(Task::Map(peek, var));
                if var {
                    stack.push(Task::ValueVariable(value));
                } else {
                    stack.push(Task::Value(value));
                }
                stack.push(Task::Value(key));
            }
        }
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

impl<W: WriteAdapter> Deref for McSerializer<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<W: WriteAdapter> DerefMut for McSerializer<W> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
