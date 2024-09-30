//! "Hello, world!" example for Flipper Zero.
//! This app prints "Hello, Rust!" to the console then exits.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::{debug, info, println};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
    name = "Hello, Rust!",
    app_version = 1,
    has_icon = true,
    // See `docs/icons.md` for icon format
    icon = "icons/rustacean-10x10.icon",
);

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    info!("Hello, reader of the logs!");
    println!("Hello, {}!", "Rust");

    let ret_code = 0;
    debug!("Return code: {}", ret_code);
    ret_code
}
