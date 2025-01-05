//! Furi helpers.

use core::ffi::CStr;
use core::fmt::Display;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

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
    name: &'static CStr,
    data: *mut T,
}

impl<T> UnsafeRecord<T> {
    /// Opens a record.
    pub unsafe fn open(name: &'static CStr) -> Self {
        Self {
            name,
            data: crate::furi_record_open(name.as_ptr()) as *mut T,
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

/// Heap-allocated value.
///
/// This is intended for situations where it is not possible to rely upon a global allocator.
/// Most users should make use of `flipperzero-alloc`.
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct FuriBox<T: ?Sized>(NonNull<T>);

impl<T> FuriBox<T> {
    /// Allocates and initializes a correctly aligned value on the system heap.
    pub fn new(value: T) -> Self {
        let ptr = unsafe { crate::aligned_malloc(size_of::<T>(), align_of::<T>()) as *mut T };
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());

        unsafe {
            ptr.write(value);

            // SAFETY: Pointer is non-null, aligned and represents a valid `T`
            FuriBox::from_raw(ptr)
        }
    }

    /// Consume the box and return raw pointer.
    ///
    /// Caller is responsible for calling `T::drop()` and freeing the pointer with `aligned_free`.
    pub fn into_raw(b: FuriBox<T>) -> *mut T {
        b.0.as_ptr()
    }

    /// Constructs a box from a raw pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory problems.
    ///
    /// The caller is responsible for ensuring the pointer is non-null, was allocated using `aligned_malloc`
    /// with the correct alignment for `T` and that the memory represents a valid `T`.
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        FuriBox(NonNull::new_unchecked(raw))
    }

    /// Returns a raw pointer to the Box’s contents.
    ///
    /// The caller must ensure that the Box outlives the pointer this function returns,
    /// or else it will end up dangling.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an `UnsafeCell`) using this pointer
    /// or any pointer derived from it. If you need to mutate the contents of the Box, use `as_mut_ptr``.
    ///
    /// This method guarantees that for the purpose of the aliasing model,
    /// this method does not materialize a reference to the underlying memory,
    /// and thus the returned pointer will remain valid when mixed with
    /// other calls to as_ptr and as_mut_ptr.
    pub fn as_ptr(b: &FuriBox<T>) -> *const T {
        b.0.as_ptr().cast_const()
    }

    /// Returns a raw mutable pointer to the Box’s contents.
    ///
    /// The caller must ensure that the Box outlives the pointer this function returns,
    /// or else it will end up dangling.
    ///
    /// This method guarantees that for the purpose of the aliasing model,
    /// this method does not materialize a reference to the underlying memory,
    /// and thus the returned pointer will remain valid when mixed with
    /// other calls to `as_ptr`` and `as_mut_ptr``.
    pub fn as_mut_ptr(b: &mut FuriBox<T>) -> *mut T {
        b.0.as_ptr()
    }
}

impl<T: ?Sized> Drop for FuriBox<T> {
    fn drop(&mut self) {
        // SAFETY: Pointer was allocated by `aligned_malloc`
        unsafe { crate::aligned_free(self.0.as_ptr().cast()) }
    }
}

impl<T: ?Sized> AsRef<T> for FuriBox<T> {
    fn as_ref(&self) -> &T {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> AsMut<T> for FuriBox<T> {
    fn as_mut(&mut self) -> &mut T {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized> Deref for FuriBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for FuriBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_mut() }
    }
}
