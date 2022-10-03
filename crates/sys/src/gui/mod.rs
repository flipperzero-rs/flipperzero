//! Low-level bindings to GUI service.

use crate::{c_string, opaque};
use core::ffi::c_char;
use core::fmt::Display;

pub mod canvas;
pub mod elements;
pub mod variable_item_list;
pub mod view;
pub mod view_dispatcher;
pub mod view_port;

pub const RECORD_GUI: *const c_char = c_string!("gui");

opaque!(Gui);

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputEvent {
    pub sequence: u32,
    pub key: InputKey,
    pub event_type: InputType,
}

impl Display for InputEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "InputEvent(seq={}, key={}, type={})",
            self.sequence, self.key, self.event_type
        )
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

extern "C" {
    #[link_name = "gui_add_view_port"]
    pub fn add_view_port(gui: *mut Gui, view_port: *mut view_port::ViewPort, layer: GuiLayer);
    #[link_name = "gui_remove_view_port"]
    pub fn remove_view_port(gui: *mut Gui, view_port: *mut view_port::ViewPort);
}
