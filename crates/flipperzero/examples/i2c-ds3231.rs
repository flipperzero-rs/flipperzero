//! Example of communicating with a [DS3231] Real-Time Clock.
//!
//! [DS3231]: https://www.analog.com/media/en/technical-documentation/data-sheets/DS3231.pdf

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::{error, furi::time::Duration, gpio::i2c, println};
use flipperzero_rt::{entry, manifest};
use ufmt::derive::uDebug;

manifest!(name = "I2C DS3231 Example");
entry!(main);

#[derive(Debug, uDebug)]
enum Hour {
    F12 { hour: u8, pm: bool },
    F24(u8),
}

#[derive(Debug, uDebug)]
struct RtcTime {
    year: u8,
    month: u8,
    date: u8,
    day: u8,
    hour: Hour,
    minutes: u8,
    seconds: u8,
}

impl RtcTime {
    fn parse(data: [u8; 7]) -> Self {
        let unbcd = |b: u8| 10 * (b >> 4) + (b & 0x0F);

        const FORMAT_12HR: u8 = 0b0100_0000;

        Self {
            year: unbcd(data[6]),
            month: unbcd(data[5] & 0x1F),
            date: unbcd(data[4] & 0x3F),
            day: data[3] & 0x07,
            hour: if data[2] & FORMAT_12HR != 0 {
                Hour::F12 {
                    hour: unbcd(data[2] & 0x1F),
                    pm: data[2] & 0b0010_0000 != 0,
                }
            } else {
                Hour::F24(unbcd(data[2] & 0x3F))
            },
            minutes: unbcd(data[1] & 0x7F),
            seconds: unbcd(data[0] & 0x7F),
        }
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    let mut bus = i2c::Bus::EXTERNAL.acquire();
    let rtc = i2c::DeviceAddress::new(0x68);
    let timeout = Duration::from_millis(50);

    if bus.is_device_ready(rtc, timeout) {
        let mut data = [0; 7];
        if bus.read_exact(rtc, 0x00, &mut data, timeout).is_ok() {
            println!("Time: {:?}", RtcTime::parse(data));
        } else {
            error!("Could not read from DS3231");
        }
    } else {
        error!("DS3231 is not connected and ready");
    }

    0
}
