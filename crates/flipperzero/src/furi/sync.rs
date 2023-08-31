//! Furi syncronization primitives.

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use flipperzero_sys as sys;
use lock_api::{GuardNoSend, RawMutex, RawMutexTimed};
use sys::furi::Status;

use super::time::{Duration, Instant};

const MUTEX_TYPE: u8 = sys::FuriMutexType_FuriMutexTypeNormal;

/// A [`RawMutex`] implementation backed by Furi. You probably want to use [`Mutex`]
/// instead.
pub struct FuriMutex(AtomicPtr<sys::FuriMutex>);

impl FuriMutex {
    const fn new() -> Self {
        Self(AtomicPtr::new(ptr::null_mut()))
    }

    unsafe fn get(&self) -> *mut sys::FuriMutex {
        let mutex = self.0.load(Ordering::Acquire);
        if !mutex.is_null() {
            mutex
        } else {
            self.create()
        }
    }

    unsafe fn create(&self) -> *mut sys::FuriMutex {
        let mutex = unsafe { sys::furi_mutex_alloc(MUTEX_TYPE) };

        match self
            .0
            .compare_exchange(ptr::null_mut(), mutex, Ordering::Release, Ordering::Relaxed)
        {
            Ok(_) => mutex,
            Err(global_ptr) => {
                unsafe { sys::furi_mutex_free(mutex) };
                global_ptr
            }
        }
    }

    /// Attempts to acquire the mutex within `timeout` ticks, or without blocking if
    /// `timeout` is zero.
    fn try_acquire(&self, timeout: u32) -> bool {
        let status: Status = unsafe { sys::furi_mutex_acquire(self.get(), timeout).into() };
        status.is_ok()
    }
}

impl Drop for FuriMutex {
    fn drop(&mut self) {
        let mutex = self.0.load(Ordering::Acquire);
        if !mutex.is_null() {
            unsafe { sys::furi_mutex_free(mutex) };
        }
    }
}

unsafe impl RawMutex for FuriMutex {
    // See docs to the parent definition
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self = FuriMutex::new();
    type GuardMarker = GuardNoSend;

    fn lock(&self) {
        // `INCLUDE_vTaskSuspend` is set to 1 in the Flipper Zero's FreeRTOS config, so as
        // long as this timeout value matches `portMAX_DELAY` in the FreeRTOS config, this
        // will block indefinitely as intended.
        assert!(self.try_acquire(u32::MAX));
    }

    fn try_lock(&self) -> bool {
        self.try_acquire(0)
    }

    unsafe fn unlock(&self) {
        let status: Status = unsafe { sys::furi_mutex_release(self.get()).into() };
        if status.is_err() {
            panic!("furi_mutex_release failed: {}", status);
        }
    }
}

unsafe impl RawMutexTimed for FuriMutex {
    type Duration = Duration;
    type Instant = Instant;

    fn try_lock_for(&self, timeout: Self::Duration) -> bool {
        self.try_acquire(timeout.0)
    }

    fn try_lock_until(&self, timeout: Self::Instant) -> bool {
        let now = Instant::now();
        self.try_lock_for(timeout - now)
    }
}

pub type Mutex<T> = lock_api::Mutex<FuriMutex, T>;
pub type MutexGuard<'a, T> = lock_api::MutexGuard<'a, FuriMutex, T>;

#[flipperzero_test::tests]
mod tests {
    use super::Mutex;

    #[test]
    fn unshared_mutex_does_not_block() {
        let mutex = Mutex::new(7u64);

        {
            let mut value = mutex.lock();
            assert_eq!(*value, 7);
            *value = 42;
        }

        {
            let value = mutex.lock();
            assert_eq!(*value, 42);
        }
    }
}
