//! Furi mutex.

use crate::opaque;
use crate::furi::base::Status;
use crate::furi::thread::FuriThreadId;

opaque!(FuriMutex);

#[repr(C)]
pub enum Type {
    Normal,
    Recursive,
}

extern "C" {
    #[link_name="furi_mutex_alloc"]
    pub fn alloc(type_: Type) -> *mut FuriMutex;
    #[link_name="furi_mutex_free"]
    pub fn free(instance: *mut FuriMutex);
    #[link_name="furi_mutex_acquire"]
    pub fn acquire(instance: *mut FuriMutex, timeout: u32) -> Status;
    #[link_name="furi_mutex_release"]
    pub fn release(instance: *mut FuriMutex) -> Status;
    #[link_name="furi_mutex_get_owner"]
    pub fn get_owner(instance: *mut FuriMutex) -> FuriThreadId;
}
