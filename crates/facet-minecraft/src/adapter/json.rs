use facet_json::JsonWrite;

use crate::{FacetAdapter, WriteAdapter};

/// Implement `facet_serialize` -> `facet_minecraft`
impl<T: JsonWrite> WriteAdapter for FacetAdapter<T> {
    type Error = core::convert::Infallible;

    #[expect(clippy::unit_arg)]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        Ok(<T as JsonWrite>::write(self, buf))
    }

    fn reserve(&mut self, len: usize) { <T as JsonWrite>::reserve(self, len); }
}

// -------------------------------------------------------------------------------------------------

/// Implement `facet_minecraft` -> `facet_serialize`
impl<T: WriteAdapter> JsonWrite for FacetAdapter<T>
where T::Error: core::fmt::Debug
{
    fn write(&mut self, buf: &[u8]) { <T as WriteAdapter>::write(self, buf).unwrap(); }

    fn reserve(&mut self, additional: usize) { <T as WriteAdapter>::reserve(self, additional); }
}
