//! [MUTF-8](https://docs.oracle.com/javase/8/docs/api/java/io/DataInput.html) encoded strings.

use alloc::{borrow::Cow, string::String, vec::Vec};

/// A MUTF-8 encoded, growable string.
///
/// Similar to a [`String`], but uses MUTF-8 encoding.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mutf8String(Vec<u8>);

impl Mutf8String {
    /// Create a new [`Mutf8String`] from a byte vector.
    ///
    /// # Warning
    /// This requires that the byte vector is a valid MUTF-8 string.
    #[inline]
    #[must_use]
    pub const fn new_raw(bytes: Vec<u8>) -> Self { Self(bytes) }

    /// Create a new [`Mutf8String`] from a string slice.
    #[must_use]
    pub fn new_str(string: &str) -> Self { Mutf8Str::new_str(string).into_owned() }
}

impl Mutf8String {
    /// Create a [`Mutf8Str`] for this [`Mutf8String`].
    #[inline]
    #[must_use]
    pub const fn as_mutf8_str(&self) -> &Mutf8Str { Mutf8Str::new_raw(self.0.as_slice()) }

    /// Create a mutable [`Mutf8Str`] for this [`Mutf8String`].
    #[inline]
    #[must_use]
    pub const fn as_mutf8_str_mut(&mut self) -> &mut Mutf8Str {
        Mutf8Str::new_raw_mut(self.0.as_mut_slice())
    }

    /// Convert the MUTF-8 string to a UTF-8 [`str`],
    /// replacing invalid sequences with the Unicode replacement character (�).
    ///
    /// See [`simd_cesu8::decode_lossy`] for more details.
    #[inline]
    #[must_use]
    pub fn to_str_lossy(&self) -> Cow<'_, str> { self.as_mutf8_str().to_str_lossy() }

    /// Convert the MUTF-8 string to a UTF-8 [`str`],
    /// returning an error if the conversion fails.
    ///
    /// See [`simd_cesu8::decode`] for more details.
    #[inline]
    #[must_use]
    pub fn try_as_str(&self) -> Result<Cow<'_, str>, simd_cesu8::DecodingError> {
        self.as_mutf8_str().try_as_str()
    }

    /// Convert the MUTF-8 string to a UTF-8 [`String`],
    /// replacing invalid sequences with the Unicode replacement character (�).
    ///
    /// See [`simd_cesu8::decode_lossy`] for more details.
    #[inline]
    #[must_use]
    pub fn to_string_lossy(&self) -> String { self.as_mutf8_str().to_string_lossy() }

    /// Convert the MUTF-8 string to a UTF-8 [`String`],
    /// returning an error if the conversion fails.
    ///
    /// See [`simd_cesu8::decode`] for more details.
    #[inline]
    #[must_use]
    pub fn try_as_string(&self) -> Result<String, simd_cesu8::DecodingError> {
        self.as_mutf8_str().try_as_string()
    }
}

// -------------------------------------------------------------------------------------------------

impl core::convert::AsRef<Mutf8Str> for Mutf8String {
    fn as_ref(&self) -> &Mutf8Str { Mutf8Str::new_raw(&self.0) }
}
impl core::convert::AsMut<Mutf8Str> for Mutf8String {
    fn as_mut(&mut self) -> &mut Mutf8Str { Mutf8Str::new_raw_mut(&mut self.0) }
}

impl core::borrow::Borrow<Mutf8Str> for Mutf8String {
    fn borrow(&self) -> &Mutf8Str { Mutf8Str::new_raw(&self.0) }
}
impl core::borrow::BorrowMut<Mutf8Str> for Mutf8String {
    fn borrow_mut(&mut self) -> &mut Mutf8Str { Mutf8Str::new_raw_mut(&mut self.0) }
}

impl core::ops::Deref for Mutf8String {
    type Target = Mutf8Str;

    fn deref(&self) -> &Self::Target { Mutf8Str::new_raw(&self.0) }
}
impl core::ops::DerefMut for Mutf8String {
    fn deref_mut(&mut self) -> &mut Self::Target { Mutf8Str::new_raw_mut(&mut self.0) }
}

impl core::convert::TryFrom<Mutf8String> for String {
    type Error = simd_cesu8::DecodingError;

    #[inline]
    fn try_from(value: Mutf8String) -> Result<Self, Self::Error> { value.try_as_string() }
}

// -------------------------------------------------------------------------------------------------

/// A slice of a [`Mutf8String`].
///
/// Similar to a [`str`], but uses MUTF-8 encoding.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Mutf8Str([u8]);

impl Mutf8Str {
    /// Create a new [`Mutf8Str`] from a byte slice.
    ///
    /// # Warning
    /// This requires that the byte slice is a valid MUTF-8 string.
    #[inline]
    #[must_use]
    pub const fn new_raw(bytes: &[u8]) -> &Self {
        // SAFETY: `Mutf8Str` is a transparent wrapper around a byte slice
        unsafe { &*(core::ptr::from_ref::<[u8]>(bytes) as *const Mutf8Str) }
    }

    /// Create a new mutable [`Mutf8Str`] from a mutable byte slice.
    ///
    /// # Warning
    /// This requires that the byte slice is a valid MUTF-8 string.
    #[inline]
    #[must_use]
    pub const fn new_raw_mut(bytes: &mut [u8]) -> &mut Self {
        // SAFETY: `Mutf8Str` is a transparent wrapper around a byte slice
        unsafe { &mut *(core::ptr::from_mut::<[u8]>(bytes) as *mut Mutf8Str) }
    }

    /// Create a new [`Mutf8Str`] or [`Mutf8String`] from a string slice.
    ///
    /// See [`simd_cesu8::encode`] for more details.
    #[must_use]
    pub fn new_str(string: &str) -> Cow<'_, Mutf8Str> {
        match simd_cesu8::encode(string) {
            Cow::Borrowed(val) => Cow::Borrowed(Self::new_raw(val)),
            Cow::Owned(val) => Cow::Owned(Mutf8String::new_raw(val)),
        }
    }

    /// Get the raw inner byte slice of the [`Mutf8Str`].
    #[inline]
    #[must_use]
    pub const fn as_raw_bytes(&self) -> &[u8] { &self.0 }

    /// Get the raw inner mutable byte slice of the [`Mutf8Str`].
    #[inline]
    #[must_use]
    pub const fn as_raw_bytes_mut(&mut self) -> &mut [u8] { &mut self.0 }
}

impl Mutf8Str {
    /// Returns `true` if the string has a length of 0.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Returns the length of the string in bytes.
    #[inline]
    #[must_use]
    pub const fn len_bytes(&self) -> usize { self.0.len() }

    /// Convert the MUTF-8 string to a UTF-8 [`str`],
    /// replacing invalid sequences with the Unicode replacement character (�).
    ///
    /// See [`simd_cesu8::decode_lossy`] for more details.
    #[must_use]
    pub fn to_str_lossy(&self) -> Cow<'_, str> { simd_cesu8::decode_lossy(&self.0) }

    /// Convert the MUTF-8 string to a UTF-8 [`str`],
    /// returning an error if the conversion fails.
    ///
    /// See [`simd_cesu8::decode`] for more details.
    #[must_use]
    pub fn try_as_str(&self) -> Result<Cow<'_, str>, simd_cesu8::DecodingError> {
        simd_cesu8::decode(&self.0)
    }

    /// Convert the MUTF-8 string to a UTF-8 [`String`].
    ///
    /// See [`simd_cesu8::decode_lossy`] for more details.
    #[must_use]
    pub fn to_string_lossy(&self) -> String { self.to_str_lossy().into_owned() }

    /// Convert the MUTF-8 string to a UTF-8 [`String`],
    /// returning an error if the conversion fails.
    ///
    /// See [`simd_cesu8::decode`] for more details.
    #[must_use]
    pub fn try_as_string(&self) -> Result<String, simd_cesu8::DecodingError> {
        self.try_as_str().map(Cow::into_owned)
    }
}

// -------------------------------------------------------------------------------------------------

impl alloc::borrow::ToOwned for Mutf8Str {
    type Owned = Mutf8String;

    fn to_owned(&self) -> Self::Owned { Mutf8String::new_raw(self.0.to_vec()) }
}

impl core::convert::TryFrom<&Mutf8Str> for String {
    type Error = simd_cesu8::DecodingError;

    #[inline]
    fn try_from(value: &Mutf8Str) -> Result<Self, Self::Error> { value.try_as_string() }
}

impl core::convert::TryFrom<&mut Mutf8Str> for String {
    type Error = simd_cesu8::DecodingError;

    #[inline]
    fn try_from(value: &mut Mutf8Str) -> Result<Self, Self::Error> { value.try_as_string() }
}
