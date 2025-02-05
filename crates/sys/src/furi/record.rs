//! Low-level wrappers around Furi Record API.

use core::ffi::CStr;

/// Low-level wrapper of a record handle.
pub struct UnsafeRecord<T> {
    name: &'static CStr,
    data: *mut T,
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
            data: unsafe { crate::furi_record_open(name.as_ptr()) } as *mut T,
        }
    }

    /// Returns the record data as a raw pointer.
    pub fn as_ptr(&self) -> *mut T {
        self.data
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
