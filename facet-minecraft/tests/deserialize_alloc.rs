//! TODO
#![expect(dead_code, reason = "WIP")]

use facet_format::DeserializeError as FDError;
use facet_minecraft::{Deserializable, deserialize::DeserializeError};

#[repr(transparent)]
struct TestCursor(&'static [u8]);

impl TestCursor {
    fn read<'de, T: Deserializable<'de>>(&mut self) -> Result<T, FDError<DeserializeError>> {
        let (value, remaining) = T::from_slice_borrowed(self.0)?;
        self.0 = remaining;
        Ok(value)
    }
}

// -------------------------------------------------------------------------------------------------
