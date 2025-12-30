//! TODO

use facet::{Facet, Field, Shape};
use facet_format::{
    DeserializeError as FDError, EnumVariantHint, FieldEvidence, FormatDeserializer, FormatParser,
    ParseEvent, ProbeStream, ScalarTypeHint, ScalarValue,
};
use facet_reflect::Span;

mod error;
pub use error::{DeserializeError, DeserializeErrorKind};

#[cfg(feature = "jit")]
mod jit;
#[cfg(feature = "jit")]
pub use jit::McJitFormat;

mod parse;

mod stack;
use stack::{DeserializerStack, StackEntry};

#[cfg(feature = "streaming")]
pub(crate) mod stream;
#[cfg(feature = "streaming")]
pub use stream::*;

pub(crate) mod r#trait;
pub use r#trait::Deserializable;

/// A function pointer to a deserialization function.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct DeserializeFn {
    ptr: for<'de> fn(
        &mut McDeserializer<'de>,
        &'de Field,
    ) -> Result<ParseEvent<'de>, DeserializeError>,
}

impl DeserializeFn {
    /// Create a new [`DeserializeFn`].
    #[inline]
    #[must_use]
    pub const fn new(
        ptr: for<'de> fn(
            &mut McDeserializer<'de>,
            &'de Field,
        ) -> Result<ParseEvent<'de>, DeserializeError>,
    ) -> Self {
        Self { ptr }
    }

    /// Call the deserialization function.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeError`] if deserialization fails.
    #[inline]
    pub fn call<'de>(
        &self,
        deserializer: &mut McDeserializer<'de>,
        field: &'de Field,
    ) -> Result<ParseEvent<'de>, DeserializeError> {
        (self.ptr)(deserializer, field)
    }
}

impl
    From<
        for<'de> fn(
            &mut McDeserializer<'de>,
            &'de Field,
        ) -> Result<ParseEvent<'de>, DeserializeError>,
    > for DeserializeFn
{
    #[inline]
    fn from(
        ptr: for<'de> fn(
            &mut McDeserializer<'de>,
            &'de Field,
        ) -> Result<ParseEvent<'de>, DeserializeError>,
    ) -> Self {
        Self::new(ptr)
    }
}

// -------------------------------------------------------------------------------------------------

/// A deserializer that implements [`FormatParser`].
pub struct McDeserializer<'de> {
    #[allow(dead_code, reason = "WIP")]
    input: &'de [u8],
    counter: usize,

    stack: DeserializerStack,
    peek: Option<ParseEvent<'de>>,
}

impl<'de> McDeserializer<'de> {
    /// Create a new [`McDeserializer`] using the given counter.
    #[must_use]
    pub const fn new(input: &'de [u8]) -> Self {
        Self { input, counter: 0, stack: DeserializerStack::new(), peek: None }
    }

    /// Returns the number of bytes consumed so far.
    #[inline]
    #[must_use]
    pub const fn consumed(&self) -> usize { self.counter }

    /// Parse the next event from the input.
    #[expect(unused_variables, reason = "WIP")]
    fn parse_next(&mut self) -> Result<Option<ParseEvent<'de>>, DeserializeError> {
        let Some(entry) = self.stack.next_mut() else { return Ok(None) };
        match entry {
            StackEntry::Struct { remaining } => todo!(),
            StackEntry::Enum { variants, variant, remaining } => todo!(),
            StackEntry::Sequence { remaining } => todo!(),
            StackEntry::Map { remaining } => todo!(),

            StackEntry::Scalar { hint } => {
                let hint = *hint;
                self.parse_scalar(hint, false).map(|value| Some(ParseEvent::Scalar(value)))
            }
            StackEntry::Optional { present } => todo!(),
        }
    }

    fn parse_scalar(
        &mut self,
        hint: ScalarTypeHint,
        variable: bool,
    ) -> Result<ScalarValue<'de>, DeserializeError> {
        match self.input.get(self.counter..) {
            Some(input) => parse::parse_input(input, hint, variable).map(|(value, consumed)| {
                self.counter += consumed;
                value
            }),
            None => Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                expected: self.counter,
                found: self.input.len(),
            })),
        }
    }
}

impl<'de> FormatParser<'de> for McDeserializer<'de> {
    type Error = DeserializeError;
    type Probe<'a>
        = McDeserializerProbe
    where
        Self: 'a;

    fn next_event(&mut self) -> Result<Option<ParseEvent<'de>>, Self::Error> {
        self.peek.take().map_or_else(|| self.parse_next(), |event| Ok(Some(event)))
    }

    fn peek_event(&mut self) -> Result<Option<ParseEvent<'de>>, Self::Error> {
        self.peek.clone().map_or_else(
            || {
                let event = self.next_event()?;
                self.peek.clone_from(&event);
                Ok(event)
            },
            |event| Ok(Some(event)),
        )
    }

    fn skip_value(&mut self) -> Result<(), Self::Error> { self.next_event().map(|_| ()) }

    fn begin_probe(&mut self) -> Result<Self::Probe<'_>, Self::Error> { Ok(McDeserializerProbe) }

    fn is_self_describing(&self) -> bool { false }

    fn hint_struct_fields(&mut self, num: usize) { self.stack.push_struct_hint(num); }

    fn hint_scalar_type(&mut self, hint: ScalarTypeHint) { self.stack.push_scalar_hint(hint); }

    fn hint_sequence(&mut self) { self.stack.push_sequence_hint(None); }

    fn hint_array(&mut self, len: usize) { self.stack.push_sequence_hint(Some(len)); }

    fn hint_option(&mut self) { self.stack.push_optional_hint(); }

    fn hint_map(&mut self) { self.stack.push_map_hint(); }

    fn hint_enum(&mut self, variants: &[EnumVariantHint]) { self.stack.push_enum_hint(variants); }

    fn hint_opaque_scalar(&mut self, _ident: &'static str, _shape: &'static Shape) -> bool { false }

    fn current_span(&self) -> Option<Span> {
        Some(Span::new(self.counter, self.input.len().saturating_sub(self.counter)))
    }
}

/// A deserializer probe that implements [`ProbeStream`].
pub struct McDeserializerProbe;

impl<'a> ProbeStream<'a> for McDeserializerProbe {
    type Error = DeserializeError;

    fn next(&mut self) -> Result<Option<FieldEvidence<'a>>, Self::Error> { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// Deserialize a value of type `T` from a byte slice and returning any
/// remaining bytes.
///
/// # Note
///
/// This function **does not** support JIT!
///
/// Use [`from_slice_borrowed`] or any of the other deserialization functions if
/// you want JIT support.
///
/// # Errors
///
/// This function will return an error if deserialization fails.
pub fn from_slice<T: Deserializable<'static>>(
    input: &[u8],
) -> Result<(T, &[u8]), FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    let mut format = FormatDeserializer::new_owned(McDeserializer::new(input));

    format.deserialize_root::<T>().and_then(|val| {
        let consumed = format.parser_mut().consumed();
        if let Some(remaining) = input.get(consumed..) {
            Ok((val, remaining))
        } else {
            // This should never happen, but just in case...
            Err(FDError::Parser(DeserializeError::new(
                DeserializeErrorKind::UnexpectedEndOfInput {
                    expected: consumed,
                    found: input.len(),
                },
            )))
        }
    })
}

/// Deserialize a value of type `T` from a byte slice and returning any
/// remaining bytes, allowing zero-copy borrowing.
///
/// This variant requires the input to outlive the result (`'input: 'facet`),
/// enabling zero-copy deserialization of string fields as `&str` or `Cow<str>`.
///
/// Use this when you need maximum performance and can guarantee the input
/// buffer outlives the deserialized value. For most use cases, prefer
/// [`from_slice`] which doesn't have lifetime requirements.
///
/// # Errors
///
/// This function will return an error if deserialization fails.
#[cfg(not(feature = "jit"))]
pub fn from_slice_borrowed<'input: 'facet, 'facet, T: Deserializable<'facet>>(
    input: &'input [u8],
) -> Result<(T, &'input [u8]), FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    let mut format = FormatDeserializer::new(McDeserializer::new(input));

    format.deserialize_root::<T>().and_then(|val| {
        let consumed = format.parser_mut().consumed();
        if let Some(remaining) = input.get(consumed..) {
            Ok((val, remaining))
        } else {
            // This should never happen, but just in case...
            Err(FDError::Parser(DeserializeError::new(
                DeserializeErrorKind::UnexpectedEndOfInput {
                    expected: consumed,
                    found: input.len(),
                },
            )))
        }
    })
}

/// Deserialize a value of type `T` from a byte slice and returning any
/// remaining bytes, allowing zero-copy borrowing.
///
/// This variant requires the input to outlive the result (`'input: 'facet`),
/// enabling zero-copy deserialization of string fields as `&str` or `Cow<str>`.
///
/// Use this when you need maximum performance and can guarantee the input
/// buffer outlives the deserialized value. For most use cases, prefer
/// [`from_slice`] which doesn't have lifetime requirements.
///
/// # Errors
///
/// This function will return an error if deserialization fails.
#[cfg(feature = "jit")]
pub fn from_slice_borrowed<'input: 'facet, 'facet, T: Deserializable<'facet>>(
    input: &'input [u8],
) -> Result<(T, &'input [u8]), FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    let mut format = McDeserializer::new(input);

    if let Some(result) = facet_format::jit::try_deserialize_with_format_jit::<T, _>(&mut format) {
        result.and_then(|val| {
            let consumed = format.consumed();
            if let Some(remaining) = input.get(consumed..) {
                Ok((val, remaining))
            } else {
                // This should never happen, but just in case...
                Err(FDError::Parser(DeserializeError::new(
                    DeserializeErrorKind::UnexpectedEndOfInput {
                        expected: consumed,
                        found: input.len(),
                    },
                )))
            }
        })
    } else {
        // Fallback to non-JIT deserialization

        let mut format = FormatDeserializer::new(format);

        format.deserialize_root::<T>().and_then(|val| {
            let consumed = format.parser_mut().consumed();
            if let Some(remaining) = input.get(consumed..) {
                Ok((val, remaining))
            } else {
                // This should never happen, but just in case...
                Err(FDError::Parser(DeserializeError::new(
                    DeserializeErrorKind::UnexpectedEndOfInput {
                        expected: consumed,
                        found: input.len(),
                    },
                )))
            }
        })
    }
}
