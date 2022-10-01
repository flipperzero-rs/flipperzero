//! Furi Kernel API.

use core::time::Duration;

/// Convert [`Duration`] to ticks.
#[inline]
pub fn duration_to_ticks(duration: Duration) -> u32 {
    // This maxes out at about 50 days
    let duration_ms: u32 = duration.as_millis().try_into().unwrap_or(u32::MAX);

    unsafe { ms_to_ticks(duration_ms) }
}

extern "C" {
    #[link_name = "furi_ms_to_ticks"]
    pub fn ms_to_ticks(milliseconds: u32) -> u32;
    #[link_name = "furi_delay_ms"]
    pub fn delay_ms(msec: u32);
    #[link_name = "furi_delay_us"]
    pub fn delay_us(usec: u32);
}
