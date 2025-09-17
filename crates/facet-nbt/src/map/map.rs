use alloc::borrow::Cow;
use core::{fmt::Debug, hash::Hash};

use indexmap::{Equivalent, IndexMap};

use crate::{Hasher, Mutf8Str, NbtItem};

/// A map of MUTF-8 strings to NBT items.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct NbtMap<'a>(Option<IndexMap<Cow<'a, Mutf8Str>, NbtItem<'a>, Hasher>>);

impl Debug for NbtMap<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(map) = &self.0 {
            f.debug_tuple("NbtMap").field(map).finish()
        } else {
            f.debug_tuple("NbtMap").field(&"None").finish()
        }
    }
}

impl<'a> NbtMap<'a> {
    /// Create a new [`NbtMap`] marked as `None`.
    #[must_use]
    pub const fn new_none() -> Self { Self(None) }

    /// Returns `true` if this [`NbtMap`] is marked as `None`.
    #[must_use]
    pub const fn is_none(&self) -> bool { self.0.is_none() }

    /// Create a new [`NbtMap`] marked as `Some`.
    #[must_use]
    pub const fn new_some() -> Self { Self(Some(IndexMap::with_hasher(Self::hasher()))) }

    /// Returns `true` if this [`NbtMap`] is marked as `Some`.
    #[must_use]
    pub const fn is_some(&self) -> bool { self.0.is_some() }

    /// Returns the number of elements in this map.
    #[must_use]
    pub fn len(&self) -> Option<usize> { self.0.as_ref().map(IndexMap::len) }

    /// Returns `true` if this map contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> Option<bool> { self.0.as_ref().map(IndexMap::is_empty) }

    /// Create a new [`NbtMap`] with the given capacity.
    #[must_use]
    pub fn some_with_capacity(capacity: usize) -> Self {
        Self(Some(IndexMap::with_capacity_and_hasher(capacity, Self::hasher())))
    }

    /// Returns the number of elements the map can hold without reallocating.
    #[must_use]
    pub fn capacity(&self) -> Option<usize> { self.0.as_ref().map(IndexMap::capacity) }

    /// Returns `true` if the [`NbtMap`] contains a value for the specified key.
    ///
    /// See [`IndexMap::contains_key`] for more details.
    #[must_use]
    pub fn contains_key<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(&self, key: &Q) -> bool {
        self.0.as_ref().is_some_and(|map| map.contains_key(key))
    }

    /// Get a reference to the value corresponding to the key.
    ///
    /// See [`IndexMap::get`] for more details.
    #[must_use]
    pub fn get<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &self,
        key: &Q,
    ) -> Option<&NbtItem<'a>> {
        self.0.as_ref().and_then(|map| map.get(key))
    }

    /// Get references to the key-value pair corresponding to the key.
    ///
    /// See [`IndexMap::get_key_value`] for more details.
    #[must_use]
    pub fn get_key_value<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &self,
        key: &Q,
    ) -> Option<(&Cow<'a, Mutf8Str>, &NbtItem<'a>)> {
        self.0.as_ref().and_then(|map| map.get_key_value(key))
    }

    /// Get the index and references to the key-value pair corresponding to the
    /// key.
    ///
    /// See [`IndexMap::get_full`] for more details.
    #[inline]
    #[must_use]
    pub fn get_full<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &self,
        key: &Q,
    ) -> Option<(usize, &Cow<'a, Mutf8Str>, &NbtItem<'a>)> {
        self.0.as_ref().and_then(|map| map.get_full(key))
    }

    /// Get a mutable reference to the value corresponding to the key.
    ///
    /// See [`IndexMap::get_mut`] for more details.
    #[inline]
    #[must_use]
    pub fn get_mut<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &mut self,
        key: &Q,
    ) -> Option<&mut NbtItem<'a>> {
        self.0.as_mut().and_then(|map| map.get_mut(key))
    }

    /// Get mutable references to the key-value pair corresponding to the key.
    ///
    /// See [`IndexMap::get_key_value_mut`] for more details.
    #[inline]
    #[must_use]
    pub fn get_key_value_mut<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &mut self,
        key: &Q,
    ) -> Option<(&Cow<'a, Mutf8Str>, &mut NbtItem<'a>)> {
        self.0.as_mut().and_then(|map| map.get_key_value_mut(key))
    }

    /// Get the index and mutable references to the key-value pair corresponding
    /// to the key.
    ///
    /// See [`IndexMap::get_full_mut`] for more details.
    #[inline]
    #[must_use]
    pub fn get_full_mut<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &mut self,
        key: &Q,
    ) -> Option<(usize, &Cow<'a, Mutf8Str>, &mut NbtItem<'a>)> {
        self.0.as_mut().and_then(|map| map.get_full_mut(key))
    }

    /// Insert a key-value pair into the [`NbtMap`].
    ///
    /// See [`IndexMap::insert`] for more details.
    #[inline]
    pub fn insert(&mut self, key: Cow<'a, Mutf8Str>, value: NbtItem<'a>) -> Option<NbtItem<'a>> {
        self.0
            .get_or_insert_with(|| IndexMap::with_capacity_and_hasher(1, Self::hasher()))
            .insert(key, value)
    }

    /// Remove a key-value pair from the [`NbtMap`],
    /// swapping the last element into its place.
    ///
    /// This does not preserve the order of elements.
    ///
    /// See [`IndexMap::swap_remove`] for more details.
    #[inline]
    pub fn swap_remove<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &mut self,
        key: &Q,
    ) -> Option<NbtItem<'a>> {
        self.0.as_mut().and_then(|map| map.swap_remove(key))
    }

    /// Remove a key-value pair from the [`NbtMap`],
    /// shifting all elements after it to the left.
    ///
    /// This preserves the order of elements.
    ///
    /// See [`IndexMap::shift_remove`] for more details.
    #[inline]
    pub fn shift_remove<Q: Equivalent<Cow<'a, Mutf8Str>> + Hash + ?Sized>(
        &mut self,
        key: &Q,
    ) -> Option<NbtItem<'a>> {
        self.0.as_mut().and_then(|map| map.shift_remove(key))
    }

    /// Get an iterator over the key-value pairs of the [`Nbt`].
    #[inline]
    #[must_use]
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        <&Self as IntoIterator>::into_iter(self)
    }

    /// Get a mutable iterator over the key-value pairs of the [`Nbt`].
    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        <&mut Self as IntoIterator>::into_iter(self)
    }

    /// Convert this [`NbtMap`] into an owned copy, cloning any borrowed
    /// data in the process.
    #[must_use]
    pub fn into_owned(self) -> NbtMap<'static> {
        match self.0 {
            None => NbtMap(None),
            Some(map) => NbtMap(Some(
                map.into_iter()
                    .map(|(k, v)| (Cow::Owned(k.into_owned()), v.into_owned()))
                    .collect(),
            )),
        }
    }

    /// Get a reference to the underlying [`IndexMap`].
    ///
    /// Requires manually accessing via `Nbt::as_inner(&nbt)`.
    #[inline]
    #[must_use]
    pub const fn as_inner<'b>(
        map: &'b NbtMap<'a>,
    ) -> &'b Option<IndexMap<Cow<'a, Mutf8Str>, NbtItem<'a>, Hasher>> {
        &map.0
    }

    /// Get a mutable reference to the underlying [`IndexMap`].
    ///
    /// Requires manually accessing via `Nbt::as_inner_mut(&mut nbt)`.
    #[inline]
    #[must_use]
    pub const fn as_inner_mut<'b>(
        map: &'b mut NbtMap<'a>,
    ) -> &'b mut Option<IndexMap<Cow<'a, Mutf8Str>, NbtItem<'a>, Hasher>> {
        &mut map.0
    }

    /// Create a new [`Hasher`] instance.
    const fn hasher() -> Hasher {
        #[cfg(not(feature = "foldhash"))]
        {
            Hasher::new()
        }
        #[cfg(feature = "foldhash")]
        {
            Hasher::with_seed(4_899_682_396_686_039_889)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> IntoIterator for NbtMap<'a> {
    type IntoIter = indexmap::map::IntoIter<Cow<'a, Mutf8Str>, NbtItem<'a>>;
    type Item = (Cow<'a, Mutf8Str>, NbtItem<'a>);

    fn into_iter(self) -> Self::IntoIter {
        self.0.unwrap_or_else(|| IndexMap::with_hasher(Self::hasher())).into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'b NbtMap<'a> {
    type IntoIter = core::iter::Flatten<
        core::option::IntoIter<indexmap::map::Iter<'b, Cow<'a, Mutf8Str>, NbtItem<'a>>>,
    >;
    type Item = (&'b Cow<'a, Mutf8Str>, &'b NbtItem<'a>);

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_ref().map(IndexMap::iter).into_iter().flatten()
    }
}
impl<'a, 'b> IntoIterator for &'b mut NbtMap<'a> {
    type IntoIter = core::iter::Flatten<
        core::option::IntoIter<indexmap::map::IterMut<'b, Cow<'a, Mutf8Str>, NbtItem<'a>>>,
    >;
    type Item = (&'b Cow<'a, Mutf8Str>, &'b mut NbtItem<'a>);

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_mut().map(IndexMap::iter_mut).into_iter().flatten()
    }
}
