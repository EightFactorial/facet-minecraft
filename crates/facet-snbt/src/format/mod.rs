//! TODO

mod legacy;
pub use legacy::{Legacy, LegacySnbt};

#[cfg(feature = "alloc")]
mod modern;
#[cfg(feature = "alloc")]
pub use modern::{Modern, ModernSnbt};

// -------------------------------------------------------------------------------------------------

/// A trait representing an SNBT format.
pub trait SnbtFormat<'a>: core::fmt::Debug + Default + Copy + Eq + Send + Sync {
    /// The inner storage type.
    type Inner: Sized + facet_core::Facet<'a>;
}
