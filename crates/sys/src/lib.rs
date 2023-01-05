//! Low-level bindings for the Flipper Zero.

#![no_std]

/// Re-export bindings
pub use bindings::*;
use core::hint::unreachable_unchecked;

pub mod furi;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bindings;

// Re-export macro for safe compile-time c-string creation
pub use real_c_string::real_c_string as c_string;

/// Crash the system.
#[macro_export]
macro_rules! crash {
    ($msg:literal $(,)?) => {
        unsafe {
            // Crash message is passed via r12
            let msg = $crate::c_string!($msg);
            core::arch::asm!("", in("r12") msg, options(nomem, nostack));

            $crate::furi_crash();
            // `unreachable!` generates exception machinery, `noreturn` does not
            core::arch::asm!("", options(noreturn));
        }
    };
}

// TODO: find a better place
#[doc(hidden)]
#[inline(always)]
pub fn furi_crash() {
    // SAFETY: crash function has no invariants to uphold
    // and it always crashes the program
    unsafe {
        __furi_crash();
        unreachable_unchecked();
    }
}

// TODO: find a better place
#[doc(hidden)]
#[inline(always)]
pub fn furi_halt() {
    // SAFETY: crash function has no invariants to uphold
    // and it always crashes the program
    unsafe {
        __furi_halt();
        unreachable_unchecked();
    }
}
