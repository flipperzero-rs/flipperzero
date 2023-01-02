//! Safe wrappers for Flipper GUI APIs.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod canvas;
pub mod gui;
pub mod view;
pub mod view_port;
