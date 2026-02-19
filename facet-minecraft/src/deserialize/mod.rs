//! TODO

use facet::Facet;

pub mod error;
pub mod fns;

/// A trait for types that can be deserialized.
pub trait Deserialize<'facet>: Facet<'facet> {}
