//! Demonstrates use of the Flipper Zero GPIO API.
//!
//! This currently uses the `flipperzero-sys` crate as it is not currently
//! exposed in the high-level `flipperzero` crate.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use core::ffi::CStr;
use core::time::Duration;

use flipperzero::furi::thread::sleep;
use flipperzero::println;
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

// Define the FAP Manifest for this application
manifest!(name = "Rust GPIO example");

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    unsafe {
        println!("Configuring pin C0 as output pin");
        sys::furi_hal_gpio_init_simple(&sys::gpio_ext_pc0, sys::GpioMode_GpioModeOutputPushPull);

        println!("Pulling pin C0 high");
        sys::furi_hal_gpio_write(&sys::gpio_ext_pc0, true);

        sleep(Duration::from_secs(1));

        let state = sys::furi_hal_gpio_read(&sys::gpio_ext_pc0);
        println!("Pin C0 is {}", if state { "high" } else { "low" });

        println!("Pulling pin C0 low");
        sys::furi_hal_gpio_write(&sys::gpio_ext_pc0, false);

        sleep(Duration::from_secs(1));

        let state = sys::furi_hal_gpio_read(&sys::gpio_ext_pc0);
        println!("Pin C0 is {}", if state { "high" } else { "low" });
    }

    0
}
