#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "serde_json_canonicalizer requires that either `std` (default) or `alloc` feature is enabled"
}

extern crate alloc;

mod jcs;
mod util;

#[doc(inline)]
pub use crate::util::*;
