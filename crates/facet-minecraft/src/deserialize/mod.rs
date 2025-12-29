//! TODO

use facet::{Attr, Facet, Field, ScalarType, Shape};
use facet_format::{
    DeserializeError as FDError, FieldEvidence, FormatDeserializer, FormatParser, ParseEvent,
    ProbeStream,
};

mod error;
pub use error::{DeserializeError, DeserializeErrorKind};

#[cfg(feature = "jit")]
mod jit;
#[cfg(feature = "jit")]
pub use jit::McJitFormat;

pub(crate) mod r#trait;
pub use r#trait::Deserializable;

use crate::{
    attribute::Attr as McAttr,
    iterator::{FieldOrShape, ShapeFieldIter},
};

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

// -------------------------------------------------------------------------------------------------

/// A deserializer that implements [`FormatParser`].
pub struct McDeserializer<'de> {
    #[allow(dead_code, reason = "WIP")]
    input: &'de [u8],
    counter: usize,

    iter: ShapeFieldIter<'de>,
    peek: Option<ParseEvent<'de>>,
}

impl<'de> McDeserializer<'de> {
    /// Create a new [`McDeserializer`] using the given counter.
    #[must_use]
    pub fn new(shape: &'de Shape, input: &'de [u8]) -> Self {
        Self { input, counter: 0, iter: ShapeFieldIter::new(shape), peek: None }
    }

    /// Returns the number of bytes consumed so far.
    #[inline]
    #[must_use]
    pub const fn consumed(&self) -> usize { self.counter }

    /// Parse the next event from the input.
    fn parse_next(&mut self) -> Result<Option<ParseEvent<'de>>, DeserializeError> {
        let Some(field) = self.iter.next_field(|| todo!()) else {
            return if self.iter.is_empty() { Ok(None) } else { todo!() };
        };

        let mut variable = false;
        if let FieldOrShape::Field(field) = field {
            // Check for a custom deserializer
            if let Some(McAttr::Deserialize(Some(deserialize_fn))) =
                field.get_attr(Some("mc"), "deserialize").and_then(Attr::get_as::<McAttr>)
            {
                return deserialize_fn.call(self, field).map(Some);
            }
            // Check for variable-length encoding
            if Some(&McAttr::Variable)
                == field.get_attr(Some("mc"), "variable").and_then(Attr::get_as::<McAttr>)
            {
                variable = true;
            }
        }

        let shape = field.shape();
        if let Some(scalar) = shape.scalar_type() {
            match (scalar, variable) {
                // Units, booleans and strings
                (ScalarType::Unit, false) => todo!(),
                (ScalarType::Bool, false) => todo!(),
                (ScalarType::Str, false) => todo!(),
                (ScalarType::String, false) => todo!(),
                (ScalarType::CowStr, false) => todo!(),
                // Standard unsigned integers
                (ScalarType::U8, false) => todo!(),
                (ScalarType::U16, false) => todo!(),
                (ScalarType::U32, false) => todo!(),
                (ScalarType::U64, false) => todo!(),
                (ScalarType::U128, false) => todo!(),
                (ScalarType::USize, false) => todo!(),
                // Variable-length unsigned integers
                (ScalarType::U16, true) => todo!(),
                (ScalarType::U32, true) => todo!(),
                (ScalarType::U64, true) => todo!(),
                (ScalarType::U128, true) => todo!(),
                (ScalarType::USize, true) => todo!(),
                // Standard signed integers
                (ScalarType::I8, false) => todo!(),
                (ScalarType::I16, false) => todo!(),
                (ScalarType::I32, false) => todo!(),
                (ScalarType::I64, false) => todo!(),
                (ScalarType::I128, false) => todo!(),
                (ScalarType::ISize, false) => todo!(),
                // Variable-length signed integers
                (ScalarType::I16, true) => todo!(),
                (ScalarType::I32, true) => todo!(),
                (ScalarType::I64, true) => todo!(),
                (ScalarType::I128, true) => todo!(),
                (ScalarType::ISize, true) => todo!(),
                // Floating point numbers
                (ScalarType::F32, false) => todo!(),
                (ScalarType::F64, false) => todo!(),
                // ScalarType::Char => todo!(),
                // ScalarType::IpAddr => todo!(),
                // ScalarType::Ipv4Addr => todo!(),
                // ScalarType::Ipv6Addr => todo!(),
                // ScalarType::SocketAddr => todo!(),
                // ScalarType::ConstTypeId => todo!(),
                _ => todo!(),
            }
        } else {
            todo!()
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

    let mut format = FormatDeserializer::new_owned(McDeserializer::new(T::SHAPE, input));

    format.deserialize_root::<T>().and_then(|val| {
        let consumed = format.parser_mut().consumed();
        if let Some(remaining) = input.get(consumed..) {
            Ok((val, remaining))
        } else {
            // This should never happen, but just in case...
            Err(FDError::Parser(DeserializeError::new_eof(consumed, input.len())))
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

    let mut format = FormatDeserializer::new(McDeserializer::new(T::SHAPE, input));

    format.deserialize_root::<T>().and_then(|val| {
        let consumed = format.parser_mut().consumed();
        if let Some(remaining) = input.get(consumed..) {
            Ok((val, remaining))
        } else {
            // This should never happen, but just in case...
            Err(FDError::Parser(DeserializeError::new_eof(consumed, input.len())))
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

    let mut format = McDeserializer::new(T::SHAPE, input);

    if let Some(result) = facet_format::jit::try_deserialize_with_format_jit::<T, _>(&mut format) {
        result.and_then(|val| {
            let consumed = format.consumed();
            if let Some(remaining) = input.get(consumed..) {
                Ok((val, remaining))
            } else {
                // This should never happen, but just in case...
                Err(FDError::Parser(DeserializeError::new_eof(consumed, input.len())))
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
                Err(FDError::Parser(DeserializeError::new_eof(consumed, input.len())))
            }
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// Deserialize a value of type `T` from a [`Reader`](std::io::Read).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[cfg(feature = "streaming")]
pub fn from_reader<'facet, T: Deserializable<'facet>, R: std::io::Read>(
    _reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    todo!()
}

/// Deserialize a value of type `T` from an asynchronous
/// [`AsyncRead`](futures_io::AsyncRead).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[cfg(feature = "futures-io")]
pub async fn from_async_reader<'facet, T: Deserializable<'facet>, R: futures_io::AsyncRead>(
    _reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    todo!()
}

/// Deserialize a value of type `T` from an asynchronous
/// [`AsyncRead`](tokio::io::AsyncRead).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[cfg(feature = "tokio")]
pub async fn from_tokio_reader<'facet, T: Deserializable<'facet>, R: tokio::io::AsyncRead>(
    _reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    todo!()
}
