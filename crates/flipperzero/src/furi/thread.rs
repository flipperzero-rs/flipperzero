//! Furi Thread API.

use flipperzero_sys as sys;

/// Puts the current thread to sleep for at least the specified amount of time.
pub fn sleep(duration: core::time::Duration) {
    unsafe {
        // For durations of 1h+, use delay_ms so uint32_t doesn't overflow
        if duration < core::time::Duration::from_secs(3600) {
            sys::furi::kernel::delay_us(duration.as_micros() as u32);
        } else {
            sys::furi::kernel::delay_ms(duration.as_millis() as u32);
        }
    }
}
