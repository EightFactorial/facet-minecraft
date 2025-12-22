use alloc::vec::Vec;

/// A buffer that can be used for serialization.
pub trait SerializeBuffer {
    /// Extend the buffer with the given data.
    ///
    /// Returns `true` if the data was successfully added or `false` otherwise.
    fn extend_buffer(&mut self, data: &[u8]) -> bool;

    /// Retrieve the contents written to the buffer.
    ///
    /// This should only return the data that has been written so far,
    /// not the entire capacity of the buffer.
    fn get_content(&self) -> &[u8];
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
impl<T> SerializeBuffer for std::io::Cursor<T>
where
    Self: std::io::Write,
    T: AsRef<[u8]>,
{
    fn extend_buffer(&mut self, data: &[u8]) -> bool {
        std::io::Write::write_all(self, data).is_ok()
    }

    fn get_content(&self) -> &[u8] {
        let pos = self.position().try_into().unwrap_or_default();
        self.get_ref().as_ref().get(..pos).unwrap_or(&[])
    }
}

impl SerializeBuffer for Vec<u8> {
    fn extend_buffer(&mut self, data: &[u8]) -> bool {
        self.extend_from_slice(data);
        true
    }

    fn get_content(&self) -> &[u8] { self.as_slice() }
}
