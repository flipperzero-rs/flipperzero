//! High-level bindings for the Flipper Zero.

#![no_std]

pub mod furi;
pub mod panic_handler;

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        core::write!($crate::furi::io::Stdout, $($args)*).unwrap()
    };
}

#[macro_export]
macro_rules! println {
    ($pat:expr, $($args:tt)*) => {
        core::write!($crate::furi::io::Stdout, core::concat!($pat, "\r\n"), $($args)*).unwrap()
    };
}
