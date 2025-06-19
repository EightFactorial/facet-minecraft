/// An error that occurred during deserialization.
pub struct DeserializeError<'mem, 'facet, 'shape> {
    _phantom: core::marker::PhantomData<(&'mem (), &'facet (), &'shape ())>,
}
