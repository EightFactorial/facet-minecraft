use alloc::borrow::Cow;

use facet::Facet;

use crate::assert::AssertProtocol;

/// Deserialize a type from the given byte slice.
///
/// # Errors
/// Returns an error if the deserialization fails.
pub fn deserialize<'input, 'facet, T>(_input: &'input [u8]) -> Result<T, DeserializeError<'input>>
where
    'input: 'facet,
    T: Facet<'facet> + AssertProtocol<'facet>,
{
    <T as AssertProtocol<'facet>>::assert();

    todo!()
}

/// An error that can occur during deserialization.
#[derive(Debug)]
pub struct DeserializeError<'input> {
    /// The input that caused the error.
    pub input: Cow<'input, [u8]>,
    /// Where the error occurred.
    pub source: &'static str,
}

// -------------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------------
