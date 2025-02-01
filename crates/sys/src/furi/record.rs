//! Low-level wrappers around Furi Record API.

use core::ffi::CStr;
use core::ptr::NonNull;

/// Low-level wrapper of a record handle.
///
/// This effectively acts as a reference count for the open underlying Record.
pub struct UnsafeRecord<T> {
    name: &'static CStr,
    raw: NonNull<T>,
}

impl<T> UnsafeRecord<T> {
    /// Opens a record.
    ///
    /// # Safety
    ///
    /// `T` must be the correct C type for the record identified by `name`.
    pub unsafe fn open(name: &'static CStr) -> Self {
        Self {
            name,
            // SAFETY: `furi_record_open` blocks until the record is initialized with a valid value.
            raw: unsafe { NonNull::new_unchecked(crate::furi_record_open(name.as_ptr()).cast()) },
        }
    }

    /// Returns the record data as a raw pointer.
    pub fn as_ptr(&self) -> *mut T {
        self.raw.as_ptr()
    }
}

impl<T> Clone for UnsafeRecord<T> {
    fn clone(&self) -> Self {
        // SAFETY: Opening a record multiple times just increases its reference count.
        unsafe { Self::open(self.name) }
    }
}

impl<T> Drop for UnsafeRecord<T> {
    fn drop(&mut self) {
        unsafe {
            // decrement the holders count
            crate::furi_record_close(self.name.as_ptr());
        }
    }
}
