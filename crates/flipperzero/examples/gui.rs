//! Demonstrates use of the Flipper Zero GUI.
//!
//! This app writes "Hello, Rust!" to the display.
//!
//! Currently uses unsafe `sys` bindings as there is no high level GUI API yet.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use core::ffi::{c_char, c_void};
use core::ptr;
use core::time::Duration;

use flipperzero::furi::thread::sleep;
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

// GUI record
const RECORD_GUI: *const c_char = sys::c_string!("gui");
const FULLSCREEN: sys::GuiLayer = sys::GuiLayer_GuiLayerFullscreen;

manifest!(name = "Rust GUI example");
entry!(main);

/// View draw handler.
pub unsafe extern "C" fn draw_callback(canvas: *mut sys::Canvas, _context: *mut c_void) {
    unsafe {
        sys::canvas_draw_str(canvas, 39, 31, sys::c_string!("Hello, Rust!"));
    }
}

fn main(_args: *mut u8) -> i32 {
    // Currently there is no high level GUI bindings,
    // so this all has to be done using the `sys` bindings.
    unsafe {
        let view_port = sys::view_port_alloc();
        sys::view_port_draw_callback_set(view_port, Some(draw_callback), ptr::null_mut());

        let gui = sys::furi_record_open(RECORD_GUI) as *mut sys::Gui;
        sys::gui_add_view_port(gui, view_port, FULLSCREEN);

        sleep(Duration::from_secs(1));

        sys::view_port_enabled_set(view_port, false);
        sys::gui_remove_view_port(gui, view_port);
        sys::furi_record_close(RECORD_GUI);
        sys::view_port_free(view_port);
    }

    0
}
