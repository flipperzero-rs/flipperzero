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
pub mod notification;
pub mod storage;
pub mod toolbox;

#[doc(hidden)]
pub mod __internal {
    // Re-export for use in macros
    pub use ufmt;
}

flipperzero_test::tests_runner!(
    name = "flipperzero-rs Unit Tests",
    [
        crate::furi::message_queue::tests,
        crate::furi::rng::tests,
        crate::furi::string::tests,
        crate::furi::sync::tests,
        crate::toolbox::crc32::tests,
        crate::toolbox::md5::tests,
        crate::toolbox::sha256::tests,
    ]
);
