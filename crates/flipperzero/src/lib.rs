//! High-level bindings for the Flipper Zero.

#![no_std]
#![cfg_attr(feature = "unstable_intrinsics", feature(int_roundings))]
#![cfg_attr(feature = "unstable_lints", feature(must_not_suspend))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "service-dialogs")]
pub mod dialogs;
pub mod furi;
#[cfg(feature = "service-gui")]
pub mod gui;
pub mod input;
pub(crate) mod internals;
pub mod kernel;
pub mod macros;
