//! Macros for Flipper Zero.

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {{
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__macro_support::ufmt;
        ufmt::uwrite!($crate::furi::io::Stdout, $($args)*).ok();
    }};
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {{
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__macro_support::ufmt;
        ufmt::uwriteln!($crate::furi::io::Stdout, $($args)*).ok();
    }};
}
