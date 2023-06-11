//! "Hello, world!" example for Flipper Zero.
//! This app writes "Hello, Rust!" to a file on the SD card and then exits.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::io::*;
use flipperzero::println;
use flipperzero::storage::*;
use flipperzero_rt::{entry, manifest};

manifest!(name = "Rust storage example");
entry!(main);

fn main(_args: *mut u8) -> i32 {
    // First, we'll create a file on the SD card and write "Hello, Rust!" to it.
    let file = OpenOptions::new()
        .write(true)
        .create_always(true)
        .open(CStr::from_bytes_with_nul(b"/ext/hello-rust.txt\0").unwrap());

    match file {
        Ok(mut handle) => {
            if let Err(e) = handle.write(b"Hello, Rust!") {
                println!("couldn't write to file: {}", e);
            }
        }
        Err(e) => println!("couldn't open path: {}", e),
    }

    // Now, we'll open it and read it back.
    let mut buffer: [u8; 16] = [0; 16];
    let file = OpenOptions::new()
        .read(true)
        .open(CStr::from_bytes_with_nul(b"/ext/hello-rust.txt\0").unwrap());

    match file {
        Ok(mut handle) => match handle.read(&mut buffer) {
            Ok(n) => println!("Read from file: {:?}", &buffer[..n]),
            Err(e) => println!("couldn't read from file: {}", e),
        },
        Err(e) => println!("couldn't open path: {}", e),
    }

    0

    // File is synchronized and closed when `f` goes out of scope.
}
