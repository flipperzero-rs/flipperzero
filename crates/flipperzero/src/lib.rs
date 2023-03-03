//! High-level bindings for the Flipper Zero.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod dialogs;
pub mod dolphin;
pub mod furi;
pub mod gui;
pub mod macros;

#[doc(hidden)]
pub mod __internal {
    // Re-export for use in macros
    pub use ufmt;
}
