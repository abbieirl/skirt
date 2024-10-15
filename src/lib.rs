//! # Skirt

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(feature = "nightly", feature(negative_impls))]

mod lazy_lock;
mod mutex;
mod once;
mod once_lock;
// mod rwlock;

/// Synchronization primitives that rely on spin-locking mechanisms.
pub mod sync {
    pub use crate::lazy_lock::*;
    pub use crate::mutex::*;
    pub use crate::once::*;
    pub use crate::once_lock::*;
    // pub use crate::rwlock::*;

    pub(crate) mod atomic {
        #[cfg(not(feature = "portable"))]
        pub use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
        #[cfg(feature = "portable")]
        pub use portable_atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
    }
}
