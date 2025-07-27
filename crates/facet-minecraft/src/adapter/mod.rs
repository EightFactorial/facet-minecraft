mod write;
pub use write::WriteAdapter;

mod write_cursor;
pub use write_cursor::SliceCursor;

#[cfg(feature = "json")]
mod json;

/// An adapter for [`facet_serialize`] and [`facet_deserialize`]
/// traits to work with [`facet_minecraft`]'s traits and
/// vice versa.
#[derive(Debug, Clone, Copy)]
pub struct FacetAdapter<T>(pub T);

// -------------------------------------------------------------------------------------------------

impl<T> core::ops::Deref for FacetAdapter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> core::ops::DerefMut for FacetAdapter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
