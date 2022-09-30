//! Furi Kernel API.

extern "C" {
    #[link_name = "furi_delay_ms"]
    pub fn delay_ms(msec: u32);
    #[link_name = "furi_delay_us"]
    pub fn delay_us(usec: u32);
}
