//! Low-level bindings for the Flipper Zero.

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

// Features that identify thumbv7em-none-eabihf.
// Until target_abi is stable, this also permits thumbv7em-none-eabi.
#[cfg(not(all(
    target_arch = "arm",
    target_feature = "thumb2",
    target_feature = "v7",
    target_feature = "dsp",
    target_os = "none",
    //target_abi = "eabihf",
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
/// The only argument is a message with which the system should crash
/// which should contain no NULs. The following will not compile:
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
#[macro_export]
macro_rules! crash {
    ($msg:expr $(,)?) => {{
        const MESSAGE: *const ::core::primitive::i8 =
            match ::core::ffi::CStr::from_bytes_with_nul(
                ::core::concat!($msg, "\0").as_bytes(),
            ) {
                Ok(cstr) => cstr.as_ptr(),
                Err(error) => panic!("message contains NULs"),
            };
        unsafe {
            // Crash message is passed via r12
            ::core::arch::asm!("", in("r12") MESSAGE, options(nomem, nostack));

            $crate::__furi_crash_implementation();
            ::core::hint::unreachable_unchecked();
        }
    }};
}

// Re-export bindings
pub use bindings::*;

// Definition of inline functions
pub use inlines::furi_hal_gpio::*;
