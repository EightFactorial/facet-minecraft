pub use facet_deserialize::DeserError;
use facet_deserialize::{
    Cooked, DeserErrorKind, Expectation, Format, NextData, Outcome, Span, Spanned,
};

use crate::{Minecraft, assert::AssertProtocol};

/// Deserialize a type from the given byte slice.
///
/// # Errors
/// Returns an error if the deserialization fails.
#[inline]
#[expect(clippy::result_large_err)]
pub fn deserialize<'input: 'facet, 'facet, 'shape, T: AssertProtocol<'facet>>(
    input: &'input [u8],
) -> Result<T, DeserError<'input, 'shape, Cooked>> {
    <T as AssertProtocol<'facet>>::assert();

    facet_deserialize::deserialize(input, Minecraft)
}

// -------------------------------------------------------------------------------------------------

impl Format for Minecraft {
    type Input<'i> = [u8];
    type SpanType = Cooked;

    fn source(&self) -> &'static str { crate::ERROR_SOURCE }

    fn next<'i, 'f, 's: 'i>(
        &mut self,
        next: NextData<'i, 'f, 's, Cooked, [u8]>,
        _expt: Expectation,
    ) -> (
        NextData<'i, 'f, 's, Cooked, [u8]>,
        Result<Spanned<Outcome<'i>, Cooked>, Spanned<DeserErrorKind<'s>, Cooked>>,
    ) {
        let _input = &next.input()[next.start()..];

        todo!()
    }

    fn skip<'i, 'f, 's: 'i>(
        &mut self,
        next: NextData<'i, 'f, 's, Cooked, [u8]>,
    ) -> (
        NextData<'i, 'f, 's, Cooked, [u8]>,
        Result<Span<Cooked>, Spanned<DeserErrorKind<'s>, Cooked>>,
    ) {
        let _input = &next.input()[next.start()..];

        todo!()
    }
}
