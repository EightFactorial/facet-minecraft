//! TODO

use core::marker::PhantomData;

use facet::{Facet, Field};
use facet_format::{
    DeserializeError as FDError, FieldEvidence, FormatDeserializer, FormatParser, ParseEvent,
    ProbeStream,
};

mod error;
pub use error::{DeserializeError, DeserializeErrorKind};

mod r#trait;
pub use r#trait::{Deserializable, TypeDeserializable};

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
#[derive(Default)]
pub struct McDeserializer<'de> {
    consumed: usize,
    _marker: PhantomData<&'de ()>,
}

impl McDeserializer<'_> {
    /// Create a new [`McDeserializer`].
    #[must_use]
    pub const fn new() -> Self { Self { consumed: 0, _marker: PhantomData } }

    /// Returns the number of bytes consumed so far.
    #[inline]
    #[must_use]
    pub const fn consumed(&self) -> usize { self.consumed }
}

/// A deserializer probe that implements [`ProbeStream`].
pub struct McDeserializerProbe<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'de> FormatParser<'de> for McDeserializer<'de> {
    type Error = DeserializeError;
    type Probe<'a>
        = McDeserializerProbe<'de>
    where
        Self: 'a;

    fn next_event(&mut self) -> Result<Option<ParseEvent<'de>>, Self::Error> { todo!() }

    fn peek_event(&mut self) -> Result<Option<ParseEvent<'de>>, Self::Error> { todo!() }

    fn skip_value(&mut self) -> Result<(), Self::Error> { todo!() }

    fn begin_probe(&mut self) -> Result<Self::Probe<'_>, Self::Error> { todo!() }
}

impl<'a> ProbeStream<'a> for McDeserializerProbe<'a> {
    type Error = DeserializeError;

    fn next(&mut self) -> Result<Option<FieldEvidence<'a>>, Self::Error> { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// Deserialize a value of type `T` from a byte slice and returning any
/// remaining bytes.
///
/// # Errors
///
/// This function will return an error if deserialization fails.
pub fn from_slice<T: Deserializable<'static>>(
    input: &[u8],
) -> Result<(T, &[u8]), FDError<DeserializeError>> {
    let mut format = FormatDeserializer::new_owned(McDeserializer::new());
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
pub fn from_slice_borrowed<'input: 'facet, 'facet, T: Deserializable<'facet>>(
    input: &'input [u8],
) -> Result<(T, &'input [u8]), FDError<DeserializeError>> {
    let mut format = FormatDeserializer::new(McDeserializer::new());
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

// -------------------------------------------------------------------------------------------------

/// Deserialize a value of type `T` from a [`Reader`](std::io::Read).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[cfg(feature = "streaming")]
pub fn from_reader<R: std::io::Read, T: Deserializable<'static>>(
    _reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    todo!()
}

/// Deserialize a value of type `T` from an asynchronous
/// [`AsyncRead`](futures_io::AsyncRead).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[cfg(feature = "futures-lite")]
pub async fn from_async_reader<R: futures_lite::AsyncRead, T: Deserializable<'static>>(
    _reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
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
pub async fn from_tokio_reader<R: tokio::io::AsyncRead, T: Deserializable<'static>>(
    _reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    todo!()
}
