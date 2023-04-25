//! High-level bindings for the Flipper Zero.

#![no_std]
#![cfg_attr(test, no_main)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod dialogs;
pub mod dolphin;
pub mod furi;
pub mod gui;
pub mod io;
pub mod macros;
pub mod storage;

#[doc(hidden)]
pub mod __internal {
    // Re-export for use in macros
    pub use ufmt;
}

flipperzero_test::tests_runner!(
    name = "flipperzero-rs Unit Tests",
    [crate::furi::message_queue::tests, crate::furi::sync::tests]
);
