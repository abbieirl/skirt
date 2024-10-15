//! # Skirt

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(negative_impls))]

mod mutex;
mod once;
mod rwlock;

pub mod sync {
    pub use crate::mutex::*;
    pub use crate::once::*;
    pub use crate::rwlock::*;
}
