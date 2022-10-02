use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

use flipperzero_sys::c_string;
use flipperzero_sys::furi;

extern "C" {
    fn aligned_free(p: *mut c_void);
    fn aligned_malloc(size: usize, align: usize) -> *mut c_void;
    fn memmgr_get_total_heap() -> usize;
    fn memmgr_get_free_heap() -> usize;
    fn memmgr_get_minimum_free_heap() -> usize;
}

pub struct FuriAlloc;

unsafe impl GlobalAlloc for FuriAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        aligned_malloc(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        aligned_free(ptr as *mut c_void);
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // https://github.com/flipperdevices/flipperzero-firmware/issues/1747#issuecomment-1253636552
        self.alloc(layout)
    }
}

#[global_allocator]
static ALLOCATOR: FuriAlloc = FuriAlloc;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    unsafe {
        furi::thread_yield();
        furi::crash(c_string!("Rust: Out of Memory\r\n"))
    }
}

pub fn get_total_heap() -> usize {
    unsafe { memmgr_get_total_heap() }
}

pub fn get_free_heap() -> usize {
    unsafe { memmgr_get_free_heap() }
}

pub fn get_minimum_free_heap() -> usize {
    unsafe { memmgr_get_minimum_free_heap() }
}

