//! Low-level bindings for the Flipper Zero.

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

// Features that identify thumbv7em-none-eabihf.
// NOTE: `arm_target_feature` is currently unstable (see rust-lang/rust#44839)
#[cfg(not(any(
    all(
        target_arch = "arm",
        //target_feature = "thumb2",
        //target_feature = "v7",
        //target_feature = "dsp",
        target_os = "none",
        target_abi = "eabihf",
    ),
    miri
)))]
core::compile_error!("This crate requires `--target thumbv7em-none-eabihf`");

pub mod furi;
mod inlines;

#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::transmute_int_to_bool,
    clippy::useless_transmute,
    rustdoc::broken_intra_doc_links
)]
mod bindings;

/// Crash the system.
///
/// May be provided with an optional message which should not contain NULs.
/// The following will not compile:
///
/// ```compile_fail
/// flipperzero_sys::crash!("Has a \0 NUL");
/// ```
///
/// # Examples
///
/// Crash the system with a *"Hello world!"* message:
///
/// ```
/// flipperzero_sys::crash!("Hello world!");
/// ```
///
/// Crash the system with default *"Fatal Error"* message:
///
/// ```
/// flipperzero_sys::crash!();
/// ```
#[macro_export]
macro_rules! crash {
    () => {
        $crate::__crash_implementation!(::core::ptr::null());
    };
    ($msg:expr $(,)?) => {{
        let message = const {
            match ::core::ffi::CStr::from_bytes_with_nul(::core::concat!($msg, "\0").as_bytes()) {
                Err(_) => c"nul in crash message",
                Ok(m) => m,
            }
        };

        $crate::__crash_implementation!(message.as_ptr());
    }};
}

/// Crash the system.
///
/// This is an internal implementation detail.
#[doc(hidden)]
#[macro_export]
macro_rules! __crash_implementation {
    ($ptr:expr) => {
        unsafe {
            // Crash message is passed via r12
            ::core::arch::asm!(
                "ldr pc,=__furi_crash_implementation",
                in("r12") ($ptr),
                options(nomem, nostack),
            );

            ::core::hint::unreachable_unchecked();
        }
    };
}

/// Halt the system.
///
/// May be provided with an optional message which should not contain NULs.
/// The following will not compile:
///
/// ```compile_fail
/// flipperzero_sys::halt!("Has a \0 NUL");
/// ```
///
/// # Examples
///
/// Halt the system with a *"Hello world!"* message:
///
/// ```
/// flipperzero_sys::crash!("Hello world!");
/// ```
///
/// Halt the system with default *"System halt requested."* message:
///
/// ```
/// flipperzero_sys::crash!();
/// ```
#[macro_export]
macro_rules! halt {
    () => {
        $crate::__halt_implementation!(::core::ptr::null());
    };
    ($msg:expr $(,)?) => {{
        // Optional message
        let message = const {
            match ::core::ffi::CStr::from_bytes_with_nul(::core::concat!($msg, "\0").as_bytes()) {
                Err(_) => c"nul in halt message",
                Ok(m) => m,
            }
        };

        $crate::__halt_implementation!(message.as_ptr());
    }};
}

/// Halt the system.
///
/// This is an internal implementation detail.
#[doc(hidden)]
#[macro_export]
macro_rules! __halt_implementation {
    ($ptr:expr) => {
        unsafe {
            // Halt message is passed via r12
            ::core::arch::asm!(
                "ldr pc,=__furi_halt_implementation",
                in("r12") ($ptr),
                options(nomem, nostack))
            ;

            ::core::hint::unreachable_unchecked();
        }
    };
}

// Re-export bindings
pub use bindings::*;

// Definition of inline functions
pub use inlines::furi_hal_gpio::*;
