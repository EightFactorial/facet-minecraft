//! TODO

use alloc::{borrow::Cow, vec::Vec};

use facet::{Facet, Shape, ShapeLayout, Variant};
use facet_format::{
    DynamicValueEncoding, DynamicValueTag, EnumVariantEncoding, FieldOrdering, FormatSerializer,
    MapEncoding, ScalarValue, SerializeError as FSError, StructFieldMode,
};
use facet_reflect::{FieldItem, Peek};
use uuid::Uuid;

mod buffer;
pub use buffer::SerializeBuffer;

mod error;
pub use error::{SerializeError, SerializeErrorKind};

pub(crate) mod r#trait;
pub use r#trait::Serializable;

/// A function pointer to a serialization function.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct SerializeFn {
    ptr: for<'buffer> fn(
        &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
    ) -> Result<(), SerializeError>,
}

impl SerializeFn {
    /// Create a new [`SerializeFn`].
    #[inline]
    #[must_use]
    pub const fn new(
        ptr: for<'buffer> fn(
            &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
        ) -> Result<(), SerializeError>,
    ) -> Self {
        Self { ptr }
    }

    /// Call the serialization function.
    ///
    /// # Errors
    ///
    /// This function will return an error if serialization fails.
    #[inline]
    pub fn call<'buffer>(
        &self,
        serializer: &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
    ) -> Result<(), SerializeError> {
        (self.ptr)(serializer)
    }
}

impl
    From<
        for<'buffer> fn(
            &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
        ) -> Result<(), SerializeError>,
    > for SerializeFn
{
    #[inline]
    fn from(
        ptr: for<'buffer> fn(
            &mut McSerializer<'buffer, dyn SerializeBuffer + 'buffer>,
        ) -> Result<(), SerializeError>,
    ) -> Self {
        Self::new(ptr)
    }
}

// -------------------------------------------------------------------------------------------------

/// A serializer that implements [`FormatSerializer`].
pub struct McSerializer<'buffer, B: SerializeBuffer + ?Sized> {
    buffer: &'buffer mut B,
    variable_length: bool,
    value_size: usize,
}

impl<'buffer, B: SerializeBuffer + ?Sized> McSerializer<'buffer, B> {
    /// Create a new [`McSerializer`].
    #[inline]
    #[must_use]
    pub const fn new(buffer: &'buffer mut B) -> Self {
        Self { buffer, variable_length: false, value_size: 0 }
    }

    /// Reborrow the serializer with a shorter lifetime.
    #[inline]
    #[must_use]
    pub const fn reborrow<'a>(&'a mut self) -> McSerializer<'a, B> {
        McSerializer {
            buffer: self.buffer,
            variable_length: self.variable_length,
            value_size: self.value_size,
        }
    }

    /// Reborrow the serializer over a dynamic buffer.
    #[inline]
    #[must_use]
    pub const fn as_dyn<'a>(&'a mut self) -> McSerializer<'a, dyn SerializeBuffer + 'a>
    where
        B: Sized + 'a,
    {
        McSerializer {
            buffer: self.buffer,
            variable_length: self.variable_length,
            value_size: self.value_size,
        }
    }

    /// Consume the serializer and return the buffer reference.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> &'buffer mut B { self.buffer }
}

impl<B: SerializeBuffer> FormatSerializer for McSerializer<'_, B> {
    type Error = SerializeError;

    fn struct_metadata(&mut self, shape: &Shape) -> Result<(), Self::Error> {
        if self.variable_length && !shape.is_transparent() {
            Err(SerializeError::variable_length(shape))
        } else {
            Ok(())
        }
    }

    fn begin_struct(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn end_struct(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn variant_metadata(&mut self, variant: &'static Variant) -> Result<(), Self::Error> {
        if let Some(disciminant) = variant.discriminant {
            self.scalar_variable(ScalarValue::I64(disciminant), true)
        } else {
            Err(SerializeError::new(SerializeErrorKind::DiscriminantMissing))
        }
    }

    fn field_metadata_with_value(
        &mut self,
        field: &FieldItem,
        _value: Peek<'_, '_>,
    ) -> Result<bool, Self::Error> {
        if let Some(field) = field.field.as_ref()
            && let Some(attr) = field.get_attr(Some("mc"), "serialize")
            && let Some(serialize) = attr.get_as::<SerializeFn>()
        {
            serialize.call(&mut self.as_dyn()).map(|()| true)
        } else {
            Ok(false)
        }
    }

    fn field_metadata(&mut self, field: &FieldItem) -> Result<(), Self::Error> {
        if let Some(field) = field.field.as_ref() {
            if field.has_attr(Some("mc"), "variable") {
                self.variable_length = true;
            }

            if let ShapeLayout::Sized(layout) = field.shape().layout {
                self.value_size = layout.size();
            }
        }

        Ok(())
    }

    fn field_key(&mut self, _: &str) -> Result<(), Self::Error> { Ok(()) }

    fn scalar(&mut self, val: ScalarValue<'_>) -> Result<(), Self::Error> {
        let variable_length = core::mem::take(&mut self.variable_length);
        self.scalar_variable(val, variable_length)
    }

    fn is_self_describing(&self) -> bool { false }

    fn serialize_opaque_scalar(
        &mut self,
        shape: &'static Shape,
        value: Peek<'_, '_>,
    ) -> Result<bool, Self::Error> {
        if shape.is_type::<Uuid>() {
            self.scalar_variable(ScalarValue::U128(value.get::<Uuid>().unwrap().as_u128()), false)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn serialize_byte_sequence(&mut self, bytes: &[u8]) -> Result<bool, Self::Error> {
        self.scalar_variable(ScalarValue::Bytes(Cow::Borrowed(bytes)), false)?;
        Ok(true)
    }

    fn serialize_byte_array(&mut self, bytes: &[u8]) -> Result<bool, Self::Error> {
        if self.buffer.extend_buffer(bytes) {
            Ok(true)
        } else {
            Err(SerializeError::new(SerializeErrorKind::BufferError))
        }
    }

    fn begin_seq_with_len(&mut self, len: usize) -> Result<(), Self::Error> {
        self.scalar_variable(ScalarValue::U64(len as u64), true)
    }

    fn begin_seq(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn end_seq(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn begin_map_with_len(&mut self, len: usize) -> Result<(), Self::Error> {
        self.scalar_variable(ScalarValue::U64(len as u64), true)
    }

    fn end_map(&mut self) -> Result<(), Self::Error> { Ok(()) }

    fn begin_option_some(&mut self) -> Result<(), Self::Error> {
        self.scalar_variable(ScalarValue::Bool(true), false)
    }

    fn serialize_none(&mut self) -> Result<(), Self::Error> {
        self.scalar_variable(ScalarValue::Bool(false), false)
    }

    // ---------------------------------------------------------------------------------------------

    fn preferred_field_order(&self) -> FieldOrdering { FieldOrdering::Declaration }

    fn struct_field_mode(&self) -> StructFieldMode { StructFieldMode::Unnamed }

    fn enum_variant_encoding(&self) -> EnumVariantEncoding { EnumVariantEncoding::Index }

    fn map_encoding(&self) -> MapEncoding { MapEncoding::Pairs }

    fn dynamic_value_encoding(&self) -> DynamicValueEncoding { DynamicValueEncoding::Tagged }

    fn dynamic_value_tag(&mut self, _tag: DynamicValueTag) -> Result<(), Self::Error> { Ok(()) }
}

impl<B: SerializeBuffer + ?Sized> McSerializer<'_, B> {
    fn scalar_variable(&mut self, val: ScalarValue, variable: bool) -> Result<(), SerializeError> {
        if match (val, variable) {
            (ScalarValue::Unit | ScalarValue::Null, false) => true,
            (ScalarValue::Bool(v), false) => {
                let byte = v as u8;
                self.buffer.extend_buffer(&[byte])
            }

            (ScalarValue::I64(v), false) => {
                let bytes = v.to_le_bytes();
                self.buffer.extend_buffer(&bytes[..self.value_size])
            }
            (ScalarValue::U64(v), false) => {
                let bytes = v.to_le_bytes();
                self.buffer.extend_buffer(&bytes[..self.value_size])
            }
            (ScalarValue::I64(v), true) => {
                let mut buffer = [0; _];
                let len = Self::var_u64(v as u64, &mut buffer);
                self.buffer.extend_buffer(&buffer[..len])
            }
            (ScalarValue::U64(v), true) => {
                let mut buffer = [0; _];
                let len = Self::var_u64(v, &mut buffer);
                self.buffer.extend_buffer(&buffer[..len])
            }

            (ScalarValue::I128(v), false) => {
                let bytes = v.to_le_bytes();
                self.buffer.extend_buffer(&bytes[..self.value_size])
            }
            (ScalarValue::U128(v), false) => {
                let bytes = v.to_le_bytes();
                self.buffer.extend_buffer(&bytes[..self.value_size])
            }
            (ScalarValue::I128(v), true) => {
                let mut buffer = [0; _];
                let len = Self::var_u128(v as u128, &mut buffer);
                self.buffer.extend_buffer(&buffer[..len])
            }
            (ScalarValue::U128(v), true) => {
                let mut buffer = [0; _];
                let len = Self::var_u128(v, &mut buffer);
                self.buffer.extend_buffer(&buffer[..len])
            }

            (ScalarValue::F64(v), false) => {
                let bytes = v.to_le_bytes();
                self.buffer.extend_buffer(&bytes[..self.value_size])
            }
            (ScalarValue::Str(v), false) => {
                let mut buffer = [0; _];
                let len = Self::var_u64(v.len() as u64, &mut buffer);
                self.buffer.extend_buffer(&buffer[..len]) && self.buffer.extend_buffer(v.as_bytes())
            }
            (ScalarValue::Bytes(v), false) => {
                let mut buffer = [0; _];
                let len = Self::var_u64(v.len() as u64, &mut buffer);
                self.buffer.extend_buffer(&buffer[..len]) && self.buffer.extend_buffer(v.as_ref())
            }

            // Unsupported
            (ScalarValue::Char(_), _) => {
                return Err(SerializeError::unsupported_type::<char>());
            }
            (val, true) => {
                return Err(SerializeError::variable_length_scalar(&val));
            }
        } {
            Ok(())
        } else {
            Err(SerializeError::new(SerializeErrorKind::BufferError))
        }
    }

    fn var_u64(mut v: u64, buf: &mut [u8; 10]) -> usize {
        let mut byte;
        let mut count = 0;
        while (v != 0 || count == 0) && count < 10 {
            byte = (v & 0b0111_1111) as u8;
            v = (v >> 7) & (u64::MAX >> 6);
            if v != 0 {
                byte |= 0b1000_0000;
            }
            buf[count] = byte;
            count += 1;
        }
        count
    }

    fn var_u128(mut v: u128, buf: &mut [u8; 19]) -> usize {
        let mut byte;
        let mut count = 0;
        while (v != 0 || count == 0) && count < 19 {
            byte = (v & 0b0111_1111) as u8;
            v = (v >> 7) & (u128::MAX >> 6);
            if v != 0 {
                byte |= 0b1000_0000;
            }
            buf[count] = byte;
            count += 1;
        }
        count
    }
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value of type `T` into a byte vector.
///
/// # Errors
///
/// This function will return an error if serialization fails.
pub fn to_vec<'facet, T: Serializable<'facet> + ?Sized>(
    value: &T,
) -> Result<Vec<u8>, FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    let mut buffer = T::SERIALIZE_HINT
        .maximum()
        .or(T::SERIALIZE_HINT.minimum())
        .map_or_else(Vec::new, Vec::with_capacity);
    to_buffer::<T, Vec<u8>>(value, &mut buffer)?;
    Ok(buffer)
}

/// Serialize a value of type `T` into a buffer,
/// returning a slice containing the serialized data.
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or if the buffer cannot be written to.
pub fn to_buffer<'output, 'facet, T: Serializable<'facet> + ?Sized, B: SerializeBuffer>(
    value: &T,
    buffer: &'output mut B,
) -> Result<&'output [u8], FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    let mut format = McSerializer::new(buffer);
    facet_format::serialize_root(&mut format, Peek::new(value))?;
    Ok(buffer.get_content())
}

// -------------------------------------------------------------------------------------------------

/// Serialize a value of type `T` into a [`Writer`](std::io::Write).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "streaming")]
pub fn to_writer<'facet, T: Serializable<'facet> + ?Sized, W: std::io::Write>(
    value: &T,
    writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not
    // serializable!") };

    std::io::Write::write_all(writer, to_vec::<T>(value)?.as_slice())
        .map_err(|err| FSError::Backend(SerializeError::from(err)))
}

/// Serialize a value of type `T` into an asynchronous
/// [`AsyncWrite`](futures_lite::AsyncWrite).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "futures-lite")]
pub async fn to_async_writer<
    'facet,
    T: Serializable<'facet> + ?Sized,
    W: futures_lite::AsyncWrite + Unpin,
>(
    value: &T,
    writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not
    // serializable!") };

    futures_lite::AsyncWriteExt::write_all(writer, to_vec::<T>(value)?.as_slice())
        .await
        .map_err(|err| FSError::Backend(SerializeError::from(err)))
}

/// Serialize a value of type `T` into an asynchronous
/// [`AsyncWrite`](tokio::io::AsyncWrite).
///
/// # Errors
///
/// This function will return an error if serialization fails,
/// or the writer encounters an I/O error.
#[cfg(feature = "tokio")]
pub async fn to_tokio_writer<
    'facet,
    T: Serializable<'facet> + ?Sized,
    W: tokio::io::AsyncWrite + Unpin,
>(
    value: &T,
    writer: &mut W,
) -> Result<(), FSError<SerializeError>> {
    // const { assert!(T::SERIALIZABLE.possible(), "This type is not serializable!")
    // };

    tokio::io::AsyncWriteExt::write_all(writer, to_vec::<T>(value)?.as_slice())
        .await
        .map_err(|err| FSError::Backend(SerializeError::from(err)))
}
