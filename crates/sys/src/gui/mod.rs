//! Low-level bindings to GUI service.

use crate::{c_string, opaque};
use core::ffi::c_char;
use core::fmt::Display;

pub mod canvas;
pub mod elements;
pub mod text_input;
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
    pub const UP: InputKey = Self(0);
    pub const DOWN: InputKey = Self(1);
    pub const RIGHT: InputKey = Self(2);
    pub const LEFT: InputKey = Self(3);
    pub const OK: InputKey = Self(4);
    pub const BACK: InputKey = Self(5);

    /// Describes the key which was pressed.
    pub fn description(self) -> &'static str {
        match self {
            Self::UP => "Up",
            Self::DOWN => "Down",
            Self::RIGHT => "Right",
            Self::LEFT => "Left",
            Self::OK => "Ok",
            Self::BACK => "Back",
            _ => "Unknown",
        }
    }
}

impl Display for InputKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputType(pub u8);

impl InputType {
    pub const PRESS: InputType = Self(0);
    pub const RELEASE: InputType = Self(1);
    pub const SHORT: InputType = Self(2);
    pub const LONG: InputType = Self(3);
    pub const REPEAT: InputType = Self(4);

    /// Describes the type of event.
    pub fn description(self) -> &'static str {
        match self {
            Self::PRESS => "Press",
            Self::RELEASE => "Release",
            Self::SHORT => "Short",
            Self::LONG => "Long",
            Self::REPEAT => "Repeat",
            _ => "Unknown",
        }
    }
}

impl Display for InputType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

extern "C" {
    #[link_name = "gui_add_view_port"]
    pub fn add_view_port(gui: *mut Gui, view_port: *mut view_port::ViewPort, layer: GuiLayer);
    #[link_name = "gui_remove_view_port"]
    pub fn remove_view_port(gui: *mut Gui, view_port: *mut view_port::ViewPort);
}
