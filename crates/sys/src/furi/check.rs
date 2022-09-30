//! Furi Check API.

use core::ffi::c_char;

extern "C" {
    #[link_name = "furi_crash"]
    pub fn crash(message: *const c_char) -> !;
}
