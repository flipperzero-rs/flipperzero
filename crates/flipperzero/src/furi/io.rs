//! Furi I/O API.

use core::fmt::{Write, Arguments};

use flipperzero_sys as sys;

pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            if sys::furi::thread::stdout_write(s.as_ptr(), s.len()) != s.len() {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

impl Stdout {
    pub fn flush(&mut self) -> core::fmt::Result {
        unsafe {
            if sys::furi::thread::stdout_flush().is_err() {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    // Avoid generating exception machinery
    Stdout.write_fmt(args).ok();
}

#[doc(hidden)]
pub fn _write_str(s: &str) {
    // Adoid generating exception machinery
    Stdout.write_str(s).ok();
}
