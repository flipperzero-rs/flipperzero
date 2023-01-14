//! Furi Thread API.

// TODO: decide what to do with delays not fitting in 32 bits

pub mod sync {
    use core::time::Duration;
    use flipperzero_sys as sys;

    /// Puts the current thread to sleep for at least the specified amount of time.
    pub fn sleep(duration: Duration) {
        const MAX_US_DURATION: Duration = Duration::from_secs(3600);

        unsafe {
            // For durations of 1h+, use delay_ms so uint32_t doesn't overflow
            if duration < MAX_US_DURATION {
                sys::furi_delay_us(duration.as_micros() as u32);
            } else {
                sys::furi_delay_ms(duration.as_millis() as u32);
                // TODO: add reamining us-part
            }
        }
    }
}
