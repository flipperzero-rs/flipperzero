//! GUI example for Flipper Zero.
//! This app write "Hello, Rust!" to the display.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::c_void;
use core::ptr;
use core::time::Duration;

use flipperzero::furi::thread::sleep;
use flipperzero_gui::gui::{Gui, GuiLayer};
use flipperzero_gui::view_port::ViewPort;
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

manifest!(name = "Rust GUI example");
entry!(main);

/// View draw handler.
///
/// # Safety
///
/// `canvas` should be a valid pointer to [`sys::Canvas`]
pub unsafe extern "C" fn draw_callback(canvas: *mut sys::Canvas, _context: *mut c_void) {
    unsafe {
        sys::canvas_draw_str(canvas, 39, 31, sys::c_string!("Hello, Rust!"));
    }
}

fn main(_args: *mut u8) -> i32 {
    // Currently there is no high level GUI bindings,
    // so this all has to be done using the `sys` bindings.
    let view_port = ViewPort::new().into_raw();
    let view_port = unsafe {
        sys::view_port_draw_callback_set(view_port.as_ptr(), Some(draw_callback), ptr::null_mut());
        ViewPort::from_raw(view_port)
    };

    let mut gui = Gui::new();
    let mut view_port = gui.add_view_port(view_port, GuiLayer::Fullscreen);

    sleep(Duration::from_secs(1));

    view_port.view_port_mut().set_enabled(false);

    0
}
