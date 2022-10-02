//! Low-level bindings for the Flipper Zero.

#![no_std]

pub mod canvas;
pub mod furi;
pub mod gui;
pub mod icon;
pub mod lfrfid;
pub mod notification;
pub mod toolbox;
pub mod view_port;

/// Declare an opaque type.
#[macro_export]
macro_rules! opaque {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            _data: [u8; 0],
            _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
        }
    };
}

/// Create a static C string.
/// Will automatically add a nul terminator.
#[macro_export]
macro_rules! c_string {
    ($str:expr) => {{
        concat!($str, "\0").as_ptr() as *const core::ffi::c_char
    }};
}
