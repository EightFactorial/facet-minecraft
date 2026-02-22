//! TODO
#![allow(
    clippy::new_without_default,
    unknown_lints,
    renamed_and_removed_lints,
    elidable_lifetime_names,
    reason = "Temporary"
)]

use core::{
    error::Error,
    fmt::{self, Debug, Display},
    str::Utf8Error,
};

use facet_reflect::ReflectError;

use crate::deserialize::iter::DeserializeIter;

/// An error that can occur during deserialization.
pub struct DeserializeError<'facet> {
    _phantom: core::marker::PhantomData<&'facet ()>,
}

impl<'facet> DeserializeError<'facet> {
    /// Create a new [`DeserializeError`].
    #[must_use]
    pub fn new() -> Self { Self { _phantom: core::marker::PhantomData } }
}

impl<'facet> Error for DeserializeError<'facet> {}
impl<'facet> Display for DeserializeError<'facet> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl<'facet> Debug for DeserializeError<'facet> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl<'facet> From<ReflectError> for DeserializeError<'facet> {
    fn from(_: ReflectError) -> Self { todo!() }
}
#[cfg(feature = "std")]
impl<'facet> From<std::io::Error> for DeserializeError<'facet> {
    fn from(_: std::io::Error) -> Self { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur during deserialization.
#[expect(clippy::large_enum_variant, reason = "May contain iterator")]
pub enum DeserializeIterError<'facet, const BORROW: bool> {
    /// Attempted to create a boolean from a non-boolean value.
    Boolean(u8),
    /// Attempted to borrow data with a static lifetime.
    StaticBorrow,
    /// An error from the facet reflection system.
    Reflect(ReflectError),
    /// An error from UTF-8 decoding.
    Utf8(Utf8Error),
    /// An error from the deserializer running out of data.
    EndOfInput {
        /// The error's data.
        error: EndOfInput,
        /// The iterator, which may be resumed with more data.
        iterator: DeserializeIter<'facet, BORROW>,
    },
}

impl<'facet, const BORROW: bool> DeserializeIterError<'facet, BORROW> {
    /// A placeholder constructor for [`DeserializeIterError`].
    #[must_use]
    pub fn new() -> Self { todo!() }
}

impl<'facet, const BORROW: bool> From<ReflectError> for DeserializeIterError<'facet, BORROW> {
    fn from(value: ReflectError) -> Self { Self::Reflect(value) }
}
impl<'facet, const BORROW: bool> From<Utf8Error> for DeserializeIterError<'facet, BORROW> {
    fn from(value: Utf8Error) -> Self { Self::Utf8(value) }
}

impl<'facet, const BORROW: bool> From<DeserializeIterError<'facet, BORROW>>
    for DeserializeError<'facet>
{
    fn from(_: DeserializeIterError<'facet, BORROW>) -> Self {
        Self { _phantom: core::marker::PhantomData }
    }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur during deserialization of a value.
#[derive(Debug, Clone)]
pub enum DeserializeValueError {
    /// Attempted to create a boolean from a non-boolean value.
    Boolean(u8),
    /// Attempted to borrow data with a static lifetime.
    StaticBorrow,
    /// An error from the facet reflection system.
    Reflect(ReflectError),
    /// An error from UTF-8 decoding.
    Utf8(Utf8Error),
    /// An error from the deserializer running out of data.
    EndOfInput(EndOfInput),
}

impl From<ReflectError> for DeserializeValueError {
    fn from(value: ReflectError) -> Self { Self::Reflect(value) }
}
impl From<Utf8Error> for DeserializeValueError {
    fn from(value: Utf8Error) -> Self { Self::Utf8(value) }
}
impl From<EndOfInput> for DeserializeValueError {
    fn from(value: EndOfInput) -> Self { Self::EndOfInput(value) }
}

impl From<DeserializeValueError> for DeserializeError<'_> {
    fn from(_: DeserializeValueError) -> Self { Self { _phantom: core::marker::PhantomData } }
}

/// An error indicating that the end of the input was reached unexpectedly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndOfInput {
    /// The number of bytes that were read.
    pub had: usize,
    /// The number of additional bytes that were expected.
    pub expected: usize,
}
