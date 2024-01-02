//! Example of communicating with a [GC9A01] LCD module.
//!
//! [GC9A01]: https://www.buydisplay.com/download/ic/GC9A01A.pdf

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use flipperzero::{error, furi::time::Duration, gpio::spi, println};
use flipperzero_rt::{entry, manifest};

manifest!(name = "SPI GC9A01 Example");
entry!(main);

fn main(_args: *mut u8) -> i32 {
    let mut bus = spi::EXTERNAL.acquire();
    let timeout = Duration::from_millis(50);

    // TODO: Something correct.
    let mut data = [0; 7];
    if bus.read_exact(&mut data, timeout).is_ok() {
        println!("Read data");
    } else {
        error!("Could not read from GC9A01");
    }

    0
}
