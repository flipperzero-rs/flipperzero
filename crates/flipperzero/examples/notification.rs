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
    notification::{
        /*messages, notes,*/ NotificationApp, NotificationMessage, NotificationSequence,
    },
    notification_sequence,
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
        for sequence in [ONLY_RED, ONLY_GREEN, ONLY_BLUE] {
            app.notify(sequence);
            sleep(Duration::from_secs(1));
        }

        let reset_rgb = notification_sequence![
            NotificationMessage::led_red(0),
            NotificationMessage::led_green(0),
            NotificationMessage::led_blue(0),
        ];
        app.notify(reset_rgb);

        // Success!
        let success = notification_sequence![
            NotificationMessage::display_backlight(0xFF),
            NotificationMessage::led_green(255),
            NotificationMessage::vibro(true),
            NotificationMessage::sound_on(523.25, 1.0),
            NotificationMessage::delay(50),
            NotificationMessage::vibro(false),
            NotificationMessage::sound_on(659.26, 1.0),
            NotificationMessage::delay(50),
            NotificationMessage::sound_on(783.99, 1.0),
            NotificationMessage::delay(50),
            NotificationMessage::sound_on(1046.5, 1.0),
            NotificationMessage::delay(50),
            NotificationMessage::sound_off(),
        ];
        app.notify(success);
    }

    0
}

const ONLY_RED: NotificationSequence = notification_sequence![
    NotificationMessage::led_red(255),
    NotificationMessage::led_green(0),
    NotificationMessage::led_blue(0),
    NotificationMessage::do_not_reset(),
];

const ONLY_GREEN: NotificationSequence = notification_sequence![
    NotificationMessage::led_red(0),
    NotificationMessage::led_green(255),
    NotificationMessage::led_blue(0),
    NotificationMessage::do_not_reset(),
];

const ONLY_BLUE: NotificationSequence = notification_sequence![
    NotificationMessage::led_red(0),
    NotificationMessage::led_green(0),
    NotificationMessage::led_blue(255),
    NotificationMessage::do_not_reset(),
];
