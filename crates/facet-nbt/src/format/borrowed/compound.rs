use fxhash::FxBuildHasher;
use indexmap::IndexMap;

use super::BorrowedTag;
use crate::{mutf8::Mutf8Str, prelude::NbtCompound};

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorrowedCompound<'a>(IndexMap<&'a Mutf8Str, BorrowedTag<'a>, FxBuildHasher>);

// -------------------------------------------------------------------------------------------------

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
        facet::Shape::builder_for_sized::<Self>()
            .type_identifier("BorrowedCompound")
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
                    // TODO: This should be `&'facet Mutf8Str` instead of `&'facet Mutf8String`
                    .intersection(<&'facet crate::mutf8::Mutf8String>::SHAPE.vtable.marker_traits())
                    .intersection(BorrowedTag::<'facet>::SHAPE.vtable.marker_traits())
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
