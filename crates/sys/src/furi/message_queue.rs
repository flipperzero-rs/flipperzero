//! Low-level bindings to Furi message queue API

use core::ffi::c_void;

use crate::furi::base::Status;
use crate::opaque;

opaque!(FuriMessageQueue);

extern "C" {
    #[link_name = "furi_message_queue_alloc"]
    pub fn alloc(count: u32, size: usize) -> *const FuriMessageQueue;

    #[link_name = "furi_message_queue_free"]
    pub fn free(queue: *const FuriMessageQueue);

    #[link_name = "furi_message_queue_put"]
    pub fn put(queue: *const FuriMessageQueue, payload: *const c_void, timeout: u32) -> Status;

    #[link_name = "furi_message_queue_get"]
    pub fn get(queue: *const FuriMessageQueue, payload: *mut c_void, timeout: u32) -> Status;

    #[link_name = "furi_message_queue_get_capacity"]
    pub fn capacity(queue: *const FuriMessageQueue) -> u32;

    #[link_name = "furi_message_queue_get_count"]
    pub fn count(queue: *const FuriMessageQueue) -> u32;

    #[link_name = "furi_message_queue_get_space"]
    pub fn space(queue: *const FuriMessageQueue) -> u32;
}