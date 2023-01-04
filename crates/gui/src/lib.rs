//! Safe wrappers for Flipper GUI APIs.

#![no_std]
// requred for effectve calculation of some values
#![feature(int_roundings)]

extern crate alloc;

pub mod canvas;
pub mod gui;
pub mod icon;
pub mod icon_animation;
pub mod input;
pub mod view;
pub mod view_port;
pub mod xbm;
