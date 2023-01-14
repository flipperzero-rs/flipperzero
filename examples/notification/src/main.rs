//! Example of using the Flipper Zero notification API.
//! This currently uses the `flipperzero-sys` crate as it is not currently
//! exposed in the high-level `flipperzero` crate.
//! 
//! See https://github.com/flipperdevices/flipperzero-firmware/blob/0.74.2/applications/services/notification/notification_messages.h#L66
//! for possible message sequences.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::c_char;
use core::time::Duration;

use flipperzero::furi::thread::sync::sleep;
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

/// Record for the notifications app
const RECORD_NOTIFICATION: *const c_char = sys::c_string!("notification");

// Define the FAP Manifest for this application
manifest!(name = "Rust notification example");

// Define the entry function
entry!(main);

// Entry point
fn main(_args: *mut u8) -> i32 {
    let notification_app = unsafe { sys::furi::UnsafeRecord::<sys::NotificationApp>::open(RECORD_NOTIFICATION) };

    unsafe {
        // Set the notification LED to different colours
        for sequence in [&sys::sequence_set_only_red_255, &sys::sequence_set_only_green_255, &sys::sequence_set_only_blue_255] {
            sys::notification_message(notification_app.as_raw(), sequence);
            sleep(Duration::from_secs(1));
        }
        sys::notification_message(notification_app.as_raw(), &sys::sequence_reset_rgb);

        // Success!
        sys::notification_message(notification_app.as_raw(), &sys::sequence_success);
    }

    0
}
