//! Furi base definitions.

use core::fmt::Display;

/// Operation status.
/// The Furi API switches between using `enum FuriStatus`, `int32_t` and `uint32_t`.
/// Since these all use the same bit representation, we can just "cast" the returns to this type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Status(pub i32);

impl Status {
    /// Describes the status result of the operation.
    pub fn description(self) -> &'static str {
        use status::*;

        match self {
            OK => "Operation completed successfully",
            ERR => "Unspecified RTOS error",
            ERR_TIMEOUT => "Operation not completed within the timeout period",
            ERR_RESOURCE => "Resource not available",
            ERR_PARAMETER => "Parameter error",
            ERR_NO_MEMORY => "System is out of memory",
            ERR_ISR => "Not allowed in ISR context",
            _ => "Unknown",
        }
    }

    /// Was the operation successful?
    pub fn is_ok(self) -> bool {
        self == status::OK
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}: {}", self, self.description())
    }
}

impl From<i32> for Status {
    fn from(code: i32) -> Self {
        Status(code)
    }
}

/// Status codes.
pub mod status {
    use super::Status;

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
}
