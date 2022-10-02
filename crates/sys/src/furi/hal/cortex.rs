//! Furi HAL Cortex API.

extern "C" {
    #[link_name = "furi_hal_cortex_delay_us"]
    pub fn delay_us(microseconds: u32);
    #[link_name = "furi_hal_cortex_instructions_per_microsecond"]
    pub fn instructions_per_microsecond() -> u32;
}
