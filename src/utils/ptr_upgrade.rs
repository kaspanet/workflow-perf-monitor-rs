#[cfg(target_os = "windows")]
use std::num::NonZeroIsize;

/// Triage a return value of windows handle to `Some(handle)` or `None`
#[cfg(target_os = "windows")]
pub trait HandleUpgrade: Sized {
    fn upgrade(self) -> Option<NonZeroIsize>;
}

#[cfg(target_os = "windows")]
impl HandleUpgrade for isize {
    #[inline]
    fn upgrade(self) -> Option<NonZeroIsize> {
        NonZeroIsize::new(self)
    }
}
