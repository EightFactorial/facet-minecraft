use alloc::{borrow::Cow, vec::Vec};
use core::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, TryCastError, Unalign, Unaligned};

/// A clone-on-write smart pointer to a byte buffer.
///
/// Guaranteed to contain a valid representation of `T`, but is not necessarily
/// aligned. May require copying data to access the inner value.
#[repr(transparent)]
pub struct ByteCow<'a, T: ?Sized>(Cow<'a, [u8]>, PhantomData<T>);

impl<T: ?Sized> ByteCow<'_, T> {
    /// Get a reference to the inner byte slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        match &self.0 {
            Cow::Borrowed(b) => b,
            Cow::Owned(b) => b.as_slice(),
        }
    }

    /// Create a new [`ByteCow`] with a shorter lifetime.
    #[must_use]
    pub const fn reborrow(&self) -> ByteCow<'_, T> {
        ByteCow(Cow::Borrowed(self.as_slice()), PhantomData)
    }

    /// Create an owned [`ByteCow`] from a reference, cloning the data.
    #[must_use]
    pub fn to_owned(&self) -> ByteCow<'static, T> { self.clone().into_owned() }

    /// Create an owned [`ByteCow`], cloning the data if it is not owned.
    #[must_use]
    pub fn into_owned(self) -> ByteCow<'static, T> {
        match self.0 {
            Cow::Borrowed(b) => ByteCow(Cow::Owned(b.to_vec()), PhantomData),
            Cow::Owned(b) => ByteCow(Cow::Owned(b), PhantomData),
        }
    }

    /// Create a new, empty [`ByteCow`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is never used without being set to
    /// a valid value first.
    #[must_use]
    pub const unsafe fn empty() -> ByteCow<'static, T> {
        ByteCow(Cow::Owned(Vec::new()), PhantomData)
    }

    /// Create a new [`ByteCow`] from a byte slice without checking if it is
    /// valid.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the byte slice is a valid representation of
    /// `T`.
    #[must_use]
    pub const unsafe fn from_slice_unchecked(bytes: &[u8]) -> ByteCow<'_, T> {
        ByteCow(Cow::Borrowed(bytes), PhantomData)
    }

    /// Create a new [`ByteCow`] from a byte vector without checking if it is
    /// valid.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the byte vector is a valid representation of
    /// `T`.
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> ByteCow<'static, T> {
        ByteCow(Cow::Owned(bytes), PhantomData)
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Immutable + IntoBytes + ?Sized> ByteCow<'_, T> {
    /// Create a new [`ByteCow`] from a `T`.
    ///
    /// Clones the raw bytes of `T` into an owned buffer.
    #[must_use]
    pub fn new(value: &T) -> ByteCow<'static, T> {
        // Safety: `T` implements `IntoBytes`, so the slice is valid.
        unsafe { Self::from_bytes_unchecked(value.as_bytes().to_vec()) }
    }

    /// Create a new [`ByteCow`] from a reference to `T`.
    #[must_use]
    pub fn new_ref(value: &T) -> ByteCow<'_, T> {
        // Safety: `T` implements `IntoBytes`, so the slice is valid.
        unsafe { Self::from_slice_unchecked(value.as_bytes()) }
    }

    /// Set the inner value to a new `T`.
    ///
    /// Clones the raw bytes of `T` into the inner buffer.
    pub fn set(&mut self, value: &T) {
        let slice = self.0.to_mut();
        slice.clear();
        slice.extend_from_slice(value.as_bytes());
    }
}

impl<T: Immutable + IntoBytes> From<T> for ByteCow<'_, T> {
    fn from(value: T) -> Self { Self::new(&value) }
}
impl<'a, T: Immutable + IntoBytes + ?Sized> From<&'a T> for ByteCow<'a, T> {
    fn from(value: &'a T) -> Self { Self::new_ref(value) }
}

// -------------------------------------------------------------------------------------------------

impl<T: Immutable + FromBytes + ?Sized> ByteCow<'_, T> {
    /// Get a reference to the inner `T`.
    ///
    /// Requires that `T` is [`Unaligned`].
    #[must_use]
    #[expect(clippy::should_implement_trait, reason = "Different function signature")]
    #[expect(clippy::missing_panics_doc, reason = "This should be guaranteed not to panic")]
    pub fn as_ref(&self) -> &T
    where
        T: KnownLayout + Unaligned,
    {
        Self::try_as_ref(self).unwrap()
    }

    /// Get a mutable reference to the inner `T`.
    ///
    /// Requires that `T` is [`Unaligned`].
    #[must_use]
    #[expect(clippy::should_implement_trait, reason = "Different function signature")]
    #[expect(clippy::missing_panics_doc, reason = "This should be guaranteed not to panic")]
    pub fn as_mut(&mut self) -> &mut T
    where
        T: KnownLayout + IntoBytes + Unaligned,
    {
        Self::try_as_mut(self).unwrap()
    }

    /// Try to get a reference to the inner `T`.
    ///
    /// # Errors
    ///
    /// Returns an error if the inner bytes are not aligned.
    pub fn try_as_ref(&self) -> Result<&T, TryCastError<&[u8], T>>
    where
        T: KnownLayout,
    {
        T::try_ref_from_bytes(&self.0)
    }

    /// Try to get a mutable reference to the inner `T`.
    ///
    /// # Errors
    ///
    /// Returns an error if the inner bytes are not aligned.
    pub fn try_as_mut(&mut self) -> Result<&mut T, TryCastError<&mut [u8], T>>
    where
        T: KnownLayout + IntoBytes,
    {
        T::try_mut_from_bytes(self.0.to_mut())
    }

    /// Create an owned `T` from the inner bytes.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should be guaranteed not to panic")]
    pub fn get(&self) -> T
    where
        T: Sized,
    {
        T::read_from_bytes(&self.0).unwrap()
    }

    /// Get an unaligned reference to the inner `T`.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should be guaranteed not to panic")]
    pub fn get_unaligned(&self) -> Unalign<T>
    where
        T: Sized,
    {
        Unalign::<T>::read_from_bytes(&self.0).unwrap()
    }
}

impl<T: Immutable + FromBytes + KnownLayout + Unaligned + ?Sized> Deref for ByteCow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { self.as_ref() }
}
impl<T: Immutable + FromBytes + KnownLayout + IntoBytes + Unaligned + ?Sized> DerefMut
    for ByteCow<'_, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
}

// -------------------------------------------------------------------------------------------------

impl<T: Immutable + FromBytes + Sized> ByteCow<'_, [T]> {
    /// Create an owned `T` from the inner bytes at the given index.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should be guaranteed not to panic")]
    pub fn get_index(&self, index: usize) -> Option<T> {
        let start = index.checked_mul(core::mem::size_of::<T>())?;
        let end = start.checked_add(core::mem::size_of::<T>())?;
        self.0.get(start..end).map(|b| T::read_from_bytes(b).unwrap())
    }

    /// Create a [`Vec<T>`] from the inner bytes.
    ///
    /// Clones each element into the vector.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should be guaranteed not to panic")]
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.0.len() / core::mem::size_of::<T>());
        for chunk in self.0.chunks_exact(core::mem::size_of::<T>()) {
            vec.push(T::read_from_bytes(chunk).unwrap());
        }
        vec
    }

    /// Create a [`Vec<T>`] from the inner bytes.
    ///
    /// Clones each element if they are not owned.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should be guaranteed not to panic")]
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.0.len() / core::mem::size_of::<T>());
        for chunk in self.0.into_owned().chunks_exact(core::mem::size_of::<T>()) {
            vec.push(T::read_from_bytes(chunk).unwrap());
        }
        vec
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Debug + Immutable + FromBytes + Sized> Debug for ByteCow<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ByteCow").field(&self.get()).finish()
    }
}
impl<T: Debug + Immutable + FromBytes + Sized> Debug for ByteCow<'_, [T]> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ByteCow").field(&self.to_vec()).finish()
    }
}

impl<T: ?Sized> Clone for ByteCow<'_, T> {
    fn clone(&self) -> Self { ByteCow(self.0.clone(), PhantomData) }
}

impl<T: ?Sized> PartialEq for ByteCow<'_, T> {
    fn eq(&self, other: &Self) -> bool { self.as_slice() == other.as_slice() }
}
impl<T: ?Sized> Eq for ByteCow<'_, T> {}

impl<T: ?Sized> Hash for ByteCow<'_, T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.as_slice().hash(state) }
}
