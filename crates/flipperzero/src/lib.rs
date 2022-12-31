//! High-level bindings for the Flipper Zero.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod furi;
pub mod macros;
