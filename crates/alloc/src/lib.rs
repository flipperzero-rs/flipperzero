//! Alloc support for the Flipper Zero.
//! *Note:* This currently requires using nightly.

#![no_std]
#![feature(alloc_error_handler)]

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

use flipperzero_sys as sys;
use sys::c_string;

pub struct FuriAlloc;

unsafe impl GlobalAlloc for FuriAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        sys::furi::memmgr::aligned_malloc(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        sys::furi::memmgr::aligned_free(ptr as *mut c_void);
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
        sys::furi::thread::yield_();
        sys::furi::check::crash(c_string!("Rust: Out of Memory\r\n"))
    }
}
