use facet::Facet;

use crate::{adapter::WriteAdapter, assert::AssertProtocol};

/// Serialize a type to the given writer.
///
/// # Errors
/// Returns an error if the serialization fails.
pub fn serialize<'input, 'facet, T, W>(_value: &'input T, _writer: W) -> Result<(), W::Error>
where
    'input: 'facet,
    'facet: 'input,
    T: Facet<'facet> + AssertProtocol<'facet>,
    W: WriteAdapter,
{
    <T as AssertProtocol<'facet>>::assert();

    todo!()
}

// -------------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------------
