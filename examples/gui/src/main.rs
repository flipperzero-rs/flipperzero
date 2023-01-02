//! GUI example for Flipper Zero.
//! This app write "Hello, Rust!" to the display.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;
// Alloc
extern crate alloc;
extern crate flipperzero_alloc;

use core::time::Duration;

use flipperzero::furi::thread::sleep;
use flipperzero_gui::gui::{Gui, GuiLayer};
use flipperzero_gui::view_port::{ViewPort, ViewPortCallbacks};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;
use flipperzero_sys::Canvas;

manifest!(name = "Rust GUI example");
entry!(main);

fn main(_args: *mut u8) -> i32 {
    let view_port = new_view_port();
    let mut gui = Gui::new();
    let mut view_port = gui.add_view_port(view_port, GuiLayer::Fullscreen);

    sleep(Duration::from_secs(1));

    view_port.view_port_mut().set_enabled(false);

    0
}

fn new_view_port() -> ViewPort<impl ViewPortCallbacks> {
    let mut view_port = ViewPort::new();

    struct Callbacks;

    impl ViewPortCallbacks for Callbacks {
        fn on_draw(&mut self, canvas: *mut Canvas) {
            // # SAFETY: `canvas` should be a valid pointer
            unsafe {
                sys::canvas_draw_str(canvas, 39, 31, sys::c_string!("Hello, Rust!"));
            }
        }
    }
    view_port.set_callbacks(Callbacks);

    view_port
}
