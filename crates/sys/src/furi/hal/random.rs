//! Furi HAL random number generation.

extern "C" {
    #[link_name = "furi_hal_random_fill_buf"]
    pub fn fill_buf(buf: *mut u8, len: u32);
    #[link_name = "furi_hal_random_get"]
    pub fn get() -> u32;
}
