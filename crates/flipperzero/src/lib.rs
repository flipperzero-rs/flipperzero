//! High-level bindings for the Flipper Zero.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod filesystem;
pub mod furi;
pub mod macros;
