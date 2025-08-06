//! A trait and methods for asserting that a type can be hashed.

use facet_core::{Facet, Shape};

/// A trait for asserting that a type can be hashed.
pub trait AssertHashable<'facet>: Facet<'facet> {
    /// An assertion that the type can be hashed.
    const ASSERT: () = assert!(assert_hashable(Self::SHAPE), "Type cannot be hashed!");
    /// An assertion that the type can be hashed.
    fn assert() { const { Self::ASSERT } }
}

// -------------------------------------------------------------------------------------------------

const fn assert_hashable(_shape: &Shape) -> bool { todo!() }
