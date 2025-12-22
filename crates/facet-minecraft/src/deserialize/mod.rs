//! TODO

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

/// A function pointer to a deserialization function.
pub type DeserializeFn = fn();

/// A deserializer that implements [`FormatParser`].
#[derive(Debug)]
pub struct McDeserializer<'de> {
    counter: usize,
    #[expect(dead_code, reason = "WIP")]
    input: &'de [u8],
}

impl<'de> McDeserializer<'de> {
    /// Create a new [`McDeserializer`] using the given counter.
    #[must_use]
    pub const fn new(input: &'de [u8]) -> Self { Self { counter: 0usize, input } }

    /// Returns the number of bytes consumed so far.
    #[inline]
    #[must_use]
    pub const fn consumed(&self) -> usize { self.counter }
}

/// A deserializer probe that implements [`ProbeStream`].
pub struct McDeserializerProbe;

impl<'de> FormatParser<'de> for McDeserializer<'de> {
    type Error = DeserializeError;
    type Probe<'a>
        = McDeserializerProbe
    where
        Self: 'a;

    fn next_event(&mut self) -> Result<ParseEvent<'de>, Self::Error> { todo!() }

    fn peek_event(&mut self) -> Result<ParseEvent<'de>, Self::Error> { todo!() }

    fn skip_value(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_probe(&mut self) -> Result<Self::Probe<'_>, Self::Error> { todo!() }
}

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

    let mut format = FormatDeserializer::new(McDeserializer::new(input));

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

    let mut format = McDeserializer::new(input);

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
