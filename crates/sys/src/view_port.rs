//! Low-level bindings to ViewPort API.

use core::ffi::c_void;
use core::fmt::Display;

use crate::opaque;

use super::canvas::Canvas;

opaque!(ViewPort);

#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputEvent {
    pub sequence: u32,
    pub key: InputKey,
    pub event_type: InputType,
}

impl Display for InputEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "InputEvent(seq={}, key={}, type={})", self.sequence, self.key, self.event_type)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputKey(pub u8);

impl InputKey {
    /// Describes the key which was pressed.
    pub fn description(self) -> &'static str {
        use keys::*;

        match self {
            UP => "Up",
            DOWN => "Down",
            RIGHT => "Right",
            LEFT => "Left",
            OK => "Ok",
            BACK => "Back",
            _ => "Unknown",
        }
    }
}

impl Display for InputKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Input keys.
pub mod keys {
    use super::InputKey;

    pub const UP: InputKey = InputKey(0);
    pub const DOWN: InputKey = InputKey(1);
    pub const RIGHT: InputKey = InputKey(2);
    pub const LEFT: InputKey = InputKey(3);
    pub const OK: InputKey = InputKey(4);
    pub const BACK: InputKey = InputKey(5);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputType(pub u8);


impl InputType {
    /// Describes the type of event.
    pub fn description(self) -> &'static str {
        use input_types::*;

        match self {
            PRESS => "Press",
            RELEASE => "Release",
            SHORT => "Short",
            LONG => "Long",
            REPEAT => "Repeat",
            _ => "Unknown",
        }
    }
}

impl Display for InputType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Input types.
pub mod input_types {
    use super::InputType;

    pub const PRESS: InputType = InputType(0);
    pub const RELEASE: InputType = InputType(1);
    pub const SHORT: InputType = InputType(2);
    pub const LONG: InputType = InputType(3);
    pub const REPEAT: InputType = InputType(4);
}


pub type ViewPortDrawCallback = extern fn(*mut Canvas, *mut c_void);
pub type ViewPortInputCallback = extern fn(*mut InputEvent, *mut c_void);

extern "C" {
    #[link_name = "view_port_alloc"]
    pub fn alloc() -> *mut ViewPort;
    #[link_name = "view_port_free"]
    pub fn free(view_port: *mut ViewPort);
    #[link_name = "view_port_enabled_set"]
    pub fn enabled_set(view_port: *mut ViewPort, enabled: bool);
    #[link_name = "view_port_draw_callback_set"]
    pub fn draw_callback_set(view_port: *mut ViewPort, callback: ViewPortDrawCallback, context: *mut c_void);
    #[link_name = "view_port_input_callback_set"]
    pub fn input_callback_set(view_port: *mut ViewPort, callback: ViewPortInputCallback, context: *mut c_void);
    #[link_name = "view_port_update"]
    pub fn update(view_port: *mut ViewPort);
}
