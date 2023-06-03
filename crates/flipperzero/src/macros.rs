//! Macros for Flipper Zero.

/// Creates a new [`FuriString`](crate::furi::string::FuriString) by interpolating the format string.
#[macro_export]
macro_rules! format {
    ($($args:tt)*) => {{
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__macro_support::ufmt;
        let mut buf = $crate::furi::string::FuriString::new();
        ufmt::uwrite!(buf, $($args)*).expect("unable to format string");
        buf
    }}
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {{
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__macro_support::ufmt;
        ufmt::uwrite!($crate::furi::io::Stdout, $($args)*).expect("unable to write to stdout");
    }};
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {{
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__macro_support::ufmt;
        ufmt::uwriteln!($crate::furi::io::Stdout, $($args)*).expect("unable to write to stdout");
    }};
}
