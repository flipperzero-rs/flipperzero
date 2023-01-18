//! Notification example for Flipper Zero

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use core::time::Duration;

use flipperzero::{
    furi::thread::sleep,
    notification::{sequences, NotificationApp, NotificationMessage},
};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(name = "Rust notification example");

// Define the entry function
entry!(main);

// Entry point
fn main(_args: *mut u8) -> i32 {
    let mut app = NotificationApp::open();

    // Set the notification LED to different colours
    for sequences in [
        &sequences::ONLY_RED,
        &sequences::ONLY_GREEN,
        &sequences::ONLY_BLUE,
    ] {
        app.notify(sequences);
        sleep(Duration::from_secs(1));
    }

    app.notify(&sequences::RESET_RGB);

    // Success!
    app.notify_blocking(&sequences::SUCCESS);

    0
}
