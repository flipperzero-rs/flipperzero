//! Furi helpers.

use core::ffi::c_char;
use core::fmt::Display;

use crate::FuriStatus;

/// The error type for Furi kernel operations.
#[derive(Clone, Copy, Debug, ufmt::derive::uDebug, Eq, PartialEq)]
pub enum Error {
    Unspecified,
    TimedOut,
    ResourceBusy,
    InvalidParameter,
    OutOfMemory,
    ForbiddenInISR,
    Other(i32),
}

impl Error {
    /// Describe the kind of error.
    pub fn description(&self) -> &str {
        match self {
            Self::Unspecified => "Unspecified RTOS error",
            Self::TimedOut => "Operation not completed within the timeout period",
            Self::ResourceBusy => "Resource not available",
            Self::InvalidParameter => "Parameter error",
            Self::OutOfMemory => "System is out of memory",
            Self::ForbiddenInISR => "Not allowed in ISR context",
            _ => "Unknown",
        }
    }
}

/// Create [`Error`] from [`FuriStatus`].
impl TryFrom<FuriStatus> for Error {
    type Error = i32;

    fn try_from(status: crate::FuriStatus) -> core::result::Result<Self, Self::Error> {
        match status {
            crate::FuriStatus_FuriStatusError => Ok(Self::Unspecified),
            crate::FuriStatus_FuriStatusErrorTimeout => Ok(Self::TimedOut),
            crate::FuriStatus_FuriStatusErrorResource => Ok(Self::ResourceBusy),
            crate::FuriStatus_FuriStatusErrorParameter => Ok(Self::InvalidParameter),
            crate::FuriStatus_FuriStatusErrorNoMemory => Ok(Self::OutOfMemory),
            crate::FuriStatus_FuriStatusErrorISR => Ok(Self::ForbiddenInISR),
            x => {
                if x < 0 {
                    Ok(Self::Other(x))
                } else {
                    Err(x)
                }
            }
        }
    }
}

/// Create [`FuriStatus`] from [`Error`].
impl From<Error> for FuriStatus {
    fn from(error: Error) -> Self {
        match error {
            Error::Unspecified => crate::FuriStatus_FuriStatusError,
            Error::TimedOut => crate::FuriStatus_FuriStatusErrorTimeout,
            Error::ResourceBusy => crate::FuriStatus_FuriStatusErrorResource,
            Error::InvalidParameter => crate::FuriStatus_FuriStatusErrorParameter,
            Error::OutOfMemory => crate::FuriStatus_FuriStatusErrorNoMemory,
            Error::ForbiddenInISR => crate::FuriStatus_FuriStatusErrorISR,
            Error::Other(x) => x as crate::FuriStatus,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.description(), FuriStatus::from(*self))
    }
}

impl ufmt::uDisplay for Error {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "{} ({})", self.description(), FuriStatus::from(*self))
    }
}

/// Operation status.
/// The Furi API switches between using `enum FuriStatus`, `int32_t` and `uint32_t`.
/// Since these all use the same bit representation, we can just "cast" the returns to this type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, ufmt::derive::uDebug, Eq, PartialEq)]
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

    /// Check if this is an error status.
    pub fn is_err(self) -> bool {
        self != Self::OK
    }

    /// Convert into [`Result`] type.
    pub fn into_result(self) -> Result<i32, Error> {
        match Error::try_from(self.0) {
            Err(x) => Ok(x),
            Ok(err) => Err(err),
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

impl From<crate::FuriStatus> for Status {
    fn from(code: FuriStatus) -> Self {
        Status(code)
    }
}

impl From<Status> for Result<i32, Error> {
    fn from(status: Status) -> Self {
        status.into_result()
    }
}

/// Low-level wrapper of a record handle.
pub struct UnsafeRecord<T> {
    name: *const c_char,
    data: *mut T,
}

impl<T> UnsafeRecord<T> {
    /// Opens a record.
    ///
    /// Safety: The caller must ensure that `record_name` lives for the
    /// duration of the object lifetime.
    ///
    /// # Safety
    ///
    /// The caller must provide a valid C-string `name`.
    pub unsafe fn open(name: *const c_char) -> Self {
        Self {
            name,
            data: crate::furi_record_open(name) as *mut T,
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
            crate::furi_record_close(self.name);
        }
    }
}
