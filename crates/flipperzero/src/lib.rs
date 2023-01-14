//! High-level bindings for the Flipper Zero.

#![no_std]
#![cfg_attr(feature = "unstable_lints", feature(must_not_suspend))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod dialogs;
pub mod furi;
pub mod gui;
pub(crate) mod internals;
pub mod kernel;
pub mod macros;
