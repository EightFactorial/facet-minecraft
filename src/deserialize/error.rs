/// An error that occurred during deserialization.
pub struct DeserializeError<'input, 'facet, 'shape> {
    _phantom: core::marker::PhantomData<(&'input (), &'facet (), &'shape ())>,
}
