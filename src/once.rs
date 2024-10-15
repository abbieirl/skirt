use core::fmt::{Debug, Formatter};
use core::panic::{RefUnwindSafe, UnwindSafe};

pub struct Once {}

impl UnwindSafe for Once {}
impl RefUnwindSafe for Once {}

impl Once {
    /// Creates a new `Once` value.
    #[inline]
    pub const fn new() -> Self {
        todo!()
    }

    pub fn call_once<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        todo!()
    }

    pub fn is_completed(&self) -> bool {
        todo!()
    }

    pub fn wait(&self) {
        todo!()
    }
}

impl Debug for Once {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Once").finish_non_exhaustive()
    }
}
