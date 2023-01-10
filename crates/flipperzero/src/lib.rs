//! High-level bindings for the Flipper Zero.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod dialogs;
pub mod furi;
pub mod gui;
pub mod macros;
