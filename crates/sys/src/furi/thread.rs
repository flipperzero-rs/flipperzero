//! Furi Thread API.

use core::ffi::c_char;

use crate::furi::base::Status;
use crate::opaque;

opaque!(FuriThreadId);

extern "C" {
    #[link_name = "furi_thread_get_current_id"]
    pub fn get_current_id() -> *const FuriThreadId;
    #[link_name = "furi_thread_get_name"]
    pub fn get_name(thead_id: *const FuriThreadId) -> *const c_char;
    #[link_name = "furi_thread_stdout_flush"]
    pub fn stdout_flush() -> Status;
    #[link_name = "furi_thread_stdout_write"]
    pub fn stdout_write(data: *const u8, size: usize) -> usize;
    #[link_name = "furi_thread_yield"]
    pub fn yield_();
}
