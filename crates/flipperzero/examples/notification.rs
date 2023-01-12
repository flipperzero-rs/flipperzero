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

// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::c_char;
use core::time::Duration;

use flipperzero::{
    furi::thread::sleep,
    notification::{messages, notes, NotificationApp, NotificationMessage, NotificationSequence},
};
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
    let mut app = NotificationApp::open();

    unsafe {
        // Set the notification LED to different colours
        for message in [&messages::RED_255, &messages::GREEN_255, &messages::BLUE_255] {
            let light = [
                &messages::RED_0,
                &messages::GREEN_0,
                &messages::BLUE_0,
                message,
                &messages::DO_NOT_RESET,
                &messages::END,
            ];

            app.notify(light);
            sleep(Duration::from_secs(1));
        }

        let reset_rgb = [&messages::RED_0, &messages::GREEN_0, &messages::BLUE_0, &messages::END];
        app.notify(reset_rgb);

        // Success!
        let success = [
            &messages::DISPLAY_BACKLIGHT_ON,
            &messages::GREEN_255,
            &messages::VIBRO_ON,
            &notes::C5,
            &messages::DELAY_50,
            &messages::VIBRO_OFF,
            &notes::E5,
            &messages::DELAY_50,
            &notes::G5,
            &messages::DELAY_50,
            &notes::C6,
            &messages::DELAY_50,
            &messages::SOUND_OFF,
            &messages::END,
        ];
        app.notify(success);
        sleep(Duration::from_secs(1));
    }

    0
}
