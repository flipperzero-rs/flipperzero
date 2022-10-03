//! Low-level bindings to Elements API.

use core::ffi::c_char;
use super::canvas::{Align, Canvas};

extern "C" {
    #[link_name = "elements_button_left"]
    pub fn button_left(canvas: *mut Canvas, label: *const c_char);
    #[link_name = "elements_button_center"]
    pub fn button_center(canvas: *mut Canvas, label: *const c_char);
    #[link_name = "elements_button_right"]
    pub fn button_right(canvas: *mut Canvas, label: *const c_char);

    #[link_name = "elements_multiline_text_aligned"]
    pub fn multiline_text_aligned(canvas: *mut Canvas, x: u8, y: u8, horizontal: Align, vertical: Align, label: *const c_char);
}
