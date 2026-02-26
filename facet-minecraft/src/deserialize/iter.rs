//! [`DeserializeIter`] and related types.
#![allow(dead_code, unused, reason = "WIP")]

use alloc::{borrow::Cow, string::String, vec::Vec};
use core::{
    fmt::{self, Display},
    marker::PhantomData,
};

use facet::{
    Def, Facet, HeapValue, KnownPointer, Partial, ReflectError, Shape, StructType, Type, UserType,
};
use smallvec::SmallVec;
use uuid::Uuid;

use crate::{
    DeserializeFn,
    deserialize::error::{DeserializeError, DeserializeIterError, DeserializeValueError},
};

/// An iterator over the fields of a type.
///
/// Uses [`Partial`]s to provide locations for field data.
pub struct DeserializeIter<'facet, const BORROW: bool> {
    input: &'static Shape,
    partial: Partial<'facet, BORROW>,
    stack: SmallVec<[ItemState; 8]>,
}

impl<'facet> DeserializeIter<'facet, true> {
    /// Creates a new [`DeserializeIter`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is unsized.
    pub fn new<T: Facet<'facet>>() -> Result<Self, DeserializeError<'facet>> {
        let mut stack = SmallVec::new_const();
        let partial = Partial::alloc::<T>()?;
        Self::push_partial(&mut stack, &partial, false)?;
        Ok(Self { input: T::SHAPE, partial, stack })
    }
}

impl DeserializeIter<'static, false> {
    /// Creates a new [`DeserializeIter`] for the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is unsized.
    pub fn new<T: Facet<'static>>() -> Result<Self, DeserializeError<'static>> {
        let mut stack = SmallVec::new_const();
        let partial = Partial::alloc_owned::<T>()?;
        Self::push_partial(&mut stack, &partial, false)?;
        Ok(Self { input: T::SHAPE, partial, stack })
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`Partial`] value that must be filled in by a deserializer.
pub enum PartialValue<'mem, 'facet, const BORROW: bool> {
    /// A [`bool`] value.
    Bool(PartialLense<'mem, 'facet, BORROW, bool>),
    /// A [`u8`] value.
    U8(PartialLense<'mem, 'facet, BORROW, u8>),
    /// A [`u16`] value, and whether it is variable-length encoded.
    U16(PartialLense<'mem, 'facet, BORROW, u16>, bool),
    /// A [`u32`] value, and whether it is variable-length encoded.
    U32(PartialLense<'mem, 'facet, BORROW, u32>, bool),
    /// A [`u64`] value, and whether it is variable-length encoded.
    U64(PartialLense<'mem, 'facet, BORROW, u64>, bool),
    /// A [`u128`] value, and whether it is variable-length encoded.
    U128(PartialLense<'mem, 'facet, BORROW, u128>, bool),
    /// A [`usize`] value, and whether it is variable-length encoded.
    Usize(PartialLense<'mem, 'facet, BORROW, usize>, bool),
    /// A [`i8`] value.
    I8(PartialLense<'mem, 'facet, BORROW, i8>),
    /// A [`i16`] value, and whether it is variable-length encoded.
    I16(PartialLense<'mem, 'facet, BORROW, i16>, bool),
    /// A [`i32`] value, and whether it is variable-length encoded.
    I32(PartialLense<'mem, 'facet, BORROW, i32>, bool),
    /// A [`i64`] value, and whether it is variable-length encoded.
    I64(PartialLense<'mem, 'facet, BORROW, i64>, bool),
    /// A [`i128`] value, and whether it is variable-length encoded.
    I128(PartialLense<'mem, 'facet, BORROW, i128>, bool),
    /// A [`isize`] value, and whether it is variable-length encoded.
    Isize(PartialLense<'mem, 'facet, BORROW, isize>, bool),
    /// A [`f32`] value.
    F32(PartialLense<'mem, 'facet, BORROW, f32>),
    /// A [`f64`] value.
    F64(PartialLense<'mem, 'facet, BORROW, f64>),
    /// A [`str`] value.
    Str(PartialLense<'mem, 'facet, BORROW, &'facet str>),
    /// A [`String`] value.
    String(PartialLense<'mem, 'facet, BORROW, String>),
    /// A [`Cow<'_, str>`] value.
    CowStr(PartialLense<'mem, 'facet, BORROW, Cow<'facet, str>>),
    /// A `&[u8]` value.
    Bytes(PartialLense<'mem, 'facet, BORROW, &'facet [u8]>),
    /// A [`Vec<u8>`] value.
    VecBytes(PartialLense<'mem, 'facet, BORROW, Vec<u8>>),
    /// A [`Cow<'_, [u8]>`] value.
    CowBytes(PartialLense<'mem, 'facet, BORROW, Cow<'facet, [u8]>>),
    /// A [`Uuid`] value.
    Uuid(PartialLense<'mem, 'facet, BORROW, Uuid>),

    /// A variable-length encoded [`usize`] value.
    Length(&'mem mut Option<usize>),
    /// A [`Partial`] and a [`DeserializeFn`] to use.
    Custom(&'mem mut Partial<'facet, BORROW>, DeserializeFn),
}

impl<const BORROW: bool> Display for PartialValue<'_, '_, BORROW> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(..) => f.write_str("Bool"),
            Self::U8(..) => f.write_str("U8"),
            Self::U16(..) => f.write_str("U16"),
            Self::U32(..) => f.write_str("U32"),
            Self::U64(..) => f.write_str("U64"),
            Self::U128(..) => f.write_str("U128"),
            Self::Usize(..) => f.write_str("Usize"),
            Self::I8(..) => f.write_str("I8"),
            Self::I16(..) => f.write_str("I16"),
            Self::I32(..) => f.write_str("I32"),
            Self::I64(..) => f.write_str("I64"),
            Self::I128(..) => f.write_str("I128"),
            Self::Isize(..) => f.write_str("Isize"),
            Self::F32(..) => f.write_str("F32"),
            Self::F64(..) => f.write_str("F64"),
            Self::Str(..) => f.write_str("Str"),
            Self::String(..) => f.write_str("String"),
            Self::CowStr(..) => f.write_str("CowStr"),
            Self::Bytes(..) => f.write_str("Bytes"),
            Self::VecBytes(..) => f.write_str("VecBytes"),
            Self::CowBytes(..) => f.write_str("CowBytes"),
            Self::Uuid(..) => f.write_str("Uuid"),
            Self::Length(..) => f.write_str("Length"),
            Self::Custom(partial, ..) => write!(f, "Custom ({})", partial.shape().type_name()),
        }
    }
}

/// A lense for a [`Partial`] that allows setting it's value.
pub struct PartialLense<'mem, 'facet, const BORROW: bool, T: Facet<'facet>> {
    partial: &'mem mut Partial<'facet, BORROW>,
    _phantom: PhantomData<T>,
}

impl<'mem, 'facet, const BORROW: bool, T: Facet<'facet>> PartialLense<'mem, 'facet, BORROW, T> {
    /// Creates a new [`PartialLense`] for the given [`Partial`].
    ///
    /// # Panics
    ///
    /// Panics if the [`Partial`] is not for the same type as the lense.
    pub fn new(partial: &'mem mut Partial<'facet, BORROW>) -> Self {
        partial.shape().assert_shape(T::SHAPE);
        Self { partial, _phantom: PhantomData }
    }

    /// Sets the value of the [`Partial`] this lense points to.
    ///
    /// # Panics
    ///
    /// If the shape of the [`Partial`] is not correct, this will panic.
    ///
    /// To prevent undefined behavior, the process will be aborted if this
    /// panics.
    ///
    /// TODO: Check if this can use `unwrap_unchecked` instead of `unwrap` to
    /// avoid the extra check, since we should already know that the shape
    /// is correct.
    pub fn set_value(self, value: T) {
        replace_with::replace_with_or_abort(self.partial, |val| val.set(value).unwrap());
    }
}

// -------------------------------------------------------------------------------------------------

impl<'facet, const BORROW: bool> DeserializeIter<'facet, BORROW> {
    fn push_partial(
        stack: &mut SmallVec<[ItemState; 8]>,
        partial: &Partial<'facet, BORROW>,
        variable: bool,
    ) -> Result<(), DeserializeValueError> {
        // Look for `#[facet(mc::deserialize = my_fn)]`
        if partial
            .shape()
            .attributes
            .iter()
            .any(|attr| attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "deserialize")
        {
            stack.push(ItemState::value(variable));
            return Ok(());
        }

        match partial.shape().def {
            Def::Scalar => {
                stack.push(ItemState::value(variable));
                Ok(())
            }
            Def::Map(def) => {
                stack.push(ItemState::map(None, variable));
                Ok(())
            }
            Def::Set(def) => {
                stack.push(ItemState::set(None, variable));
                Ok(())
            }
            Def::List(def) => {
                // Special case `Vec<u8>`, deserialize as bytes
                if partial.shape().is_type::<alloc::vec::Vec<u8>>() {
                    stack.push(ItemState::value(variable));
                    return Ok(());
                }

                stack.push(ItemState::list(None, variable));
                Ok(())
            }
            Def::Array(def) => {
                stack.push(ItemState::array(def.n, variable));
                Ok(())
            }
            Def::NdArray(def) => todo!(),
            Def::Slice(def) => todo!(),
            Def::Option(def) => {
                stack.push(ItemState::option(None, variable));
                Ok(())
            }
            Def::Result(def) => {
                stack.push(ItemState::result(None, variable));
                Ok(())
            }
            Def::Pointer(def) => {
                stack.push(ItemState::ptr(variable));
                Ok(())
            }
            Def::DynamicValue(def) => todo!(),

            _ if matches!(partial.shape().ty, Type::User(UserType::Struct(_))) => {
                let Type::User(UserType::Struct(data)) = partial.shape().ty else { unreachable!() };
                stack.push(ItemState::fields(data));
                Ok(())
            }
            _ if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) => {
                stack.push(ItemState::variant());
                Ok(())
            }

            _ => todo!(),
        }
    }

    fn create_value<'mem>(
        partial: &'mem mut Partial<'facet, BORROW>,
        variable: bool,
    ) -> Result<PartialValue<'mem, 'facet, BORROW>, DeserializeValueError> {
        if partial.shape().is_type::<bool>() {
            Ok(PartialValue::Bool(PartialLense::new(partial)))
        } else if partial.shape().is_type::<u8>() {
            Ok(PartialValue::U8(PartialLense::new(partial)))
        } else if partial.shape().is_type::<u16>() {
            Ok(PartialValue::U16(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<u32>() {
            Ok(PartialValue::U32(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<u64>() {
            Ok(PartialValue::U64(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<u128>() {
            Ok(PartialValue::U128(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<usize>() {
            Ok(PartialValue::Usize(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<i8>() {
            Ok(PartialValue::I8(PartialLense::new(partial)))
        } else if partial.shape().is_type::<i16>() {
            Ok(PartialValue::I16(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<i32>() {
            Ok(PartialValue::I32(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<i64>() {
            Ok(PartialValue::I64(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<i128>() {
            Ok(PartialValue::I128(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<isize>() {
            Ok(PartialValue::Isize(PartialLense::new(partial), variable))
        } else if partial.shape().is_type::<f32>() {
            Ok(PartialValue::F32(PartialLense::new(partial)))
        } else if partial.shape().is_type::<f64>() {
            Ok(PartialValue::F64(PartialLense::new(partial)))
        } else if partial.shape().is_type::<&'static str>() {
            Ok(PartialValue::Str(PartialLense::new(partial)))
        } else if partial.shape().is_type::<String>() {
            Ok(PartialValue::String(PartialLense::new(partial)))
        } else if partial.shape().is_type::<&'static [u8]>() {
            Ok(PartialValue::Bytes(PartialLense::new(partial)))
        } else if partial.shape().is_type::<Vec<u8>>() {
            Ok(PartialValue::VecBytes(PartialLense::new(partial)))
        } else if partial.shape().is_type::<Uuid>() {
            Ok(PartialValue::Uuid(PartialLense::new(partial)))
        } else {
            todo!()
        }
    }

    /// Advances the iterator to the next field.
    ///
    /// Returns itself and boolean indicating whether the iterator is
    /// complete.
    ///
    /// # Errors
    ///
    /// Returns an error if the processor fails to process a [`Partial`].
    ///
    /// If the processor is out of data, deserialization can be resumed by
    /// calling `next` again after providing more data to the processor.
    #[allow(clippy::missing_panics_doc, reason = "WIP")]
    #[expect(clippy::too_many_lines, reason = "WIP")]
    pub fn next<F: FnMut(PartialValue<'_, 'facet, BORROW>) -> Result<(), DeserializeValueError>>(
        mut self,
        mut processor: F,
    ) -> Result<(Self, bool), DeserializeIterError<'facet, BORROW>> {
        macro_rules! wrap {
            (@custom, $attr:expr) => {{
                // Look for `#[facet(mc::deserialize = my_fn)]`
                if let Some(attr) = $attr.find(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "deserialize"
                }) {
                    // Use the custom deserialize function
                    if let Some(crate::attribute::Attr::Deserialize(Some(deserialize))) =
                        attr.get_as::<crate::attribute::Attr>()
                    {
                        wrap!(@process PartialValue::Custom(&mut self.partial, *deserialize));
                        true
                    } else {
                        return Err(DeserializeIterError::new());
                    }
                } else {
                    false
                }
            }};
            (@error $($tt:tt)*) => {
                match $($tt)* {
                    Ok(value) => value,
                    Err(err) => {
                        return match err {
                            DeserializeValueError::Boolean(value) => Err(DeserializeIterError::Boolean(value)),
                            DeserializeValueError::StaticBorrow => Err(DeserializeIterError::StaticBorrow),
                            DeserializeValueError::Reflect(err) => Err(DeserializeIterError::Reflect(err)),
                            DeserializeValueError::Utf8(err) => Err(DeserializeIterError::Utf8(err)),
                            DeserializeValueError::EndOfInput(_) => unreachable!(),
                        }
                    }
                }

            };
            (@erroriter $($tt:tt)*) => {
                match $($tt)* {
                    Ok(value) => value,
                    Err(err) => {
                        return match err {
                            DeserializeValueError::Boolean(value) => Err(DeserializeIterError::Boolean(value)),
                            DeserializeValueError::StaticBorrow => Err(DeserializeIterError::StaticBorrow),
                            DeserializeValueError::Reflect(err) => Err(DeserializeIterError::Reflect(err)),
                            DeserializeValueError::Utf8(err) => Err(DeserializeIterError::Utf8(err)),
                            DeserializeValueError::EndOfInput(error) => Err(DeserializeIterError::EndOfInput { error, iterator: self }),
                        }
                    }
                }

            };
            (@process $($tt:tt)*) => {
                wrap!(@erroriter (processor)($($tt)*));
            };
        }

        loop {
            // Get the current state, or return if we're done.
            let Some(state) = self.stack.last_mut() else {
                return Ok((self, true));
            };
            #[cfg(feature = "tracing")]
            tracing::trace!(
                target: "facet_minecraft::deserialize",
                "`{}` @ \"{}\": {state}",
                self.input.type_name(),
                self.partial.path()
            );

            match state {
                ItemState::Value { variable } => {
                    // Look for `#[facet(mc::deserialize = my_fn)]` on the type
                    if wrap!(@custom, self.partial.shape().attributes.iter()) {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                        continue;
                    }

                    wrap!(@process wrap!(@error Self::create_value(&mut self.partial, *variable)));
                    let _ = self.stack.pop();
                    if !self.stack.is_empty() {
                        self.partial = self.partial.end()?;
                    }
                }

                ItemState::Fields { data, field_index } => {
                    if let Some(field) = data.fields.get(*field_index) {
                        self.partial = self.partial.begin_nth_field(*field_index)?;
                        *field_index += 1;

                        // Look for `#[facet(mc::deserialize = my_fn)]` on the field
                        if wrap!(@custom, field.attributes.iter()) {
                            self.partial = self.partial.end()?;
                            continue;
                        }

                        // Look for `#[facet(mc::variable)]`
                        let variable = field.attributes.iter().any(|attr| {
                            attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable"
                        });

                        wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                    } else {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    }
                }

                ItemState::Variant { discriminant } => {
                    #[expect(clippy::cast_possible_wrap, reason = "Desired behavior")]
                    if discriminant.is_none() {
                        // Read the discriminant
                        let mut disc = None;
                        wrap!(@process PartialValue::Length(&mut disc));
                        *discriminant = disc.map(|d| d as i64);
                    }

                    // Select the variant
                    self.partial = self.partial.select_variant(discriminant.unwrap())?;

                    if let Type::User(UserType::Enum(ty)) = self.partial.shape().ty
                        && let Some(variant) =
                            ty.variants.iter().find(|v| v.discriminant == *discriminant)
                    {
                        // Replace this stack item with the fields of the variant
                        *self.stack.last_mut().unwrap() = ItemState::fields(variant.data);
                    } else {
                        return Err(DeserializeIterError::new());
                    }
                }
                ItemState::Array { remaining, state, variable } => {
                    if !*state {
                        *state = true;

                        // Initialize the array
                        self.partial = self.partial.init_array()?;
                    }

                    if *remaining > 0 {
                        *remaining -= 1;

                        // Read the next value
                        let variable = *variable;
                        self.partial = self.partial.begin_list_item()?;
                        wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                    } else {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    }
                }
                ItemState::List { remaining, variable } => {
                    if remaining.is_none() {
                        // Read the length
                        let mut len = None;
                        wrap!(@process PartialValue::Length(&mut len));
                        *remaining = len;

                        // Initialize the list
                        self.partial = self.partial.init_list()?;
                    }

                    if remaining.unwrap() > 0 {
                        *remaining = remaining.map(|r| r - 1);
                        // Read the next value
                        let variable = *variable;
                        self.partial = self.partial.begin_list_item()?;
                        wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                    } else {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    }
                }
                ItemState::Map { remaining, state, variable } => {
                    if remaining.is_none() {
                        // Read the length
                        let mut len = None;
                        wrap!(@process PartialValue::Length(&mut len));
                        *remaining = len;

                        // Initialize the map
                        self.partial = self.partial.init_map()?;
                    }

                    if remaining.unwrap() > 0 {
                        // Note: `state` defaults to `false`
                        if *state {
                            *remaining = remaining.map(|r| r - 1);

                            // Read the next value
                            let variable = *variable;
                            self.partial = self.partial.begin_value()?;
                            wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                        } else {
                            // Read the next key
                            self.partial = self.partial.begin_key()?;
                            wrap!(@error Self::push_partial(&mut self.stack, &self.partial, false));
                        }
                    } else {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    }
                }
                ItemState::Set { remaining, variable } => {
                    if remaining.is_none() {
                        // Read the length
                        let mut len = None;
                        wrap!(@process PartialValue::Length(&mut len));
                        *remaining = len;

                        // Initialize the set
                        self.partial = self.partial.init_set()?;
                    }

                    if remaining.unwrap() > 0 {
                        *remaining = remaining.map(|r| r - 1);
                        // Read the next value
                        let variable = *variable;
                        self.partial = self.partial.begin_value()?;
                        wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                    } else {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    }
                }
                ItemState::Ptr { started, variable } => {
                    if *started {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    } else {
                        let Def::Pointer(def) = self.partial.shape().def else { unreachable!() };
                        let is_cow = matches!(def.known, Some(KnownPointer::Cow));

                        *started = true;

                        // Special case `Cow<'_, str>`
                        if is_cow
                            && let Some(pointee) = def.pointee()
                            && *pointee == *str::SHAPE
                        {
                            wrap!(@process PartialValue::CowStr(PartialLense::new(&mut self.partial)));
                            continue;
                        }
                        // Special case `Cow<'_, [u8]>`
                        if is_cow
                            && let Some(pointee) = def.pointee()
                            && let Def::Slice(slice_def) = pointee.def
                            && *slice_def.t == *u8::SHAPE
                        {
                            wrap!(@process PartialValue::CowBytes(PartialLense::new(&mut self.partial)));
                            continue;
                        }

                        // Other `Cow` types
                        if is_cow {
                            let variable = *variable;
                            self.partial = self.partial.begin_inner()?;
                            wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                            continue;
                        }

                        todo!()
                    }
                }

                ItemState::Option { state, variable } => {
                    if state.is_none() {
                        // Read the discriminant
                        let mut discriminant = None;
                        wrap!(@process PartialValue::Length(&mut discriminant));
                        match discriminant {
                            Some(0) => *state = Some(false),
                            Some(1) => *state = Some(true),
                            Some(other) => {
                                return Err(DeserializeIterError::Boolean(other.to_be_bytes()[0]));
                            }
                            None => return Err(DeserializeIterError::new()),
                        }

                        if state.unwrap() {
                            // `Some`
                            let variable = *variable;
                            self.partial = self.partial.begin_some()?;
                            wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                        } else {
                            // `None`
                            self.partial = self.partial.set_default()?;
                        }
                    } else {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    }
                }
                ItemState::Result { state, variable } => {
                    if state.is_none() {
                        // Read the discriminant
                        let mut discriminant = None;
                        wrap!(@process PartialValue::Length(&mut discriminant));
                        match discriminant {
                            Some(0) => *state = Some(false),
                            Some(1) => *state = Some(true),
                            Some(other) => {
                                return Err(DeserializeIterError::Boolean(other.to_be_bytes()[0]));
                            }
                            None => return Err(DeserializeIterError::new()),
                        }

                        if state.unwrap() {
                            // `Ok`
                            let variable = *variable;
                            self.partial = self.partial.begin_ok()?;
                            wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                        } else {
                            // `Err`
                            let variable = *variable;
                            self.partial = self.partial.begin_err()?;
                            wrap!(@error Self::push_partial(&mut self.stack, &self.partial, variable));
                        }
                    } else {
                        let _ = self.stack.pop();
                        if !self.stack.is_empty() {
                            self.partial = self.partial.end()?;
                        }
                    }
                }
            }
        }
    }

    /// Advances the iterator until completion,
    /// processing each field with the given processor.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    pub fn complete<
        F: FnMut(PartialValue<'_, 'facet, BORROW>) -> Result<(), DeserializeValueError>,
    >(
        mut self,
        mut processor: F,
    ) -> Result<HeapValue<'facet, BORROW>, DeserializeError<'facet>> {
        loop {
            match self.next(&mut processor) {
                Ok((iter, false)) => self = iter,
                Ok((iter, true)) => return Ok(iter.partial.build()?),
                Err(err) => return Err(DeserializeError::from(err)),
            }
        }
    }

    /// Consumes the iterator and returns current [`Partial`].
    ///
    /// This should only be used after the iterator has been fully processed
    /// and the final [`Partial`] is ready to be built.
    #[must_use]
    pub fn into_partial(self) -> Partial<'facet, BORROW> { self.partial }
}

#[derive(Debug, Clone, Copy)]
enum ItemState {
    Value { variable: bool },
    Fields { data: StructType, field_index: usize },
    Variant { discriminant: Option<i64> },
    Array { remaining: usize, state: bool, variable: bool },
    List { remaining: Option<usize>, variable: bool },
    Map { remaining: Option<usize>, state: bool, variable: bool },
    Set { remaining: Option<usize>, variable: bool },
    Ptr { started: bool, variable: bool },
    Option { state: Option<bool>, variable: bool },
    Result { state: Option<bool>, variable: bool },
}

impl ItemState {
    /// Create an [`ItemState::Value`].
    const fn value(variable: bool) -> Self { Self::Value { variable } }

    /// Create an [`ItemState::Fields`].
    const fn fields(data: StructType) -> Self { Self::Fields { data, field_index: 0 } }

    /// Create an [`ItemState::Variant`].
    const fn variant() -> Self { Self::Variant { discriminant: None } }

    /// Create an [`ItemState::Array`].
    const fn array(len: usize, variable: bool) -> Self {
        Self::Array { remaining: len, state: false, variable }
    }

    /// Create an [`ItemState::List`].
    const fn list(len: Option<usize>, variable: bool) -> Self {
        Self::List { remaining: len, variable }
    }

    /// Create an [`ItemState::Map`].
    const fn map(len: Option<usize>, variable: bool) -> Self {
        Self::Map { remaining: len, state: false, variable }
    }

    /// Create an [`ItemState::Set`].
    const fn set(len: Option<usize>, variable: bool) -> Self {
        Self::Set { remaining: len, variable }
    }

    /// Create an [`ItemState::Ptr`].
    const fn ptr(variable: bool) -> Self { Self::Ptr { started: false, variable } }

    /// Create an [`ItemState::Option`].
    const fn option(state: Option<bool>, variable: bool) -> Self {
        Self::Option { state, variable }
    }

    /// Create an [`ItemState::Result`].
    const fn result(state: Option<bool>, variable: bool) -> Self {
        Self::Result { state, variable }
    }
}

impl Display for ItemState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value { .. } => f.write_str("Value"),
            Self::Fields { .. } => f.write_str("Fields"),
            Self::Variant { .. } => f.write_str("Variant"),
            Self::Array { .. } => f.write_str("Array"),
            Self::List { .. } => f.write_str("List"),
            Self::Map { .. } => f.write_str("Map"),
            Self::Set { .. } => f.write_str("Set"),
            Self::Ptr { .. } => f.write_str("Ptr"),
            Self::Option { .. } => f.write_str("Option"),
            Self::Result { .. } => f.write_str("Result"),
        }
    }
}
