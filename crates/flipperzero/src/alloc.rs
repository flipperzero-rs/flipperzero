use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

use flipperzero_sys::c_string;
use flipperzero_sys::furi;

extern "C" {
    #[link_name = "free"]
    pub fn free(p: *mut c_void);

    #[link_name = "aligned_malloc"]
    pub fn aligned_malloc(size: usize, align: usize) -> *mut *mut c_void;

    #[link_name = "realloc"]
    pub fn realloc(p: *mut c_void, size: usize) -> *mut c_void;
}

pub struct FuriAlloc;

unsafe impl GlobalAlloc for FuriAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        aligned_malloc(
            layout.size(),
            layout.align().max(core::mem::size_of::<usize>()),
        ) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // https://github.com/flipperdevices/flipperzero-firmware/issues/1747#issuecomment-1253636552
        self.alloc(layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr as *mut c_void, new_size) as *mut u8
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