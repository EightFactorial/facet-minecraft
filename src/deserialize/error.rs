use core::fmt::{Debug, Display};

/// An error that occurred during deserialization.
pub struct DeserializeError<'input, 'facet, 'shape> {
    _phantom: core::marker::PhantomData<(&'input (), &'facet (), &'shape ())>,
}

// -------------------------------------------------------------------------------------------------

#[cfg(not(feature = "rich-diagnostics"))]
impl Debug for DeserializeError<'_, '_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("DeserializeError")
    }
}

#[cfg(not(feature = "rich-diagnostics"))]
impl Display for DeserializeError<'_, '_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("DeserializeError")
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "rich-diagnostics")]
impl Debug for DeserializeError<'_, '_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("DeserializeError")
    }
}

#[cfg(feature = "rich-diagnostics")]
impl Display for DeserializeError<'_, '_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("DeserializeError")
    }
}
