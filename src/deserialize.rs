use alloc::borrow::Cow;

use facet::Facet;
use facet_deserialize::{
    Cooked, DeserErrorKind, Expectation, Format, NextData, NextResult, Outcome, Span, Spanned,
};

use crate::{Minecraft, assert::AssertProtocol};

/// Deserialize a type from the given byte slice.
///
/// # Errors
/// Returns an error if the deserialization fails.
#[expect(clippy::result_large_err)]
pub fn deserialize<'input, 'facet, 'shape, T>(
    _input: &'input [u8],
) -> Result<T, DeserializeError<'input, 'shape>>
where
    'input: 'facet,
    T: Facet<'facet> + AssertProtocol<'facet>,
{
    <T as AssertProtocol<'facet>>::assert();

    todo!()
}

/// An error that can occur during deserialization.
#[derive(Debug)]
pub struct DeserializeError<'input, 'shape> {
    /// The input that caused the error.
    pub input: Cow<'input, [u8]>,
    /// Where the error occurred
    pub span: Span<Cooked>,
    /// The specific error that occurred..
    pub kind: DeserErrorKind<'shape>,
    /// The source identifier.
    pub source: &'static str,
}

// -------------------------------------------------------------------------------------------------

impl Format for Minecraft {
    type Input<'i> = [u8];
    type SpanType = Cooked;

    fn source(&self) -> &'static str { "protocol" }

    fn next<'i, 'f, 's>(
        &mut self,
        _next: NextData<'i, 'f, 's, Cooked, [u8]>,
        _expt: Expectation,
    ) -> NextResult<
        'i,
        'f,
        's,
        Spanned<Outcome<'i>, Cooked>,
        Spanned<DeserErrorKind<'s>, Cooked>,
        Cooked,
        [u8],
    >
    where
        's: 'i,
    {
        todo!()
    }

    fn skip<'i, 'f, 's>(
        &mut self,
        _next: NextData<'i, 'f, 's, Cooked, [u8]>,
    ) -> NextResult<'i, 'f, 's, Span<Cooked>, Spanned<DeserErrorKind<'s>, Cooked>, Cooked, [u8]>
    where
        's: 'i,
    {
        todo!()
    }
}
