#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "facet")]
extern crate facet_core as facet;
#[cfg(feature = "std")]
extern crate std;
