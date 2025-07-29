use fxhash::FxBuildHasher;
use indexmap::IndexMap;

use super::NbtTag;
use crate::mutf8::Mutf8String;

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NbtCompound(IndexMap<Mutf8String, NbtTag, FxBuildHasher>);

// -------------------------------------------------------------------------------------------------

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
#[expect(clippy::elidable_lifetime_names)]
unsafe impl<'facet> facet_core::Facet<'facet> for NbtCompound {
    const SHAPE: &'static facet::Shape = &const {
        facet::Shape::builder_for_sized::<Self>()
            .type_identifier("NbtCompound")
            .ty(facet::Type::User(facet::UserType::Opaque))
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
