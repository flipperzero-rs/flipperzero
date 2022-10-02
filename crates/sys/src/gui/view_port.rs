//! Low-level bindings to ViewPort API.

use core::ffi::c_void;

use super::canvas::Canvas;
use super::InputEvent;
use crate::opaque;

opaque!(ViewPort);

pub type DrawCallback = extern "C" fn(*mut Canvas, *mut c_void);
pub type InputCallback = extern "C" fn(*mut InputEvent, *mut c_void);

extern "C" {
    #[link_name = "view_port_alloc"]
    pub fn alloc() -> *mut ViewPort;
    #[link_name = "view_port_free"]
    pub fn free(view_port: *mut ViewPort);
    #[link_name = "view_port_enabled_set"]
    pub fn enabled_set(view_port: *mut ViewPort, enabled: bool);
    #[link_name = "view_port_draw_callback_set"]
    pub fn draw_callback_set(
        view_port: *mut ViewPort,
        callback: DrawCallback,
        context: *mut c_void,
    );
    #[link_name = "view_port_input_callback_set"]
    pub fn input_callback_set(
        view_port: *mut ViewPort,
        callback: InputCallback,
        context: *mut c_void,
    );
    #[link_name = "view_port_update"]
    pub fn update(view_port: *mut ViewPort);
}
