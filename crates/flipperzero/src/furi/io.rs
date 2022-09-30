//! Furi I/O API.

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
