//! Low-level bindings to the Canvas API.

use core::ffi::c_char;

use crate::opaque;

opaque!(Canvas);

extern "C" {
    #[link_name = "canvas_draw_str"]
    pub fn draw_str(canvas: *mut Canvas, x: u8, y: u8, str: *const c_char);
}
