//! Rust Runtime for the Flipper Zero.
//! 
//! This must be build with `-Z no-unique-section-names` to ensure that this module
//! is linked directly into the `.text` section.

#![no_std]

pub mod manifest;
pub mod panic_handler;

/// The C entry point.
/// This just delegates to the user's Rust entry point.
#[no_mangle]
pub unsafe extern "C" fn _start(args: *mut u8) -> i32 {
    extern "Rust" {
        fn main(args: *mut u8) -> i32;
    }

    main(args)
}

/// Define the entry point.
/// Must have the following signature: `fn(*mut u8) -> i32`.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        // Force the section to `.text` instead of `.text.main`.
        // lld seems not to automatically rename `.rel.text.main` properly.
        #[export_name = "main"]
        pub unsafe fn __main(args: *mut u8) -> i32 {
            // type check the entry function
            let f: fn(*mut u8) -> i32 = $path;

            f(args)
        }
    }
}
