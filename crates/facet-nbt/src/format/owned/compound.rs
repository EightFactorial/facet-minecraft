use fxhash::FxBuildHasher;
use indexmap::IndexMap;

use super::NbtTag;
use crate::mutf8::Mutf8String;

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
pub struct Nbt(Option<Mutf8String>, NbtCompound);

impl Nbt {
    /// Create a new empty [`Nbt`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Nbt(None, NbtCompound::new()) }

    /// Create a new [`Nbt`] from the given name and compound.
    #[inline]
    #[must_use]
    pub const fn new_named(name: Mutf8String, compound: NbtCompound) -> Self {
        Nbt(Some(name), compound)
    }

    /// Create a new [`Nbt`] from the given compound.
    #[inline]
    #[must_use]
    pub const fn new_unnamed(compound: NbtCompound) -> Self { Nbt(None, compound) }

    /// Get a reference to the name of the [`Nbt`], if it has one.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &Option<Mutf8String> { &self.0 }

    /// Get a mutable reference to the name of the [`Nbt`], if it has one.
    #[inline]
    #[must_use]
    pub const fn name_mut(&mut self) -> &mut Option<Mutf8String> { &mut self.0 }

    /// Get a reference to the inner [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn compound(&self) -> &NbtCompound { &self.1 }

    /// Get a mutable reference to the inner [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn compound_mut(&mut self) -> &mut NbtCompound { &mut self.1 }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NbtCompound(IndexMap<Mutf8String, NbtTag, FxBuildHasher>);

impl NbtCompound {
    /// Create a new empty [`NbtCompound`].
    #[must_use]
    pub const fn new() -> Self { NbtCompound(IndexMap::with_hasher(FxBuildHasher::new())) }

    /// Create a new [`NbtCompound`] with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        NbtCompound(IndexMap::with_capacity_and_hasher(capacity, FxBuildHasher::new()))
    }
}

// -------------------------------------------------------------------------------------------------

impl Extend<(Mutf8String, NbtTag)> for NbtCompound {
    #[inline]
    fn extend<T: IntoIterator<Item = (Mutf8String, NbtTag)>>(&mut self, iter: T) {
        IndexMap::extend(&mut self.0, iter);
    }
}

impl FromIterator<(Mutf8String, NbtTag)> for NbtCompound {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (Mutf8String, NbtTag)>>(iter: T) -> Self {
        NbtCompound(FromIterator::from_iter(iter))
    }
}

impl<const N: usize> From<[(Mutf8String, NbtTag); N]> for NbtCompound {
    #[inline]
    fn from(array: [(Mutf8String, NbtTag); N]) -> Self { NbtCompound::from_iter(array) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "facet")]
unsafe impl<'facet> facet_core::Facet<'facet> for NbtCompound {
    const SHAPE: &'static facet::Shape = &const {
        facet::Shape::builder_for_sized::<Self>()
            .type_identifier("NbtCompound")
            .ty(facet::Type::User(facet::UserType::Opaque))
            .def(facet_core::Def::Map(
                facet_core::MapDef::builder()
                    .k(|| Mutf8String::SHAPE)
                    .v(|| NbtTag::SHAPE)
                    .vtable(
                        &const {
                            facet_core::MapVTableBuilder::new()
                                .init_in_place_with_capacity(|ptr, cap| unsafe {
                                    ptr.put(Self::with_capacity(cap))
                                })
                                .insert(|ptr, key, val| unsafe {
                                    let map = ptr.as_mut::<Self>();
                                    let k = key.read::<Mutf8String>();
                                    let v = val.read::<NbtTag>();
                                    map.insert(k, v);
                                })
                                .len(|ptr| unsafe { ptr.get::<Self>().len() })
                                .contains_key(|ptr, key| unsafe {
                                    ptr.get::<Self>().contains_key(key.get::<Mutf8String>())
                                })
                                .get_value_ptr(|ptr, key| unsafe {
                                    ptr.get::<Self>()
                                        .get(key.get::<Mutf8String>())
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
                                                Mutf8String,
                                                NbtTag,
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
                                                Mutf8String,
                                                NbtTag,
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
                                                    Mutf8String,
                                                    NbtTag,
                                                >>(
                                                )
                                                    as *mut indexmap::map::Iter<
                                                        'facet,
                                                        Mutf8String,
                                                        NbtTag,
                                                    >,
                                            ));
                                        })
                                        .build(),
                                )
                                .build()
                        },
                    )
                    .build(),
            ))
            .build()
    };
    const VTABLE: &'static facet::ValueVTable = &const {
        facet::ValueVTable::builder::<Self>()
            .marker_traits(|| {
                facet::MarkerTraits::SEND
                    .union(facet::MarkerTraits::SYNC)
                    .union(facet::MarkerTraits::UNPIN)
                    .union(facet::MarkerTraits::UNWIND_SAFE)
                    .union(facet::MarkerTraits::REF_UNWIND_SAFE)
                    .intersection(Mutf8String::SHAPE.vtable.marker_traits())
                    .intersection(NbtTag::SHAPE.vtable.marker_traits())
            })
            .type_name(|f, _opts| ::core::fmt::Write::write_str(f, "NbtCompound"))
            .default_in_place(|| Some(|target| unsafe { target.put(Self::default()) }))
            .build()
    };
}

// -------------------------------------------------------------------------------------------------

impl core::convert::AsRef<IndexMap<Mutf8String, NbtTag, FxBuildHasher>> for NbtCompound {
    fn as_ref(&self) -> &IndexMap<Mutf8String, NbtTag, FxBuildHasher> { &self.0 }
}
impl core::convert::AsMut<IndexMap<Mutf8String, NbtTag, FxBuildHasher>> for NbtCompound {
    fn as_mut(&mut self) -> &mut IndexMap<Mutf8String, NbtTag, FxBuildHasher> { &mut self.0 }
}

impl core::ops::Deref for NbtCompound {
    type Target = IndexMap<Mutf8String, NbtTag, FxBuildHasher>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl core::ops::DerefMut for NbtCompound {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
