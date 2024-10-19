//! Example showcasing the use of a stream buffer on multiple threads.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

extern crate alloc;

use alloc::borrow::ToOwned;
use core::{ffi::CStr, num::NonZeroUsize};
use core::time::Duration;

use flipperzero::{furi::{self, thread}, println};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(name = "Stream buffer example");

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    let size = NonZeroUsize::new(1024).unwrap();
    let trigger_level = 16;
    let (tx, rx) = furi::stream_buffer::stream_buffer(size, trigger_level);
    0
}
