//! TODO

use core::marker::PhantomData;

use crate::{
    format::raw::{RawCompound, RawListTag},
    mutf8::Mutf8Str,
};

/// A reference to a slice of bytes that represents a value.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BorrowedRef<'a, T: ?Sized>(&'a [u8], PhantomData<T>);

impl<'a, T: ?Sized> BorrowedRef<'a, T> {
    /// Create a new [`BorrowedRef`] from a raw byte slice.
    ///
    /// # Warning
    /// This function expects data without a length prefix,
    /// and does not check the validity of the data provided.
    #[inline]
    #[must_use]
    pub(crate) const fn new(data: &'a [u8]) -> Self { Self(data, PhantomData) }

    /// Get the raw byte slice of the [`BorrowedRef`].
    #[inline]
    #[must_use]
    pub const fn as_raw_bytes(&self) -> &'a [u8] { self.0 }
}

impl<'a, T: BorrowedDecode<'a> + ?Sized> BorrowedRef<'a, T>
where Self: Iterator<Item = <T as BorrowedDecode<'a>>::Item>
{
    /// Get the `n`th element of the [`BorrowedRef`].
    ///
    /// # Note
    /// For types without a fixed size, this will decode all preceding elements.
    ///
    /// This matters for strings, lists, and compounds,
    /// whose size is not known until they are fully decoded.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<<T as BorrowedDecode<'a>>::Item> {
        if let Some(size) = T::ITEM_SIZE {
            // Skip ahead to the `n`th element in the slice.
            let skipped_slice = &self.0[index * size..(index + 1) * size];
            T::consume_next(BorrowedRef(skipped_slice, PhantomData)).map(|(item, _)| item)
        } else {
            // Decode all elements until we reach the `n`th one.
            self.clone().nth(index)
        }
    }
}

impl<T: ?Sized> Clone for BorrowedRef<'_, T> {
    #[inline]
    fn clone(&self) -> Self { Self(self.0, PhantomData) }
}

// -------------------------------------------------------------------------------------------------

/// SAFETY: This is almost definitely NOT safe nor sound :)
#[cfg(feature = "facet")]
unsafe impl<'a, T: facet_core::Facet<'a> + ?Sized> facet_core::Facet<'a> for BorrowedRef<'a, T>
where
    Self: Iterator,
    <Self as Iterator>::Item: core::fmt::Debug + facet_core::Facet<'a>,
{
    #[allow(unused_mut)]
    const SHAPE: &'static facet_core::Shape = &const {
        let mut builder = facet_core::Shape::builder_for_sized::<Self>()
            .type_identifier("BorrowedRef<_>")
            .type_params(&const { [facet_core::TypeParam { name: "T", shape: || <T>::SHAPE }] })
            .ty(facet_core::Type::User(facet_core::UserType::Opaque));

        #[cfg(feature = "alloc")]
        {
            builder = builder.def(facet_core::Def::List(
                facet_core::ListDef::builder()
                    .t(|| <Self as Iterator>::Item::SHAPE)
                    .vtable(
                        &const {
                            facet_core::ListVTable {
                                init_in_place_with_capacity: None,
                                push: None,
                                len: |ptr| unsafe { ptr.get::<Self>() }.clone().count(),
                                get: |ptr, idx| unsafe {
                                    ptr.get::<Self>().clone().nth(idx).map(|v| {
                                        let boxed = alloc::boxed::Box::new(v);
                                        facet_core::PtrConst::new(
                                            alloc::boxed::Box::into_raw(boxed).cast::<u8>(),
                                        )
                                    })
                                },
                                get_mut: None,
                                as_ptr: None,
                                as_mut_ptr: None,
                                iter_vtable: facet_core::IterVTable::builder()
                                    .init_with_value(|ptr| {
                                        let borrow = unsafe { ptr.get::<Self>() }.clone();
                                        let boxed = alloc::boxed::Box::<Self>::new(borrow);
                                        facet_core::PtrMut::new(
                                            alloc::boxed::Box::into_raw(boxed).cast::<u8>(),
                                        )
                                    })
                                    .next(|ptr| {
                                        let borrow = unsafe { ptr.as_mut::<Self>() };
                                        borrow.next().map(|v| {
                                            let boxed = alloc::boxed::Box::new(v);
                                            facet_core::PtrConst::new(
                                                alloc::boxed::Box::into_raw(boxed).cast::<u8>(),
                                            )
                                        })
                                    })
                                    .dealloc(|ptr| unsafe {
                                        drop(alloc::boxed::Box::from_raw(
                                            ptr.as_ptr::<Self>() as *mut Self
                                        ));
                                    })
                                    .build(),
                            }
                        },
                    )
                    .build(),
            ));
        }
        builder.build()
    };
    const VTABLE: &'static facet_core::ValueVTable = &const {
        facet_core::ValueVTable::builder::<Self>()
            .type_name(|f, opts| {
                f.write_str("BorrowedRef<")?;
                <T>::SHAPE.write_type_name(f, opts)?;
                f.write_str(">")
            })
            .build()
    };
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can be decoded from a [`BorrowedRef`].
pub trait BorrowedDecode<'a> {
    /// The type of item that can be decoded.
    type Item: Sized;
    /// Whether the type has a fixed size in bytes.
    const ITEM_SIZE: Option<usize> = None;

    /// Consume the next value from the [`BorrowedRef`],
    /// returning the value and the remaining borrowed bytes.
    fn consume_next(borrow: BorrowedRef<'a, Self>) -> Option<(Self::Item, BorrowedRef<'a, Self>)>;
}

impl<'a, T: BorrowedDecode<'a> + ?Sized> Iterator for BorrowedRef<'a, T> {
    type Item = <T as BorrowedDecode<'a>>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        T::consume_next(self.clone()).map(|(item, rem)| {
            *self = rem;
            item
        })
    }
}

impl<'a> Iterator for BorrowedRef<'a, &'a Mutf8Str> {
    type Item = &'a Mutf8Str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let (str, rem) = Mutf8Str::new_raw_prefixed(self.0);
        self.0 = rem;

        Some(str)
    }
}
impl<'a> Iterator for BorrowedRef<'a, [&'a Mutf8Str]> {
    type Item = &'a Mutf8Str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut borrow = BorrowedRef::<&'a Mutf8Str>(self.0, PhantomData);
        let result = borrow.next();
        self.0 = borrow.0;
        result
    }
}

impl<'a> Iterator for BorrowedRef<'a, RawCompound<'a>> {
    type Item = BorrowedRef<'a, RawCompound<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let mut consumed = RawCompound::new_unchecked(self.0);
        while consumed.next_entry().is_some() {}

        let borrow = &self.0[..self.0.len() - consumed.as_raw_bytes().len()];
        self.0 = consumed.as_raw_bytes();

        Some(BorrowedRef(borrow, PhantomData))
    }
}
impl<'a> Iterator for BorrowedRef<'a, [RawCompound<'a>]> {
    type Item = BorrowedRef<'a, RawCompound<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut borrow = BorrowedRef::<RawCompound<'a>>(self.0, PhantomData);
        let result = borrow.next();
        self.0 = borrow.0;
        result
    }
}

impl<'a> Iterator for BorrowedRef<'a, RawListTag<'a>> {
    type Item = BorrowedRef<'a, RawListTag<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let (.., rem) = RawListTag::parse_data(self.0)?;
        let borrow = &self.0[..self.0.len() - rem.len()];

        Some(BorrowedRef(borrow, PhantomData))
    }
}
impl<'a> Iterator for BorrowedRef<'a, [RawListTag<'a>]> {
    type Item = BorrowedRef<'a, RawListTag<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut borrow = BorrowedRef::<RawListTag<'a>>(self.0, PhantomData);
        let result = borrow.next();
        self.0 = borrow.0;
        result
    }
}

// -------------------------------------------------------------------------------------------------

impl BorrowedDecode<'_> for i8 {
    type Item = i8;

    const ITEM_SIZE: Option<usize> = Some(1);

    fn consume_next(borrow: BorrowedRef<'_, Self>) -> Option<(Self::Item, BorrowedRef<'_, Self>)> {
        borrow
            .0
            .split_first_chunk::<1>()
            .map(|(&first, rest)| (i8::from_be_bytes(first), BorrowedRef(rest, PhantomData)))
    }
}
impl BorrowedDecode<'_> for i16 {
    type Item = i16;

    const ITEM_SIZE: Option<usize> = Some(2);

    fn consume_next(borrow: BorrowedRef<'_, Self>) -> Option<(Self::Item, BorrowedRef<'_, Self>)> {
        borrow
            .0
            .split_first_chunk::<2>()
            .map(|(&first, rest)| (i16::from_be_bytes(first), BorrowedRef(rest, PhantomData)))
    }
}
impl BorrowedDecode<'_> for i32 {
    type Item = i32;

    const ITEM_SIZE: Option<usize> = Some(4);

    fn consume_next(borrow: BorrowedRef<'_, Self>) -> Option<(Self::Item, BorrowedRef<'_, Self>)> {
        borrow
            .0
            .split_first_chunk::<4>()
            .map(|(&first, rest)| (i32::from_be_bytes(first), BorrowedRef(rest, PhantomData)))
    }
}
impl BorrowedDecode<'_> for i64 {
    type Item = i64;

    const ITEM_SIZE: Option<usize> = Some(8);

    fn consume_next(borrow: BorrowedRef<'_, Self>) -> Option<(Self::Item, BorrowedRef<'_, Self>)> {
        borrow
            .0
            .split_first_chunk::<8>()
            .map(|(&first, rest)| (i64::from_be_bytes(first), BorrowedRef(rest, PhantomData)))
    }
}

impl BorrowedDecode<'_> for f32 {
    type Item = f32;

    const ITEM_SIZE: Option<usize> = Some(4);

    fn consume_next(borrow: BorrowedRef<'_, Self>) -> Option<(Self::Item, BorrowedRef<'_, Self>)> {
        borrow
            .0
            .split_first_chunk::<4>()
            .map(|(&first, rest)| (f32::from_be_bytes(first), BorrowedRef(rest, PhantomData)))
    }
}
impl BorrowedDecode<'_> for f64 {
    type Item = f64;

    const ITEM_SIZE: Option<usize> = Some(8);

    fn consume_next(borrow: BorrowedRef<'_, Self>) -> Option<(Self::Item, BorrowedRef<'_, Self>)> {
        borrow
            .0
            .split_first_chunk::<8>()
            .map(|(&first, rest)| (f64::from_be_bytes(first), BorrowedRef(rest, PhantomData)))
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, T: BorrowedDecode<'a>> BorrowedDecode<'a> for [T] {
    type Item = T::Item;

    fn consume_next(borrow: BorrowedRef<'a, Self>) -> Option<(Self::Item, BorrowedRef<'a, Self>)> {
        T::consume_next(BorrowedRef(borrow.0, PhantomData))
            .map(|(item, next)| (item, BorrowedRef(next.0, PhantomData)))
    }
}

impl<'a, T: BorrowedDecode<'a>> BorrowedDecode<'a> for [&'a [T]] {
    type Item = BorrowedRef<'a, [T]>;

    fn consume_next(
        mut borrow: BorrowedRef<'a, Self>,
    ) -> Option<(Self::Item, BorrowedRef<'a, Self>)> {
        let start = borrow.0.len();
        while let Some((_, next)) = <[T]>::consume_next(BorrowedRef(borrow.0, PhantomData)) {
            borrow = BorrowedRef(next.0, PhantomData);
        }

        let reference = &borrow.0[..start - borrow.0.len()];
        Some((BorrowedRef(reference, PhantomData), borrow))
    }
}
