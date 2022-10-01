//! Furi syncronization primitives.

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use flipperzero_sys as sys;

use crate::furi;

/// Negative trait bounds are not implemented (see rust-lang/rust#68318).
/// As a workaround we can force `!Send`/`!Sync` by pretending we own a raw pointer.
type UnsendUnsync = PhantomData<*const ()>;

/// A mutual exclusion primitive useful for protecting shared data.
pub struct Mutex<T: ?Sized> {
    mutex: *mut sys::furi::mutex::FuriMutex,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        let mutex = unsafe { sys::furi::mutex::alloc(sys::furi::mutex::Type::Normal) };
        if mutex.is_null() {
            panic!("furi_mutex_alloc failed");
        }

        Mutex {
            mutex,
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires a mutex, blocking the current thread until it is able to do so.
    pub fn lock(&self) -> furi::Result<MutexGuard<'_, T>> {
        let status = unsafe { sys::furi::mutex::acquire(self.mutex, u32::MAX) };
        if !status.is_ok() {
            return Err(status);
        }

        Ok(MutexGuard(&self, PhantomData))
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

/// An RAII implementation of a "scoped lock" of a mutex.
/// When this structure is dropped (falls out of scope), the lock will be unlocked.
pub struct MutexGuard<'a, T: ?Sized + 'a>(&'a Mutex<T>, UnsendUnsync);

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        let status = unsafe { sys::furi::mutex::release(self.0.mutex) };
        if !status.is_ok() {
            panic!("furi_mutex_release failed: {}", status);
        }
    }
}

// `UnsendUnsync` is actually a bit too strong.
// As long as `T` implements `Sync`, it's fine to access it from another thread.
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}
