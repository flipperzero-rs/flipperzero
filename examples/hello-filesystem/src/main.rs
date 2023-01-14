//! "Hello, world!" example for Flipper Zero.
//! This app writes "Hello, Rust!" to a file on the SD card and then exits.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use flipperzero::filesystem::*;
use flipperzero::println;
use flipperzero_rt::{entry, manifest};

manifest!(name = "Rust filesystem example");
entry!(main);

fn main(_args: *mut u8) -> i32 {
    let file = OpenOptions::new()
        .write(true)
        .create_always(true)
        .open("hello-rust.txt");

    if let Ok(mut handle) = file {
        if handle.write(b"Hello, Rust!").is_err() {
            println!("couldn't write to file");
        }
    } else {
        println!("couldn't open path");
    }

    0

    // File is synchronized and closed when `f` goes out of scope.
}
