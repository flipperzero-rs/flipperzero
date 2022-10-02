//! Furi memory management API.

use core::ffi::c_void;

extern "C" {
    #[link_name = "memmgr_get_free_heap"]
    pub fn get_free_heap() -> usize;
    #[link_name = "memmgr_get_total_heap"]
    pub fn get_total_heap() -> usize;
    #[link_name = "memmgr_get_minimum_free_heap"]
    pub fn get_minimum_free_heap() -> usize;

    pub fn aligned_malloc(size: usize, alignment: usize) -> *mut c_void;
    pub fn aligned_free(p: *mut c_void);
}
