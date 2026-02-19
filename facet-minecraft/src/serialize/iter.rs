//! [`SerializeIter`]
#![allow(dead_code, reason = "WIP")]

use facet::{Def, Facet, Shape, Type, UserType};
use facet_reflect::{
    FieldsForSerializeIter, HasFields, Peek, PeekDynamicValueArrayIter, PeekDynamicValueObjectIter,
    PeekListIter, PeekListLikeIter, PeekMapIter, PeekOption, PeekPointer, PeekResult, PeekSetIter,
};
use smallvec::SmallVec;

use crate::{SerializeFn, serialize::error::SerializeIterError};

/// An iterator over the fields of a type.
pub struct SerializeIter<'mem, 'facet> {
    input: &'static Shape,
    state: SmallVec<[ItemState<'mem, 'facet>; 8]>,
    next: Option<PeekValue<'mem, 'facet>>,
}

struct ItemState<'mem, 'facet> {
    iter: PeekIter<'mem, 'facet>,
    length: i128,
    write_length: bool,
    variable: bool,
}

enum PeekIter<'mem, 'facet> {
    Scalar(Peek<'mem, 'facet>),
    Map(PeekMapIter<'mem, 'facet>),
    Set(PeekSetIter<'mem, 'facet>),
    List(PeekListIter<'mem, 'facet>),
    ListLike(PeekListLikeIter<'mem, 'facet>),
    // NdArray(PeekNdArray<'mem, 'facet>),
    Fields(FieldsForSerializeIter<'mem, 'facet>),
    DynamicValueArray(PeekDynamicValueArrayIter<'mem, 'facet>),
    DynamicValueObject(PeekDynamicValueObjectIter<'mem, 'facet>),
    Option(PeekOption<'mem, 'facet>),
    Result(PeekResult<'mem, 'facet>),
    Pointer(PeekPointer<'mem, 'facet>),
}

/// A value returned by [`SerializeIter`].
///
/// Does not care about signed/unsigned values as they serialize the same,
/// and treats all variable-length values as [`u128`]s for simplicity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeekValue<'mem, 'facet> {
    /// A [`unit`](::core::primitive::unit) value.
    Unit(()),
    /// A [`bool`] value.
    Bool(bool),
    /// A [`u8`] value.
    U8(u8),
    /// A [`u16`] value.
    U16(u16),
    /// A [`u32`] value.
    U32(u32),
    /// A [`u64`] value.
    U64(u64),
    /// A [`u128`] value.
    U128(u128),
    /// A variable-length value.
    Variable(u128),
    /// An [`f32`] value.
    F32(f32),
    /// An [`f64`] value.
    F64(f64),
    /// A [`&[u8]`](::core::primitive::slice) value.
    Bytes(&'mem [u8]),
    /// A [`Peek`] and [`SerializeFn`] to use.
    Custom(Peek<'mem, 'facet>, SerializeFn),
}

impl<'mem, 'facet> TryFrom<Peek<'mem, 'facet>> for PeekValue<'mem, 'facet> {
    type Error = SerializeIterError<'mem, 'facet>;

    #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
    fn try_from(value: Peek<'mem, 'facet>) -> Result<Self, Self::Error> {
        if let Ok(&()) = value.get::<()>() {
            Ok(Self::Unit(()))
        } else if let Ok(&bool) = value.get::<bool>() {
            Ok(Self::Bool(bool))
        } else if let Ok(&u8) = value.get::<u8>() {
            Ok(Self::U8(u8))
        } else if let Ok(&i8) = value.get::<i8>() {
            Ok(Self::U8(i8 as u8))
        } else if let Ok(&u16) = value.get::<u16>() {
            Ok(Self::U16(u16))
        } else if let Ok(&i16) = value.get::<i16>() {
            Ok(Self::U16(i16 as u16))
        } else if let Ok(&u32) = value.get::<u32>() {
            Ok(Self::U32(u32))
        } else if let Ok(&i32) = value.get::<i32>() {
            Ok(Self::U32(i32 as u32))
        } else if let Ok(&u64) = value.get::<u64>() {
            Ok(Self::U64(u64))
        } else if let Ok(&i64) = value.get::<i64>() {
            Ok(Self::U64(i64 as u64))
        } else if let Ok(&u128) = value.get::<u128>() {
            Ok(Self::U128(u128))
        } else if let Ok(&i128) = value.get::<i128>() {
            Ok(Self::U128(i128 as u128))
        } else if let Ok(&usize) = value.get::<usize>() {
            Ok(Self::U64(usize as u64))
        } else if let Ok(&isize) = value.get::<isize>() {
            Ok(Self::U64(isize as u64))
        } else if let Ok(&f32) = value.get::<f32>() {
            Ok(Self::F32(f32))
        } else if let Ok(&f64) = value.get::<f64>() {
            Ok(Self::F64(f64))
        } else if let Some(str) = value.as_str() {
            Ok(Self::Bytes(str.as_bytes()))
        } else if let Some(bytes) = value.as_bytes() {
            Ok(Self::Bytes(bytes))
        } else if let Ok(str) = value.get::<alloc::string::String>() {
            Ok(Self::Bytes(str.as_bytes()))
        } else {
            Err(SerializeIterError::new())
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'mem, 'facet> SerializeIter<'mem, 'facet> {
    /// Create a new [`SerializeIter`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is not supported for serialization.
    #[inline]
    pub fn new<T: Facet<'facet> + ?Sized>(
        input: &'mem T,
    ) -> Result<Self, SerializeIterError<'mem, 'facet>> {
        Self::new_from_peek(Peek::new(input))
    }

    /// Create a new [`SerializeIter`] for the given [`Peek`].
    ///
    /// # Errors
    ///
    /// Returns an error if the [`Peek`] is not a supported type for
    /// serialization.
    pub fn new_from_peek(
        peek: Peek<'mem, 'facet>,
    ) -> Result<Self, SerializeIterError<'mem, 'facet>> {
        let mut iter = Self { input: peek.shape(), state: SmallVec::new_const(), next: None };
        iter.state.push(Self::create_state(peek, false)?);
        Ok(iter)
    }

    /// Create an [`ItemState`] for the given [`Peek`].
    #[expect(clippy::too_many_lines, reason = "Handles many cases")]
    fn create_state(
        peek: Peek<'mem, 'facet>,
        variable: bool,
    ) -> Result<ItemState<'mem, 'facet>, SerializeIterError<'mem, 'facet>> {
        match peek.shape().def {
            Def::Scalar => Ok(ItemState {
                iter: PeekIter::Scalar(peek),
                length: 1,
                write_length: false,
                variable,
            }),
            Def::Map(_) => {
                let map = peek.into_map()?;
                Ok(ItemState {
                    length: map.len() as i128,
                    write_length: true,
                    iter: PeekIter::Map(map.iter()),
                    variable,
                })
            }
            Def::Set(_) => {
                let set = peek.into_set()?;
                Ok(ItemState {
                    length: set.len() as i128,
                    write_length: true,
                    iter: PeekIter::Set(set.iter()),
                    variable,
                })
            }
            Def::List(_) => {
                let list = peek.into_list()?;
                Ok(ItemState {
                    length: list.len() as i128,
                    write_length: true,
                    iter: PeekIter::List(list.iter()),
                    variable,
                })
            }
            Def::Array(_) => {
                let array = peek.into_list_like()?;
                Ok(ItemState {
                    length: array.len() as i128,
                    write_length: false,
                    iter: PeekIter::ListLike(array.iter()),
                    variable,
                })
            }
            Def::Slice(_) => {
                let slice = peek.into_list_like()?;
                Ok(ItemState {
                    length: slice.len() as i128,
                    write_length: true,
                    iter: PeekIter::ListLike(slice.iter()),
                    variable,
                })
            }
            Def::DynamicValue(_) => {
                let dynamic = peek.into_dynamic_value()?;
                if let Some(iter) = dynamic.array_iter() {
                    Ok(ItemState {
                        length: iter.len() as i128,
                        write_length: false,
                        iter: PeekIter::DynamicValueArray(iter),
                        variable,
                    })
                } else if let Some(iter) = dynamic.object_iter() {
                    Ok(ItemState {
                        length: iter.len() as i128,
                        write_length: false,
                        iter: PeekIter::DynamicValueObject(iter),
                        variable,
                    })
                } else {
                    Err(SerializeIterError::new())
                }
            }

            Def::Option(_) => {
                let option = peek.into_option()?;
                Ok(ItemState {
                    length: i128::from(option.is_some()),
                    write_length: true,
                    iter: PeekIter::Option(option),
                    variable,
                })
            }
            Def::Result(_) => {
                let result = peek.into_result()?;
                Ok(ItemState {
                    length: i128::from(result.is_ok()),
                    write_length: true,
                    iter: PeekIter::Result(result),
                    variable,
                })
            }
            Def::Pointer(_) => Ok(ItemState {
                length: 0,
                write_length: false,
                iter: PeekIter::Pointer(peek.into_pointer()?),
                variable,
            }),

            _ if matches!(peek.shape().ty, Type::User(UserType::Struct(_))) => {
                let struct_peek = peek.into_struct()?;
                Ok(ItemState {
                    length: struct_peek.field_count() as i128,
                    write_length: false,
                    iter: PeekIter::Fields(struct_peek.fields_for_serialize()),
                    variable: false,
                })
            }

            // TODO: Return errors instead of unwrapping
            _ if matches!(peek.shape().ty, Type::User(UserType::Enum(_))) => {
                let enum_peek = peek.into_enum()?;
                let variant = enum_peek.active_variant().unwrap();
                Ok(ItemState {
                    length: i128::from(variant.discriminant.unwrap()),
                    write_length: true,
                    iter: PeekIter::Fields(enum_peek.fields_for_serialize()),
                    variable: false,
                })
            }

            _ => Err(SerializeIterError::new()),
        }
    }

    #[expect(clippy::too_many_lines, reason = "Handles many cases")]
    fn next_inner(
        &mut self,
    ) -> Option<Result<PeekValue<'mem, 'facet>, SerializeIterError<'mem, 'facet>>> {
        macro_rules! wrap {
            ($expr:expr) => {
                match $expr {
                    Ok(value) => value,
                    Err(err) => return Some(Err(err)),
                }
            };
        }

        if let Some(next) = self.next.take() {
            return Some(Ok(next));
        }

        loop {
            let state = self.state.last_mut()?;

            if state.write_length {
                state.write_length = false;
                return Some(Ok(PeekValue::Variable(state.length.cast_unsigned())));
            }

            match &mut state.iter {
                // Pop and return the scalar value.
                PeekIter::Scalar(_) => {
                    let state = self.state.pop().unwrap_or_else(|| unreachable!());
                    let PeekIter::Scalar(peek) = state.iter else { unreachable!() };

                    match wrap!(PeekValue::try_from(peek)) {
                        PeekValue::Unit(()) => {}
                        value @ PeekValue::Bytes(bytes) => {
                            self.next = Some(value);
                            return Some(Ok(PeekValue::Variable(bytes.len() as u128)));
                        }
                        value => {
                            return Some(Ok(value));
                        }
                    }
                }

                // Push values onto the stack for the next iteration.
                PeekIter::Map(iter) => match iter.next() {
                    Some((key, val)) => {
                        let variable = state.variable;
                        self.state.push(wrap!(Self::create_state(val, variable)));
                        self.state.push(wrap!(Self::create_state(key, variable)));
                    }
                    None => {
                        let _ = self.state.pop()?;
                    }
                },
                PeekIter::Set(iter) => {
                    if let Some(value) = iter.next() {
                        let variable = state.variable;
                        self.state.push(wrap!(Self::create_state(value, variable)));
                    } else {
                        let _ = self.state.pop()?;
                    }
                }
                PeekIter::List(iter) => {
                    if let Some(value) = iter.next() {
                        let variable = state.variable;
                        self.state.push(wrap!(Self::create_state(value, variable)));
                    } else {
                        let _ = self.state.pop()?;
                    }
                }
                PeekIter::ListLike(iter) => {
                    if let Some(value) = iter.next() {
                        let variable = state.variable;
                        self.state.push(wrap!(Self::create_state(value, variable)));
                    } else {
                        let _ = self.state.pop()?;
                    }
                }
                PeekIter::Fields(iter) => {
                    if let Some((field, value)) = iter.next() {
                        let mut variable = false;
                        if let Some(field) = field.field.as_ref() {
                            // Look for `#[facet(mc::serialize = my_fn)]`
                            if let Some(attr) = field.get_attr(Some("mc"), "serialize") {
                                if let Some(crate::attribute::Attr::Serialize(Some(serialize))) =
                                    attr.get_as::<crate::attribute::Attr>()
                                {
                                    return Some(Ok(PeekValue::Custom(value, *serialize)));
                                }

                                return Some(Err(SerializeIterError::new()));
                            }

                            // Look for `#[facet(mc::variable)]`
                            variable = field.has_attr(Some("mc"), "variable");
                        }

                        self.state.push(wrap!(Self::create_state(value, variable)));
                    } else {
                        let _ = self.state.pop()?;
                    }
                }
                PeekIter::DynamicValueArray(iter) => {
                    if let Some(value) = iter.next() {
                        let variable = state.variable;
                        self.state.push(wrap!(Self::create_state(value, variable)));
                    } else {
                        let _ = self.state.pop()?;
                    }
                }
                PeekIter::DynamicValueObject(iter) => {
                    if let Some((key, value)) = iter.next() {
                        let variable = state.variable;
                        self.state.push(wrap!(Self::create_state(value, variable)));
                        self.state.push(wrap!(Self::create_state(Peek::new(key), variable)));
                    } else {
                        let _ = self.state.pop()?;
                    }
                }

                PeekIter::Option(option) => {
                    if let Some(value) = option.value() {
                        *state = wrap!(Self::create_state(value, state.variable));
                    } else {
                        let _ = self.state.pop()?;
                    }
                }
                PeekIter::Result(result) => {
                    if let Some(result) = result.ok() {
                        *state = wrap!(Self::create_state(result, state.variable));
                    } else if let Some(result) = result.err() {
                        *state = wrap!(Self::create_state(result, state.variable));
                    } else {
                        unreachable!("Result wasn't either `Ok` or `Err`?");
                    }
                }
                PeekIter::Pointer(ptr) => {
                    let inner = wrap!(ptr.borrow_inner().ok_or_else(SerializeIterError::new));
                    *state = wrap!(Self::create_state(inner, state.variable));
                }
            }
        }
    }
}

impl<'mem, 'facet> Iterator for SerializeIter<'mem, 'facet> {
    type Item = Result<PeekValue<'mem, 'facet>, SerializeIterError<'mem, 'facet>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_inner() {
            Some(Ok(value)) => Some(Ok(value)),
            None => None,

            Some(Err(err)) => {
                self.state.clear(); // Prevent further iteration after an error
                Some(Err(err))
            }
        }
    }
}
