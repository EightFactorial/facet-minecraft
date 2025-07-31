use fxhash::FxBuildHasher;
use indexmap::IndexMap;

use super::BorrowedTag;
use crate::{
    format::{
        owned::{Nbt, NbtCompound},
        raw::{RawError, RawNbt},
    },
    mutf8::Mutf8Str,
};

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
pub struct BorrowedNbt<'a>(Option<&'a Mutf8Str>, BorrowedCompound<'a>);

impl<'a> BorrowedNbt<'a> {
    /// Create a new empty [`BorrowedNbt`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { BorrowedNbt::from_parts(None, BorrowedCompound::new()) }

    /// Create a new [`BorrowedNbt`] from the given name and compound.
    #[inline]
    #[must_use]
    pub const fn from_parts(name: Option<&'a Mutf8Str>, compound: BorrowedCompound<'a>) -> Self {
        BorrowedNbt(name, compound)
    }

    /// Create a new named [`BorrowedNbt`] from a byte slice.
    ///
    /// # Errors
    /// Returns an error if the byte slice is not a valid named [`BorrowedNbt`].
    pub fn new_named(data: &'a [u8]) -> Result<Self, RawError<'a>> {
        RawNbt::try_new_named(data).map(|raw| raw.to_borrowed())
    }

    /// Create a new unnamed [`BorrowedNbt`] from a byte slice.
    ///
    /// # Errors
    /// Returns an error if the byte slice is not a valid unnamed
    /// [`BorrowedNbt`].
    pub fn new_unnamed(data: &'a [u8]) -> Result<Self, RawError<'a>> {
        RawNbt::try_new_unnamed(data).map(|raw| raw.to_borrowed())
    }

    /// Get the name of the [`BorrowedNbt`], if it has one.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> Option<&'a Mutf8Str> { self.0 }

    /// Get the name of the [`BorrowedNbt`] mutably.
    #[inline]
    #[must_use]
    pub const fn name_mut(&mut self) -> &mut Option<&'a Mutf8Str> { &mut self.0 }

    /// Get the inner [`BorrowedCompound`] of the [`BorrowedNbt`].
    #[inline]
    #[must_use]
    pub const fn compound(&self) -> &BorrowedCompound<'a> { &self.1 }

    /// Get the inner [`BorrowedCompound`] of the [`BorrowedNbt`] mutably.
    #[inline]
    #[must_use]
    pub const fn compound_mut(&mut self) -> &mut BorrowedCompound<'a> { &mut self.1 }

    /// Create a new [`Nbt`] from this [`BorrowedNbt`].
    #[must_use]
    pub fn to_owned(self) -> Nbt {
        match self.0 {
            Some(name) => Nbt::new_named(name.to_mutf8_string(), self.1.to_owned()),
            None => Nbt::new_unnamed(self.1.to_owned()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorrowedCompound<'a>(IndexMap<&'a Mutf8Str, BorrowedTag<'a>, FxBuildHasher>);

impl BorrowedCompound<'_> {
    /// Create a new empty [`BorrowedCompound`].
    #[must_use]
    pub const fn new() -> Self { BorrowedCompound(IndexMap::with_hasher(FxBuildHasher::new())) }

    /// Create a new [`BorrowedCompound`] with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        BorrowedCompound(IndexMap::with_capacity_and_hasher(capacity, FxBuildHasher::new()))
    }

    /// Create a new [`NbtCompound`] from this [`BorrowedCompound`].
    #[must_use]
    pub fn to_owned(self) -> NbtCompound {
        self.0.into_iter().map(|(k, v)| (k.to_mutf8_string(), v.to_owned())).collect()
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> Extend<(&'a Mutf8Str, BorrowedTag<'a>)> for BorrowedCompound<'a> {
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a Mutf8Str, BorrowedTag<'a>)>>(&mut self, iter: T) {
        IndexMap::extend(&mut self.0, iter);
    }
}

impl<'a> FromIterator<(&'a Mutf8Str, BorrowedTag<'a>)> for BorrowedCompound<'a> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (&'a Mutf8Str, BorrowedTag<'a>)>>(iter: T) -> Self {
        BorrowedCompound(FromIterator::from_iter(iter))
    }
}

impl<'a, const N: usize> From<[(&'a Mutf8Str, BorrowedTag<'a>); N]> for BorrowedCompound<'a> {
    #[inline]
    fn from(array: [(&'a Mutf8Str, BorrowedTag<'a>); N]) -> Self {
        BorrowedCompound::from_iter(array)
    }
}

impl From<BorrowedCompound<'_>> for NbtCompound {
    #[inline]
    fn from(compound: BorrowedCompound<'_>) -> Self { compound.to_owned() }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "facet")]
unsafe impl<'facet> facet_core::Facet<'facet> for BorrowedCompound<'facet> {
    const SHAPE: &'static facet::Shape = &const {
        let mut builder = facet::Shape::builder_for_sized::<Self>()
            .type_identifier("BorrowedCompound")
            .ty(facet::Type::User(facet::UserType::Opaque));
        #[cfg(feature = "alloc")]
        {
            builder = builder.def(facet_core::Def::Map(
                facet_core::MapDef::builder()
                    .k(|| <&'facet Mutf8Str>::SHAPE)
                    .v(|| <BorrowedTag<'facet>>::SHAPE)
                    .vtable(
                        &const {
                            facet_core::MapVTableBuilder::new()
                                .init_in_place_with_capacity(|ptr, cap| unsafe {
                                    ptr.put(Self::with_capacity(cap))
                                })
                                .insert(|ptr, key, val| unsafe {
                                    let map = ptr.as_mut::<Self>();
                                    let k = key.read::<&'facet Mutf8Str>();
                                    let v = val.read::<BorrowedTag<'facet>>();
                                    map.insert(k, v);
                                })
                                .len(|ptr| unsafe { ptr.get::<Self>().len() })
                                .contains_key(|ptr, key| unsafe {
                                    ptr.get::<Self>().contains_key(key.get::<&'facet Mutf8Str>())
                                })
                                .get_value_ptr(|ptr, key| unsafe {
                                    ptr.get::<Self>()
                                        .get(key.get::<&'facet Mutf8Str>())
                                        .map(|v| facet_core::PtrConst::new(core::ptr::from_ref(v)))
                                })
                                .iter_vtable(
                                    facet_core::IterVTable::builder()
                                        .init_with_value(|ptr| unsafe {
                                            let iter =
                                                alloc::boxed::Box::new(ptr.get::<Self>().iter());
                                            facet_core::PtrMut::new(
                                                alloc::boxed::Box::into_raw(iter).cast::<u8>(),
                                            )
                                        })
                                        .next(|iter_ptr| unsafe {
                                            let state = iter_ptr.as_mut::<indexmap::map::Iter<
                                                'facet,
                                                &'facet Mutf8Str,
                                                BorrowedTag<'facet>,
                                            >>(
                                            );
                                            state.next().map(|(key, value)| {
                                                (
                                                    facet_core::PtrConst::new(key),
                                                    facet_core::PtrConst::new(value),
                                                )
                                            })
                                        })
                                        .next_back(|iter_ptr| unsafe {
                                            let state = iter_ptr.as_mut::<indexmap::map::Iter<
                                                'facet,
                                                &'facet Mutf8Str,
                                                BorrowedTag<'facet>,
                                            >>(
                                            );
                                            state.next_back().map(|(key, value)| {
                                                (
                                                    facet_core::PtrConst::new(key),
                                                    facet_core::PtrConst::new(value),
                                                )
                                            })
                                        })
                                        .dealloc(|iter_ptr| unsafe {
                                            drop(alloc::boxed::Box::from_raw(
                                                iter_ptr.as_ptr::<indexmap::map::Iter<
                                                    'facet,
                                                    &'facet Mutf8Str,
                                                    BorrowedTag<'facet>,
                                                >>(
                                                )
                                                    as *mut indexmap::map::Iter<
                                                        'facet,
                                                        &'facet Mutf8Str,
                                                        BorrowedTag<'facet>,
                                                    >,
                                            ));
                                        })
                                        .build(),
                                )
                                .build()
                        },
                    )
                    .build(),
            ));
        }
        builder.build()
    };
    const VTABLE: &'static facet::ValueVTable = &const {
        facet::ValueVTable::builder::<Self>()
            .marker_traits(|| {
                facet::MarkerTraits::SEND
                    .union(facet::MarkerTraits::SYNC)
                    .union(facet::MarkerTraits::UNPIN)
                    .union(facet::MarkerTraits::UNWIND_SAFE)
                    .union(facet::MarkerTraits::REF_UNWIND_SAFE)
                    .intersection(<&'facet Mutf8Str>::SHAPE.vtable.marker_traits())
                    .intersection(<BorrowedTag<'facet>>::SHAPE.vtable.marker_traits())
            })
            .type_name(|f, _opts| ::core::fmt::Write::write_str(f, "BorrowedCompound"))
            .default_in_place(|| Some(|target| unsafe { target.put(Self::default()) }))
            .build()
    };
}

// -------------------------------------------------------------------------------------------------

impl<'a> core::convert::AsRef<IndexMap<&'a Mutf8Str, BorrowedTag<'a>, FxBuildHasher>>
    for BorrowedCompound<'a>
{
    fn as_ref(&self) -> &IndexMap<&'a Mutf8Str, BorrowedTag<'a>, FxBuildHasher> { &self.0 }
}
impl<'a> core::convert::AsMut<IndexMap<&'a Mutf8Str, BorrowedTag<'a>, FxBuildHasher>>
    for BorrowedCompound<'a>
{
    fn as_mut(&mut self) -> &mut IndexMap<&'a Mutf8Str, BorrowedTag<'a>, FxBuildHasher> {
        &mut self.0
    }
}

impl<'a> core::ops::Deref for BorrowedCompound<'a> {
    type Target = IndexMap<&'a Mutf8Str, BorrowedTag<'a>, FxBuildHasher>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl core::ops::DerefMut for BorrowedCompound<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
