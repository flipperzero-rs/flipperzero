//! Furi Record API.

use core::ffi::{c_char, c_void};

extern "C" {
    #[link_name = "furi_record_open"]
    pub fn open(name: *const c_char) -> *mut c_void;
    #[link_name = "furi_record_close"]
    pub fn close(name: *const c_char);
}
