use core::cell::UnsafeCell;
use core::fmt::{Debug, Display, Formatter};
use core::ops::{Deref, DerefMut};

#[cfg(not(feature = "portable"))]
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(feature = "portable")]
use portable_atomic::{AtomicBool, Ordering};

/// A mutual exclusion primitive useful for protecting shared data.
pub struct Mutex<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    ///
    /// # Examples
    ///
    /// ```
    /// use skirt::sync::Mutex;
    ///
    /// let mutex = Mutex::new(0);
    /// ```
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self.try_lock().is_none() {
            #[cfg(not(feature = "std"))]
            core::hint::spin_loop();

            #[cfg(feature = "std")]
            std::thread::yield_now();
        }

        MutexGuard::new(self)
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired at this time, then [`None`] is returned.
    /// Otherwise, an RAII guard is returned. The lock will be unlocked when the
    /// guard is dropped.
    ///
    /// This function does not block.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
            .then(|| MutexGuard::new(self))
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the `Mutex` mutably, no actual locking needs to
    /// take place -- the mutable borrow statically guarantees no locks exist.
    ///
    /// # Examples
    ///
    ///```
    /// use skirt::sync::Mutex;
    ///
    /// let mut mutex = Mutex::new(0);
    /// *mutex.get_mut() = 10;
    /// assert_eq!(*mutex.lock(), 10);
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}

impl<T> From<T> for Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    /// This is equivalent to [`Mutex::new`].
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T: Default> Default for Mutex<T> {
    /// Creates a `Mutex<T>`, with the `Default` value for T.
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: ?Sized + Debug> Debug for Mutex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut d = f.debug_struct("Mutex");

        match self.try_lock() {
            Some(guard) => d.field("data", &&*guard),
            None => d.field("data", &format_args!("<locked>")),
        };

        d.finish_non_exhaustive()
    }
}

/// An RAII implementation of a "scoped lock" of a mutex. When this structure is
/// dropped (falls out of scope), the lock will be unlocked.
///
/// The data protected by the mutex can be accessed through this guard via its
/// [`Deref`] and [`DerefMut`] implementations.
///
/// This structure is created by the [`lock`] and [`try_lock`] methods on
/// [`Mutex`].
///
/// [`lock`]: Mutex::lock
/// [`try_lock`]: Mutex::try_lock
pub struct MutexGuard<'m, T: ?Sized + 'm> {
    mutex: &'m Mutex<T>,
    #[cfg(not(feature = "nightly"))]
    phantom: core::marker::PhantomData<*const ()>,
}

#[cfg(feature = "nightly")]
impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<'m, T: ?Sized> MutexGuard<'m, T> {
    const fn new(mutex: &'m Mutex<T>) -> Self {
        Self {
            mutex,
            #[cfg(not(feature = "nightly"))]
            phantom: core::marker::PhantomData,
        }
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The lock is held, giving us exclusive access to the data.
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The lock is held, giving us exclusive access to the data.
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}

impl<T: ?Sized + Debug> Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized + Display> Display for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}
