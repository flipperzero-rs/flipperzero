use crate::internals::Unsend;
use core::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
};
use flipperzero_sys::{self as sys, furi::Status};

#[inline(always)]
fn is_irq_or_masked() -> bool {
    // SAFETY: this function has no invariants to uphold
    unsafe { sys::furi_kernel_is_irq_or_masked() }
}

pub fn lock() -> Result<LockGuard, LockError> {
    if is_irq_or_masked() {
        Err(LockError::Interrupted)
    } else {
        // SAFETY: kernel is not interrupted
        let status = unsafe { sys::furi_kernel_lock() };

        Ok(match status {
            0 => LockGuard {
                was_locked: false,
                _marker: PhantomData,
            },
            1 => LockGuard {
                was_locked: true,
                _marker: PhantomData,
            },
            status => Err(LockError::ErrorStatus(Status(status)))?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "unstable_lints",
    must_not_suspend = "holding a MutexGuard across suspend \
                      points can cause deadlocks, delays, \
                      and cause Futures to not implement `Send`"
)]
pub struct LockGuard {
    was_locked: bool,
    _marker: PhantomData<Unsend>,
}

impl LockGuard {
    pub const fn was_locked(&self) -> bool {
        self.was_locked
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        // SAFETY: since `LockGuard` is `!Send` it cannot escape valid (non-interrupt) context
        let _ = unsafe { sys::furi_kernel_unlock() };
    }
}

/// A type of error which can be returned whenever a lock is acquired.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LockError {
    Interrupted,
    ErrorStatus(Status),
}

impl Display for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interrupted => write!(f, "context is in interruption state"),
            Self::ErrorStatus(status) => write!(f, "error status: {status}"),
        }
    }
}
