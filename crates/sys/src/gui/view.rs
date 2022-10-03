//! Low-level bindings to the View API.

use crate::opaque;
use core::ffi::c_void;

opaque!(View);

pub type DrawCallback = extern "C" fn(*mut super::canvas::Canvas, *mut c_void);
pub type InputCallback = extern "C" fn(*mut super::InputEvent, *mut c_void) -> bool;
pub type CustomCallback = extern "C" fn(u32, *mut c_void) -> bool;

extern "C" {
    #[link_name = "view_alloc"]
    pub fn alloc() -> *mut View;
    #[link_name = "view_free"]
    pub fn free(view: *mut View);
    #[link_name = "view_set_context"]
    pub fn set_context(view: *mut View, context: *mut c_void);

    #[link_name = "view_set_draw_callback"]
    pub fn set_draw_callback(view: *mut View, callback: DrawCallback);
    #[link_name = "view_set_input_callback"]
    pub fn set_input_callback(view: *mut View, callback: InputCallback);
    #[link_name = "view_set_custom_callback"]
    pub fn set_custom_callback(view: *mut View, callback: CustomCallback);
}