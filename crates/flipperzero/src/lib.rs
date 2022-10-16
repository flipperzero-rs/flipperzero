//! High-level bindings for the Flipper Zero.

#![no_std]

pub mod furi;

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        core::fmt::write(&mut $crate::furi::io::Stdout, core::format_args!($($args)*)).unwrap()
    };
}

#[macro_export]
macro_rules! println {
    ($pat:expr, $($args:tt)*) => {
        {
            core::fmt::write(&mut $crate::furi::io::Stdout, core::format_args!(core::concat!($pat, "\r\n"), $($args)*)).unwrap();
        }
    };

    ($msg:expr) => {
        {
            core::fmt::write(&mut $crate::furi::io::Stdout, core::format_args!(core::concat!($msg, "\r\n"))).unwrap();
        }
    };
}
