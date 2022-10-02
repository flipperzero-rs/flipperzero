//! Furi HAL Console API.

use core::ffi::{c_char, c_void};

pub type FuriHalConsoleTxCallback =
    extern "C" fn(buffer: *const u8, size: usize, context: *mut c_void);

extern "C" {
    #[link_name = "furi_hal_console_disable"]
    pub fn disable();
    #[link_name = "furi_hal_console_enable"]
    pub fn enable();
    /// Printf-like plain uart interface.
    /// *Warning:* Will not work in ISR context.
    #[link_name = "furi_hal_console_printf"]
    pub fn printf(format: *const c_char, ...);
    #[link_name = "furi_hal_console_puts"]
    pub fn puts(data: *const c_char);
    #[link_name = "furi_hal_console_set_tx_callback"]
    pub fn set_tx_callback(callback: FuriHalConsoleTxCallback, context: *mut c_void);
    #[link_name = "furi_hal_console_tx"]
    pub fn tx(buffer: *const u8, size: usize);
    #[link_name = "furi_hal_console_tx_with_new_line"]
    pub fn tx_with_new_line(buffer: *const u8, size: usize);
}
