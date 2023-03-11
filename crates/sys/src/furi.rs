//! Furi helpers.

use core::ffi::c_char;
use core::fmt::Display;
use core::ptr::NonNull;
use core::time::Duration;

/// Operation status.
/// The Furi API switches between using `enum FuriStatus`, `int32_t` and `uint32_t`.
/// Since these all use the same bit representation, we can just "cast" the returns to this type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, ufmt::derive::uDebug)]
pub struct Status(pub i32);

impl Status {
    /// Operation completed successfully.
    pub const OK: Status = Status(0);
    /// Unspecified RTOS error: run-time error but no other error message fits.
    pub const ERR: Status = Status(-1);
    /// Operation not completed within the timeout period.
    pub const ERR_TIMEOUT: Status = Status(-2);
    /// Resource not available.
    pub const ERR_RESOURCE: Status = Status(-3);
    /// Parameter error.
    pub const ERR_PARAMETER: Status = Status(-4);
    /// System is out of memory: it was impossible to allocate or reserve memory for the operation.
    pub const ERR_NO_MEMORY: Status = Status(-5);
    /// Not allowed in ISR context: the function cannot be called from interrupt service routines.
    pub const ERR_ISR: Status = Status(-6);

    /// Describes the status result of the operation.
    pub fn description(self) -> &'static str {
        match self {
            Self::OK => "Operation completed successfully",
            Self::ERR => "Unspecified RTOS error",
            Self::ERR_TIMEOUT => "Operation not completed within the timeout period",
            Self::ERR_RESOURCE => "Resource not available",
            Self::ERR_PARAMETER => "Parameter error",
            Self::ERR_NO_MEMORY => "System is out of memory",
            Self::ERR_ISR => "Not allowed in ISR context",
            _ => "Unknown",
        }
    }

    /// Was the operation successful?
    pub fn is_ok(self) -> bool {
        self == Self::OK
    }

    /// Did the operation error?
    pub fn is_err(self) -> bool {
        self != Self::OK
    }

    /// Returns `Err(Status)` if [`Status`] is an error, otherwise `Ok(ok)`.
    pub fn err_or<T>(self, ok: T) -> Result<T, Self> {
        if self.is_err() {
            Err(self)
        } else {
            Ok(ok)
        }
    }

    /// Returns `Err(Status)` if [`Status`] is an error, otherwise `Ok(or_else(Status))`.
    pub fn err_or_else<T>(self, or_else: impl Fn(Self) -> T) -> Result<T, Self> {
        if self.is_err() {
            Err(self)
        } else {
            Ok(or_else(self))
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}: {}", self, self.description())
    }
}

impl ufmt::uDisplay for Status {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "{:?}: {}", self, self.description())
    }
}

impl From<i32> for Status {
    fn from(code: i32) -> Self {
        Status(code)
    }
}

/// Low-level wrapper of a record handle.
pub struct UnsafeRecord<T> {
    name: *const c_char,
    data: NonNull<T>,
}

impl<T> UnsafeRecord<T> {
    /// Opens a record.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `record_name` lives for the
    /// duration of the object lifetime.
    pub unsafe fn open(name: *const c_char) -> Self {
        // SAFETY: the created pointer is guaranteed to be valid
        let data = unsafe { crate::furi_record_open(name) } as *mut T;
        // SAFETY: the created pointer is guaranteed to be non-null
        let data = unsafe { NonNull::new_unchecked(data) };
        Self { name, data }
    }

    /// Returns the record data as a raw pointer.
    pub fn as_raw(&self) -> *mut T {
        self.data.as_ptr()
    }

    #[deprecated = "use `as_raw(&self)` instead"]
    pub fn as_ptr(&self) -> *mut T {
        self.as_raw()
    }
}

impl<T> Drop for UnsafeRecord<T> {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: `self.name` is valid since it was used to construct this istance
            // and ownership has not been taken
            crate::furi_record_close(self.name);
        }
    }
}

/// Convert [`Duration`] to ticks.
#[inline]
pub fn duration_to_ticks(duration: Duration) -> u32 {
    // This maxes out at about 50 days
    let duration_ms: u32 = duration.as_millis().try_into().unwrap_or(u32::MAX);

    unsafe { crate::furi_ms_to_ticks(duration_ms) }
}
