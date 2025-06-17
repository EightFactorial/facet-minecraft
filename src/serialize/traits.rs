#![allow(clippy::inline_always)]

use crate::{adapter::WriteAdapter, serialize::McSerializer};

/// A serializer for Minecraft protocol data.
#[expect(clippy::missing_errors_doc)]
pub trait Serializer<'shape> {
    /// The error type returned by serialization methods
    type Error;

    /// Serialize a unit value `()`.
    fn serialize_unit(&mut self) -> Result<(), Self::Error>;

    /// Serialize an unsigned 8-bit integer.
    fn serialize_u8(&mut self, value: u8) -> Result<(), Self::Error>;
    /// Serialize an unsigned 16-bit integer.
    fn serialize_u16(&mut self, value: u16) -> Result<(), Self::Error>;
    /// Serialize an unsigned 32-bit integer.
    fn serialize_u32(&mut self, value: u32) -> Result<(), Self::Error>;
    /// Serialize an unsigned 64-bit integer.
    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error>;
    /// Serialize an unsigned 128-bit integer.
    fn serialize_u128(&mut self, value: u128) -> Result<(), Self::Error>;

    /// Serialize a `usize` integer.
    #[inline(always)]
    fn serialize_usize(&mut self, value: usize) -> Result<(), Self::Error> {
        self.serialize_u64(value as u64)
    }

    /// Serialize a signed 8-bit integer.
    fn serialize_i8(&mut self, value: i8) -> Result<(), Self::Error>;
    /// Serialize a signed 16-bit integer.
    fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error>;
    /// Serialize a signed 32-bit integer.
    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error>;
    /// Serialize a signed 64-bit integer.
    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error>;
    /// Serialize a signed 128-bit integer.
    fn serialize_i128(&mut self, value: i128) -> Result<(), Self::Error>;

    /// Serialize an `isize` integer.
    #[inline(always)]
    fn serialize_isize(&mut self, value: isize) -> Result<(), Self::Error> {
        self.serialize_i64(value as i64)
    }

    /// Serialize a single-precision floating-point value.
    fn serialize_f32(&mut self, value: f32) -> Result<(), Self::Error>;
    /// Serialize a double-precision floating-point value.
    fn serialize_f64(&mut self, value: f64) -> Result<(), Self::Error>;

    /// Serialize a boolean value.
    #[inline(always)]
    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        self.serialize_u8(u8::from(value))
    }

    /// Serialize a UTF-8 string slice.
    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error>;
    /// Serialize a raw byte slice.
    fn serialize_bytes(&mut self, value: &[u8]) -> Result<(), Self::Error>;
}

/// An extension trait for [`Serializer`] that provides
/// variable-length serialization methods.
#[expect(clippy::missing_errors_doc)]
pub trait SerializerExt<'shape>: Serializer<'shape> {
    /// Serialize a variable-length unsigned 16-bit integer.
    fn serialize_var_u16(&mut self, val: u16) -> Result<(), Self::Error>;
    /// Serialize a variable-length unsigned 32-bit integer.
    fn serialize_var_u32(&mut self, val: u32) -> Result<(), Self::Error>;
    /// Serialize a variable-length unsigned 64-bit integer.
    fn serialize_var_u64(&mut self, val: u64) -> Result<(), Self::Error>;
    /// Serialize a variable-length unsigned 128-bit integer.
    fn serialize_var_u128(&mut self, val: u128) -> Result<(), Self::Error>;

    /// Serialize a variable-length `usize` integer.
    #[inline(always)]
    fn serialize_var_usize(&mut self, val: usize) -> Result<(), Self::Error> {
        self.serialize_var_u64(val as u64)
    }

    /// Serialize a variable-length signed 16-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i16(&mut self, val: i16) -> Result<(), Self::Error> {
        self.serialize_var_u16(val as u16)
    }
    /// Serialize a variable-length signed 32-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i32(&mut self, val: i32) -> Result<(), Self::Error> {
        self.serialize_var_u32(val as u32)
    }
    /// Serialize a variable-length signed 64-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i64(&mut self, val: i64) -> Result<(), Self::Error> {
        self.serialize_var_u64(val as u64)
    }
    /// Serialize a variable-length signed 128-bit integer.
    #[inline(always)]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_var_i128(&mut self, val: i128) -> Result<(), Self::Error> {
        self.serialize_var_u128(val as u128)
    }

    /// Serialize a variable-length signed `isize` integer.
    #[inline(always)]
    fn serialize_var_isize(&mut self, val: isize) -> Result<(), Self::Error> {
        self.serialize_var_i64(val as i64)
    }
}

// -------------------------------------------------------------------------------------------------

#[expect(clippy::elidable_lifetime_names)]
impl<'shape, W: WriteAdapter> Serializer<'shape> for McSerializer<W> {
    type Error = W::Error;

    fn serialize_unit(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn serialize_u8(&mut self, val: u8) -> Result<(), Self::Error> { self.write(&[val]) }

    fn serialize_u16(&mut self, val: u16) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_u32(&mut self, val: u32) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_u64(&mut self, val: u64) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_u128(&mut self, val: u128) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    #[inline(always)]
    #[expect(clippy::cast_sign_loss)]
    fn serialize_i8(&mut self, val: i8) -> Result<(), Self::Error> { self.serialize_u8(val as u8) }

    fn serialize_i16(&mut self, val: i16) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_i32(&mut self, val: i32) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_i64(&mut self, val: i64) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_i128(&mut self, val: i128) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_f32(&mut self, val: f32) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_f64(&mut self, val: f64) -> Result<(), Self::Error> {
        self.write(&val.to_le_bytes())
    }

    fn serialize_str(&mut self, val: &str) -> Result<(), Self::Error> {
        self.serialize_var_usize(val.len())?;
        self.serialize_bytes(val.as_bytes())
    }

    fn serialize_bytes(&mut self, val: &[u8]) -> Result<(), Self::Error> { self.write(val) }
}

impl<W: WriteAdapter> SerializerExt<'_> for McSerializer<W> {
    #[expect(unused_assignments)]
    fn serialize_var_u16(&mut self, mut val: u16) -> Result<(), Self::Error> {
        let mut byte = 0u8;
        let mut count = 0u8;
        while (val != 0 || count == 0) && count < 3 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u16::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }

    #[expect(unused_assignments)]
    fn serialize_var_u32(&mut self, mut val: u32) -> Result<(), Self::Error> {
        let mut count = 0u8;
        let mut byte = 0u8;
        while (val != 0 || count == 0) && count < 5 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u32::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }

    #[expect(unused_assignments)]
    fn serialize_var_u64(&mut self, mut val: u64) -> Result<(), Self::Error> {
        let mut byte = 0u8;
        let mut count = 0u8;
        while (val != 0 || count == 0) && count < 10 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u64::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }

    #[expect(unused_assignments)]
    fn serialize_var_u128(&mut self, mut val: u128) -> Result<(), Self::Error> {
        let mut byte = 0u8;
        let mut count = 0u8;
        while (val != 0 || count == 0) && count < 19 {
            byte = (val & 0b0111_1111) as u8;
            val = (val >> 7) & (u128::MAX >> 6);
            if val != 0 {
                byte |= 0b1000_0000;
            }
            count += 1;
            self.serialize_u8(byte)?;
        }
        Ok(())
    }
}
