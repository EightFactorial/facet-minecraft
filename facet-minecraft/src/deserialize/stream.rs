use alloc::{rc::Rc, vec::Vec};
use core::{cell::RefCell, num::NonZeroUsize};
use std::io::Cursor;

use corosensei::{Coroutine, CoroutineResult, Yielder};
use facet::Shape;
use facet_format::{
    DeserializeError as FDError, EnumVariantHint, FormatDeserializer, FormatParser, ParseEvent,
    ScalarTypeHint, ScalarValue,
};
use facet_reflect::Span;

use crate::deserialize::{
    Deserializable, DeserializeError, DeserializeErrorKind, DeserializerStack, McDeserializerProbe,
    StackEntry, parse,
};

/// A wrapper around a [`Coroutine`] for deserializing a value of type `T`.
struct CoWrapper<T> {
    /// A shared buffer for reading data into.
    buffer: Rc<RefCell<Cursor<Vec<u8>>>>,
    /// A coroutine for deserializing the value.
    coroutine: Coroutine<(), Option<NonZeroUsize>, Result<T, FDError<DeserializeError>>>,
}

impl<T: Deserializable<'static>> CoWrapper<T> {
    /// Create a new [`CoWrapper`] for deserializing `T`.
    #[must_use]
    fn new() -> Self {
        let hint = T::DESERIALIZE_HINT.maximum().or(T::DESERIALIZE_HINT.minimum()).unwrap_or(0);
        let buffer = Rc::new(RefCell::new(Cursor::new(Vec::with_capacity(hint))));

        let cobuffer = Rc::clone(&buffer);
        let coroutine =
            Coroutine::new(move |yielder, ()| -> Result<T, FDError<DeserializeError>> {
                FormatDeserializer::new_owned(McStreamDeserializer::new(cobuffer, yielder))
                    .deserialize_root::<T>()
            });

        Self { buffer, coroutine }
    }

    /// Complete the deserialization synchronously.
    fn complete(
        mut self,
        mut f: impl FnMut(&mut Cursor<Vec<u8>>) -> Result<(), FDError<DeserializeError>>,
    ) -> Result<T, FDError<DeserializeError>> {
        loop {
            match self.coroutine.resume(()) {
                CoroutineResult::Yield(growth) => {
                    let mut cursor = self.buffer.borrow_mut();

                    // Grow the buffer if needed
                    let buffer = cursor.get_mut();
                    if let Some(growth) = growth {
                        buffer.resize(buffer.len() + growth.get(), 0);
                    }

                    // Read more data into the buffer
                    f(&mut cursor)?;
                }
                CoroutineResult::Return(result) => {
                    return result;
                }
            }
        }
    }

    /// Complete the deserialization asynchronously.
    #[allow(dead_code, reason = "May not be used if no async features are enabled")]
    #[expect(clippy::await_holding_refcell_ref, reason = "Necessary for coroutine")]
    async fn complete_async(
        mut self,
        mut f: impl AsyncFnMut(&mut Cursor<Vec<u8>>) -> Result<(), FDError<DeserializeError>>,
    ) -> Result<T, FDError<DeserializeError>> {
        loop {
            match self.coroutine.resume(()) {
                CoroutineResult::Yield(growth) => {
                    let mut cursor = self.buffer.borrow_mut();

                    // Grow the buffer if needed
                    let buffer = cursor.get_mut();
                    if let Some(growth) = growth {
                        buffer.resize(buffer.len() + growth.get(), 0);
                    }

                    // Read more data into the buffer
                    f(&mut cursor).await?;
                }
                CoroutineResult::Return(result) => {
                    return result;
                }
            }
        }
    }
}

/// Deserialize a value of type `T` from a [`Reader`](std::io::Read).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[expect(clippy::cast_possible_truncation, reason = "")]
pub fn from_reader<T: Deserializable<'static>, R: std::io::Read>(
    reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    CoWrapper::<T>::new().complete(|cursor| {
        let index = cursor.position() as usize;
        std::io::Read::read_exact(reader, &mut cursor.get_mut()[index..])
            .map_err(|err| FDError::Parser(DeserializeError::from(err)))
    })
}

/// Deserialize a value of type `T` from an asynchronous
/// [`AsyncRead`](futures_lite::AsyncRead).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[cfg(feature = "futures-lite")]
#[expect(clippy::cast_possible_truncation, reason = "")]
pub async fn from_async_reader<T: Deserializable<'static>, R: futures_lite::AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    CoWrapper::<T>::new()
        .complete_async(async |cursor| {
            let index = cursor.position() as usize;
            futures_lite::io::AsyncReadExt::read_exact(reader, &mut cursor.get_mut()[index..])
                .await
                .map_err(|err| FDError::Parser(DeserializeError::from(err)))
        })
        .await
}

/// Deserialize a value of type `T` from an asynchronous
/// [`AsyncRead`](tokio::io::AsyncRead).
///
/// # Errors
///
/// This function will return an error if deserialization fails,
/// or the reader encounters an I/O error.
#[cfg(feature = "tokio")]
#[expect(clippy::cast_possible_truncation, reason = "")]
pub async fn from_tokio_reader<T: Deserializable<'static>, R: tokio::io::AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<T, FDError<DeserializeError>> {
    // const { assert!(T::DESERIALIZABLE.possible(), "This type is not
    // deserializable!") };

    CoWrapper::<T>::new()
        .complete_async(async |cursor| {
            let index = cursor.position() as usize;
            tokio::io::AsyncReadExt::read_exact(reader, &mut cursor.get_mut()[index..])
                .await
                .map_or_else(|err| Err(FDError::Parser(DeserializeError::from(err))), |_| Ok(()))
        })
        .await
}

// -------------------------------------------------------------------------------------------------

/// TODO
pub struct McStreamDeserializer<'de, 'y> {
    buffer: Rc<RefCell<Cursor<Vec<u8>>>>,
    yielder: &'y Yielder<(), Option<NonZeroUsize>>,

    stack: DeserializerStack,
    peek: Option<ParseEvent<'de>>,
}

impl<'de, 'y> McStreamDeserializer<'de, 'y> {
    /// Create a new [`McStreamDeserializer`].
    #[must_use]
    pub const fn new(
        buffer: Rc<RefCell<Cursor<Vec<u8>>>>,
        yielder: &'y Yielder<(), Option<NonZeroUsize>>,
    ) -> Self {
        Self { buffer, yielder, stack: DeserializerStack::new(), peek: None }
    }

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
    ) -> Result<ScalarValue<'static>, DeserializeError> {
        let cursor = self.buffer.borrow();
        #[expect(clippy::cast_possible_truncation, reason = "")]
        let position = cursor.position() as usize;

        match cursor.get_ref().get(position..) {
            Some(input) => {
                // Attempt to parse the scalar value
                let mut result = parse::parse_owned_scalar(input, hint, variable);

                // If we hit an unexpected end of input, grow the buffer and try again
                if let Err(err) = &result
                    && let DeserializeErrorKind::UnexpectedEndOfInput { expected, found } =
                        *err.kind()
                    && found < expected
                {
                    // Release the borrow and yield
                    drop(cursor);
                    self.yielder.suspend(Some(NonZeroUsize::new(expected - found).unwrap()));
                    // Retry parsing with the grown buffer
                    let cursor = self.buffer.borrow();
                    let input = cursor.get_ref().get(position..).unwrap();
                    result = parse::parse_owned_scalar(input, hint, variable);
                }

                // If parsing succeeded, advance the cursor
                match result {
                    Ok((value, consumed)) => {
                        let mut cursor = self.buffer.borrow_mut();
                        cursor.set_position((position + consumed) as u64);
                        Ok(value)
                    }
                    Err(err) => Err(err),
                }
            }
            None => Err(DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfInput {
                expected: position,
                found: cursor.get_ref().len(),
            })),
        }
    }
}

impl<'de> FormatParser<'de> for McStreamDeserializer<'de, '_> {
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

    #[expect(clippy::cast_possible_truncation, reason = "")]
    fn current_span(&self) -> Option<Span> {
        if let Ok(buffer) = self.buffer.try_borrow() {
            Some(Span::new(buffer.position() as usize, 0))
        } else {
            None
        }
    }
}
