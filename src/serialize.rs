use alloc::{borrow::Cow, string::String, vec::Vec};
use core::ops::{Deref, DerefMut};

use facet::{Def, Facet, FieldAttribute, ShapeAttribute, StructKind, Type, UserType};
use facet_reflect::{
    FieldsForSerializeIter, HasFields, Peek, PeekListLikeIter, PeekMapIter, ScalarType,
};
use facet_serialize::Serializer;

use crate::{adapter::WriteAdapter, assert::AssertProtocol};

/// Serialize a type to the given writer.
///
/// # Errors
/// Returns an error if the serialization fails.
#[inline]
pub fn serialize<'mem, 'facet, T, W>(value: &'mem T, writer: W) -> Result<(), W::Error>
where
    'mem: 'facet,
    'facet: 'mem,
    T: Facet<'facet> + AssertProtocol<'facet>,
    W: WriteAdapter,
{
    <T as AssertProtocol<'facet>>::assert();

    serialize_iterative(Peek::new(value), &mut McSerializer(writer))
}

// -------------------------------------------------------------------------------------------------

enum Task<'mem, 'facet, 'shape> {
    Value(Peek<'mem, 'facet, 'shape>),
    ValueVariable(Peek<'mem, 'facet, 'shape>),
    Object(FieldsForSerializeIter<'mem, 'facet, 'shape>),
    Array(PeekListLikeIter<'mem, 'facet, 'shape>, bool),
    List(PeekListLikeIter<'mem, 'facet, 'shape>, bool),
    Map(PeekMapIter<'mem, 'facet, 'shape>, bool),
}

static VAR: &FieldAttribute = &FieldAttribute::Arbitrary("var");

/// Iteratively serialize a type to the given writer.
///
/// Avoids recursion to prevent depth issues with large structures.
#[expect(clippy::elidable_lifetime_names, clippy::too_many_lines)]
fn serialize_iterative<'mem, 'facet, 'shape, W: WriteAdapter>(
    peek: Peek<'mem, 'facet, 'shape>,
    writer: &mut McSerializer<W>,
) -> Result<(), W::Error> {
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

                            #[expect(clippy::cast_sign_loss)]
                            let discriminant =
                                variant.discriminant.unwrap_or_else(|| peek.discriminant()) as u64;
                            writer.start_enum_variant(discriminant)?;

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

struct McSerializer<W: WriteAdapter>(W);

/// An extension trait for [`Serializer`] that provides
/// variable-length serialization methods.
trait SerializerExt<'shape>: Serializer<'shape> {
    /// Serialize a variable-length unsigned short.
    fn serialize_var_u16(&mut self, val: u16) -> Result<(), Self::Error>;
    /// Serialize a variable-length unsigned integer.
    fn serialize_var_u32(&mut self, val: u32) -> Result<(), Self::Error>;
    /// Serialize a variable-length unsigned long.
    fn serialize_var_u64(&mut self, val: u64) -> Result<(), Self::Error>;
    /// Serialize a variable-length unsigned long long.
    fn serialize_var_u128(&mut self, val: u128) -> Result<(), Self::Error>;
    /// Serialize a variable-length unsigned size.
    #[inline]
    fn serialize_var_usize(&mut self, val: usize) -> Result<(), Self::Error> {
        self.serialize_var_u64(val as u64)
    }
    /// Serialize a variable-length signed short.
    fn serialize_var_i16(&mut self, val: i16) -> Result<(), Self::Error>;
    /// Serialize a variable-length signed integer.
    fn serialize_var_i32(&mut self, val: i32) -> Result<(), Self::Error>;
    /// Serialize a variable-length signed long.
    fn serialize_var_i64(&mut self, val: i64) -> Result<(), Self::Error>;
    /// Serialize a variable-length signed long long.
    fn serialize_var_i128(&mut self, val: i128) -> Result<(), Self::Error>;
    /// Serialize a variable-length signed size.
    #[inline]
    fn serialize_var_isize(&mut self, val: isize) -> Result<(), Self::Error> {
        self.serialize_var_i64(val as i64)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'shape, W: WriteAdapter> Serializer<'shape> for McSerializer<W> {
    type Error = W::Error;

    fn serialize_u8(&mut self, val: u8) -> Result<(), Self::Error> { self.write(&[val]) }

    fn serialize_u16(&mut self, val: u16) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_u32(&mut self, val: u32) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_u64(&mut self, val: u64) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_u128(&mut self, val: u128) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_usize(&mut self, val: usize) -> Result<(), Self::Error> {
        self.serialize_u64(val as u64)
    }

    #[expect(clippy::cast_sign_loss)]
    fn serialize_i8(&mut self, val: i8) -> Result<(), Self::Error> { self.write(&[val as u8]) }

    fn serialize_i16(&mut self, val: i16) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_i32(&mut self, val: i32) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_i64(&mut self, val: i64) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_i128(&mut self, val: i128) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_isize(&mut self, val: isize) -> Result<(), Self::Error> {
        self.serialize_i64(val as i64)
    }

    fn serialize_f32(&mut self, val: f32) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_f64(&mut self, val: f64) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_bool(&mut self, val: bool) -> Result<(), Self::Error> {
        self.write(&[u8::from(val)])
    }

    fn serialize_char(&mut self, _val: char) -> Result<(), Self::Error> {
        unimplemented!("Protocol does not support `char`")
    }

    fn serialize_str(&mut self, val: &str) -> Result<(), Self::Error> {
        self.serialize_var_usize(val.len())?;
        self.serialize_bytes(val.as_bytes())
    }

    fn serialize_bytes(&mut self, val: &[u8]) -> Result<(), Self::Error> { self.write(val) }

    fn serialize_none(&mut self) -> Result<(), Self::Error> { self.write(&[0]) }

    fn serialize_unit(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_unit_variant(&mut self, index: usize, _: &'shape str) -> Result<(), Self::Error> {
        self.serialize_var_usize(index)
    }

    fn start_enum_variant(&mut self, discriminant: u64) -> Result<(), Self::Error> {
        self.serialize_var_u64(discriminant)
    }

    fn start_object(&mut self, _: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn start_array(&mut self, _: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn start_map(&mut self, _: Option<usize>) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_field_name(&mut self, _: &'shape str) -> Result<(), Self::Error> { Ok(()) }
}

impl<W: WriteAdapter> SerializerExt<'_> for McSerializer<W> {
    #[expect(unused_assignments)]
    fn serialize_var_u16(&mut self, mut val: u16) -> Result<(), Self::Error> {
        let mut byte = 0u8;
        let mut count = 0u8;
        while (val != 0 || count == 0) && count < 3 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u16::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }

    #[expect(unused_assignments)]
    fn serialize_var_u32(&mut self, mut val: u32) -> Result<(), Self::Error> {
        let mut count = 0u8;
        let mut byte = 0u8;
        while (val != 0 || count == 0) && count < 5 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u32::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }

    #[expect(unused_assignments)]
    fn serialize_var_u64(&mut self, mut val: u64) -> Result<(), Self::Error> {
        let mut byte = 0u8;
        let mut count = 0u8;
        while (val != 0 || count == 0) && count < 10 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u64::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }

    #[expect(unused_assignments)]
    fn serialize_var_u128(&mut self, mut val: u128) -> Result<(), Self::Error> {
        let mut byte = 0u8;
        let mut count = 0u8;
        while (val != 0 || count == 0) && count < 19 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u128::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }

    #[inline]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i16(&mut self, val: i16) -> Result<(), Self::Error> {
        self.serialize_var_u16(val as u16)
    }

    #[inline]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i32(&mut self, val: i32) -> Result<(), Self::Error> {
        self.serialize_var_u32(val as u32)
    }

    #[inline]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i64(&mut self, val: i64) -> Result<(), Self::Error> {
        self.serialize_var_u64(val as u64)
    }

    #[inline]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i128(&mut self, val: i128) -> Result<(), Self::Error> {
        self.serialize_var_u128(val as u128)
    }
}

// -------------------------------------------------------------------------------------------------

impl<W: WriteAdapter> Deref for McSerializer<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<W: WriteAdapter> DerefMut for McSerializer<W> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
