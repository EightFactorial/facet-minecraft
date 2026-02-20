//! [`SerializeBuffer`] trait and implementations.

/// A buffer that can be used for serialization.
pub trait SerializeBuffer: SerializeWriter {
    /// Retrieve the contents written to the buffer.
    ///
    /// This should only return the data that has been written so far,
    /// not the entire capacity of the buffer.
    fn get_content(&self) -> &[u8];
}

/// A writer that can be used for serialization.
pub trait SerializeWriter {
    /// Write data to the writer.
    ///
    /// Returns `true` if the data was written successfully,
    /// or `false` if an error occurred.
    fn write_data(&mut self, data: &[u8]) -> bool;
}

// -------------------------------------------------------------------------------------------------

impl SerializeBuffer for alloc::vec::Vec<u8> {
    fn get_content(&self) -> &[u8] { self.as_slice() }
}
#[cfg(not(feature = "std"))]
impl SerializeWriter for alloc::vec::Vec<u8> {
    fn write_data(&mut self, data: &[u8]) -> bool {
        self.extend_from_slice(data);
        true
    }
}

#[cfg(feature = "std")]
impl<T> SerializeBuffer for std::io::Cursor<T>
where
    Self: std::io::Write,
    T: AsRef<[u8]>,
{
    fn get_content(&self) -> &[u8] {
        let pos = self.position().try_into().unwrap_or_default();
        self.get_ref().as_ref().get(..pos).unwrap_or(&[])
    }
}
#[cfg(feature = "std")]
impl<T: std::io::Write> SerializeWriter for T {
    fn write_data(&mut self, data: &[u8]) -> bool { self.write_all(data).is_ok() }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around a writer that implements
/// [`AsyncWrite`](futures_lite::AsyncWrite).
#[repr(transparent)]
#[cfg(feature = "futures-lite")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuturesLite<T>(pub T);

#[cfg(feature = "futures-lite")]
impl<T: futures_lite::AsyncWrite + Unpin> SerializeWriter for FuturesLite<T> {
    fn write_data(&mut self, data: &[u8]) -> bool {
        use futures_lite::AsyncWriteExt;
        futures_lite::future::block_on(self.0.write_all(data)).is_ok()
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around a writer that implements
/// [`AsyncWrite`](tokio::io::AsyncWrite).
#[repr(transparent)]
#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tokio<T>(pub T);

#[cfg(feature = "tokio")]
impl<T: tokio::io::AsyncWrite + Unpin> SerializeWriter for Tokio<T> {
    fn write_data(&mut self, data: &[u8]) -> bool {
        use tokio::io::AsyncWriteExt;
        tokio::runtime::Handle::current().block_on(self.0.write_all(data)).is_ok()
    }
}
