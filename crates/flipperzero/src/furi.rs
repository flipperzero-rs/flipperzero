//! High-level bindings to Furi kernel

use flipperzero_sys as sys;
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            if sys::furi::thread_stdout_write(s.as_ptr(), s.len()) != s.len() {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

impl Stdout {
    pub fn flush(&mut self) -> core::fmt::Result {
        unsafe {
            if sys::furi::thread_stdout_flush() != 0 {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

/// Puts the current thread to sleep for at least the specified amount of time.
pub fn sleep(duration: core::time::Duration) {
    unsafe {
        // For durations of 1h+, use delay_ms so uint32_t doesn't overflow
        if duration < core::time::Duration::from_secs(3600) {
            sys::furi::delay_us(duration.as_micros() as u32);
        } else {
            sys::furi::delay_ms(duration.as_millis() as u32);
        }
    }
}
