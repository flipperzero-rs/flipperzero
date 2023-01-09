//! Example of using the Flipper Zero GPIO API.
//! 
//! This currently uses the `flipperzero-sys` crate as it is not currently
//! exposed in the high-level `flipperzero` crate.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::time::Duration;

use flipperzero::println;
use flipperzero::furi::thread::sleep;
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

/// Number of GPIO on one port
const GPIO_NUMBER: usize = 16;

// Define the FAP Manifest for this application
manifest!(name = "Rust GPIO example");

// Define the entry function
entry!(main);

/// GPIO write pin.
/// This is inlined in the C header, thus not exported by the SDK.
unsafe fn gpio_write(gpio: *const sys::GpioPin, state: bool) {
    let port = (*gpio).port;
    if state {
        (*port).BSRR = (*gpio).pin as u32;
    } else {
        (*port).BSRR = ((*gpio).pin as u32) << GPIO_NUMBER;
    }
}

// Entry point
fn main(_args: *mut u8) -> i32 {
    unsafe {
        println!("Configuring pin C0 as output pin");
        sys::furi_hal_gpio_init_simple(&sys::gpio_ext_pc0, sys::GpioMode_GpioModeOutputPushPull);

        println!("Pulling pin C0 high");
        gpio_write(&sys::gpio_ext_pc0, true);

        sleep(Duration::from_secs(1));

        println!("Pulling pin C0 low");
        gpio_write(&sys::gpio_ext_pc0, false);
    }

    0
}
