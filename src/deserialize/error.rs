/// An error that occurred during deserialization.
#[derive(Debug)]
pub struct DeserializeError<'input, 'facet, 'shape> {
    _phantom: core::marker::PhantomData<(&'input (), &'facet (), &'shape ())>,
}
