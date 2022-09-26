//! Low-level bindings to Furi kernel

use core::ffi::{c_char, c_void};

use crate::opaque;

opaque!(FuriThreadId);

extern "C" {
    #[link_name = "furi_crash"]
    pub fn crash(message: *const c_char) -> !;
    #[link_name = "furi_delay_ms"]
    pub fn delay_ms(msec: u32);
    #[link_name = "furi_delay_us"]
    pub fn delay_us(usec: u32);
    #[link_name = "furi_thread_get_current_id"]
    pub fn thread_get_current_id() -> *const FuriThreadId;
    #[link_name = "furi_thread_get_name"]
    pub fn thread_get_name(thead_id: *const FuriThreadId) -> *const c_char;
    #[link_name = "furi_thread_stdout_flush"]
    pub fn thread_stdout_flush() -> i32;
    #[link_name = "furi_thread_stdout_write"]
    pub fn thread_stdout_write(data: *const u8, size: usize) -> usize;
    #[link_name = "furi_thread_yield"]
    pub fn thread_yield();
    #[link_name = "furi_record_open"]
    pub fn record_open(name: *const c_char) -> *mut c_void;
    #[link_name = "furi_record_close"]
    pub fn record_close(name: *const c_char);
}
