//! "Hello, world!" example for Flipper Zero.
//! This app prints "Hello, Rust!" to the console then exits.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use flipperzero::println;
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
    name = "Hello, Rust!",
    app_version = 1,
    has_icon = true,
    // See `docs/icons.md` for icon format
    icon = "rustacean-10x10.icon",
);

// Define the entry function
entry!(main);

// Entry point
fn main(_args: *mut u8) -> i32 {
    println!("Hello, Rust!");

    0
}
