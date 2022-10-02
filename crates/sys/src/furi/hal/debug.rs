//! Furi HAL debug API.

extern "C" {
    #[link_name = "furi_hal_debug_disable"]
    pub fn disable();
    #[link_name = "furi_hal_debug_enable"]
    pub fn enable();
}
