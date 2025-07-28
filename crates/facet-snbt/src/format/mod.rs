//! TODO

mod legacy;
pub use legacy::{Legacy, LegacySnbt};

mod modern;
pub use modern::{Modern, ModernSnbt};

// -------------------------------------------------------------------------------------------------

/// A trait representing an SNBT format.
pub trait SnbtFormat: core::fmt::Debug + Default + Copy + Eq + Send + Sync {}
