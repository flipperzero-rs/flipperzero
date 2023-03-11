//! Macros for Flipper Zero.

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {{
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__internal::ufmt;
        ufmt::uwrite!($crate::furi::io::Stdout, $($args)*).ok();
    }};
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {{
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__internal::ufmt;
        ufmt::uwriteln!($crate::furi::io::Stdout, $($args)*).ok();
    }};
}
