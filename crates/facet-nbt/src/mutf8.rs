//! [`Mutf8Str`] and [`Mutf8String`] types for MUTF-8 encoded strings.

use alloc::{
    borrow::{Cow, ToOwned},
    vec::Vec,
};
use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use indexmap::Equivalent;
pub use simd_cesu8::mutf8 as simd_cesu8;

/// An MUTF-8–encoded, growable string.
///
/// Equivalent to [`String`](alloc::string::String),
/// but uses `MUTF-8` instead of `UTF-8`.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mutf8String(Vec<u8>);

impl Debug for Mutf8String {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.as_mutf8_str(), f)
    }
}

impl Mutf8String {
    /// Create a [`Mutf8String`] from a UTF-8 byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is not valid UTF-8.
    pub fn try_from_utf8(string: &[u8]) -> Result<Self, simdutf8::basic::Utf8Error> {
        simdutf8::basic::from_utf8(string).map(Self::from_string)
    }

    /// Create a [`Mutf8String`] from a [`str`].
    ///
    /// This will clone the string data.
    #[must_use]
    pub fn from_string(string: &str) -> Self { Self(simd_cesu8::encode(string).into_owned()) }

    /// Convert this [`Mutf8String`] into a [`String`].
    ///
    /// If the string is already valid UTF-8, this will avoid an allocation.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "The input will always be valid MUTF-8")]
    pub fn to_string(&self) -> Cow<'_, str> { simd_cesu8::decode(&self.0).unwrap() }

    /// Create a slice of this [`Mutf8String`].
    #[must_use]
    pub const fn as_mutf8_str(&self) -> &Mutf8Str {
        unsafe { Mutf8Str::from_bytes_unchecked(self.0.as_slice()) }
    }

    /// Create a slice of this [`Mutf8String`].
    #[must_use]
    pub const fn as_mutf8_str_mut(&mut self) -> &mut Mutf8Str {
        unsafe { Mutf8Str::from_bytes_mut_unchecked(self.0.as_mut_slice()) }
    }

    /// Returns `true` if the given byte slice is valid MUTF-8.
    #[inline]
    #[must_use]
    pub fn is_valid_mutf8(bytes: &[u8]) -> bool { Mutf8Str::is_valid_mutf8(bytes) }

    /// Create a [`Mutf8String`] from a byte vector without checking its
    /// validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided byte vector is a valid MUTF-8
    /// encoded string.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Self { Self(bytes) }
}

impl AsRef<Mutf8Str> for Mutf8String {
    fn as_ref(&self) -> &Mutf8Str { self.as_mutf8_str() }
}
impl AsMut<Mutf8Str> for Mutf8String {
    fn as_mut(&mut self) -> &mut Mutf8Str { self.as_mutf8_str_mut() }
}

impl Borrow<Mutf8Str> for Mutf8String {
    fn borrow(&self) -> &Mutf8Str { self.as_mutf8_str() }
}
impl BorrowMut<Mutf8Str> for Mutf8String {
    fn borrow_mut(&mut self) -> &mut Mutf8Str { self.as_mutf8_str_mut() }
}

impl AsRef<[u8]> for Mutf8String {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl Borrow<[u8]> for Mutf8String {
    fn borrow(&self) -> &[u8] { &self.0 }
}

impl Equivalent<str> for Mutf8String {
    fn equivalent(&self, key: &str) -> bool { self.as_bytes() == key.as_bytes() }
}

impl Deref for Mutf8String {
    type Target = Mutf8Str;

    fn deref(&self) -> &Self::Target { self.as_mutf8_str() }
}
impl DerefMut for Mutf8String {
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mutf8_str_mut() }
}

// -------------------------------------------------------------------------------------------------

/// A slice of a MUTF-8 encoded string.
///
/// Equivalent to [`str`], but uses `MUTF-8` instead of `UTF-8`.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mutf8Str([u8]);

impl Debug for Mutf8Str {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "m\"{}\"", self.to_string())
    }
}

impl Mutf8Str {
    /// An empty MUTF-8 string.
    pub const EMPTY: &Self = unsafe { Self::from_bytes_unchecked(&[]) };

    /// Returns the underlying byte slice of this MUTF-8 string.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Returns the length of this MUTF-8 string, in bytes.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if this MUTF-8 string has a length of zero.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Create a [`Mutf8Str`] from a UTF-8 byte slice.
    ///
    /// This will return a [`Cow::Borrowed`] if the input is already valid
    /// MUTF-8, or a [`Cow::Owned`] if it was re-encoded.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is not valid UTF-8.
    pub fn try_from_utf8(bytes: &[u8]) -> Result<Cow<'_, Self>, simdutf8::basic::Utf8Error> {
        simdutf8::basic::from_utf8(bytes).map(Self::from_string)
    }

    /// Create a [`Mutf8Str`] from a UTF-8 byte slice in a `const` context.
    ///
    /// # Panics
    ///
    /// Panics if the input is not valid UTF-8 and MUTF-8.
    #[must_use]
    pub const fn try_from_utf8_const(string: &[u8]) -> &Self {
        let Ok(_) = core::str::from_utf8(string) else { panic!("Input is not valid UTF-8!") };

        let mut index = 0usize;
        while index < string.len() {
            let byte = string[index];
            assert!(
                !(byte == 0x00 || byte & 0b1111_1000 == 0b1111_0000),
                "Invalid MUTF-8 byte found!"
            );
            index += 1;
        }

        unsafe { Self::from_bytes_unchecked(string) }
    }

    /// Create a [`Mutf8Str`] from a [`str`].
    ///
    /// This will return a [`Cow::Borrowed`] if the input is already valid
    /// MUTF-8, or a [`Cow::Owned`] if it was re-encoded.
    #[must_use]
    pub fn from_string(string: &str) -> Cow<'_, Self> {
        match simd_cesu8::encode(string) {
            Cow::Borrowed(mstr) => Cow::Borrowed(unsafe { Self::from_bytes_unchecked(mstr) }),
            Cow::Owned(mstr) => Cow::Owned(Mutf8String(mstr)),
        }
    }

    /// Clone this MUTF-8 string into a [`Mutf8String`].
    #[must_use]
    pub fn to_mutf8_string(&self) -> Mutf8String { Mutf8String(self.0.to_vec()) }

    /// Convert this MUTF-8 string into a [`String`].
    ///
    /// If the string is already valid UTF-8, this will avoid an allocation.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "The input will always be valid MUTF-8")]
    pub fn to_string(&self) -> Cow<'_, str> { simd_cesu8::decode(&self.0).unwrap() }

    /// Returns `true` if the given byte slice is valid MUTF-8.
    #[must_use]
    pub fn is_valid_mutf8(bytes: &[u8]) -> bool {
        simdutf8::basic::from_utf8(bytes).is_ok()
            && ::simd_cesu8::implementation::active::contains_null_or_utf8_4_byte_char_header(bytes)
    }

    /// Create a [`Mutf8Str`] from a byte slice without checking its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided byte slice is a valid MUTF-8
    /// encoded string.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        unsafe { &*(core::ptr::from_ref(bytes) as *const Mutf8Str) }
    }

    /// Create a mutable [`Mutf8Str`] from a mutable byte slice without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided byte slice is a valid MUTF-8
    /// encoded string.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_mut_unchecked(bytes: &mut [u8]) -> &mut Self {
        unsafe { &mut *(core::ptr::from_ref(bytes) as *mut Mutf8Str) }
    }
}

impl AsRef<[u8]> for Mutf8Str {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl Borrow<[u8]> for Mutf8Str {
    fn borrow(&self) -> &[u8] { &self.0 }
}

impl Equivalent<str> for Mutf8Str {
    fn equivalent(&self, key: &str) -> bool { self.as_bytes() == key.as_bytes() }
}

// -------------------------------------------------------------------------------------------------

impl ToOwned for Mutf8Str {
    type Owned = Mutf8String;

    fn to_owned(&self) -> Self::Owned { Mutf8String(self.0.to_vec()) }
}
