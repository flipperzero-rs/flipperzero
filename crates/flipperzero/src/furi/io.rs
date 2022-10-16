//! Furi I/O API.

use core::ffi::c_char;
use core::fmt::{Write, Arguments};

use flipperzero_sys as sys;

pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = s.len();
        unsafe {
            if sys::furi_thread_stdout_write(s.as_ptr() as *const c_char, len) != len {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

impl Stdout {
    pub fn flush(&mut self) -> core::fmt::Result {
        unsafe {
            if sys::furi_thread_stdout_flush() != 0 {
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
