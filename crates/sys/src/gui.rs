//! Low-level bindings to GUI service.

use core::ffi::c_char;

use crate::{opaque, c_string};

use super::view_port::ViewPort;

pub const RECORD_GUI: *const c_char = c_string!("gui");

opaque!(Gui);

#[repr(C)]
#[non_exhaustive]
pub enum GuiLayer {
    Desktop,
    Window,
    StatusBarLeft,
    StatusBarRight,
    Fullscreen,

    /// Do not use/move, special value
    MAX,
}

extern "C" {
    #[link_name = "gui_add_view_port"]
    pub fn add_view_port(gui: *mut Gui, view_port: *mut ViewPort, layer: GuiLayer);
    #[link_name = "gui_remove_view_port"]
    pub fn remove_view_port(gui: *mut Gui, view_port: *mut ViewPort);
}
