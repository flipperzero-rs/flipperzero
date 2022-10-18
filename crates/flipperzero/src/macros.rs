//! Macros for Flipper Zero.

#[macro_export]
macro_rules! print {
    ($pat:expr, $($args:tt)*) => {{
        $crate::furi::io::_print(core::format_args!($pat, $($args)*));
    }};

    ($msg:expr) => {{
        $crate::furi::io::_write_str($msg);
    }};
}

#[macro_export]
macro_rules! println {
    ($pat:expr, $($args:tt)*) => {{
        $crate::furi::io::_print(core::format_args!(concat!($pat, "\r\n"), $($args)*));
    }};

    ($msg:expr) => {{
        $crate::furi::io::_write_str(concat!($msg, "\r\n"));
    }};
}
