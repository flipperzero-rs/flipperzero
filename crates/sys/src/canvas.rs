//! Low-level bindings to the Canvas API.

use core::ffi::c_char;
use core::fmt::Display;

use crate::opaque;

opaque!(Canvas);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Align(pub u8);

impl Align {
    /// Describes the alignment indicated.
    pub fn description(self) -> &'static str {
        use alignment::*;

        match self {
            LEFT => "Left",
            RIGHT => "Right",
            TOP => "Top",
            BOTTOM => "Bottom",
            CENTER => "Center",
            _ => "Unknown",
        }
    }
}

impl Display for Align {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Align types.
pub mod alignment {
    use super::Align;

    pub const LEFT: Align = Align(0);
    pub const RIGHT: Align = Align(1);
    pub const TOP: Align = Align(2);
    pub const BOTTOM: Align = Align(3);
    pub const CENTER: Align = Align(4);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Font(pub u8);

impl Font {
    /// Describes the font selected.
    pub fn description(self) -> &'static str {
        use font::*;

        match self {
            PRIMARY => "Primary",
            SECONDARY => "Secondary",
            KEYBOARD => "Keyboard",
            BIG_NUMBERS => "Big numbers",
            _ => "Unknown",
        }
    }
}

impl Display for Font {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Builtin fonts.
pub mod font {
    use super::Font;

    pub const PRIMARY: Font = Font(0);
    pub const SECONDARY: Font = Font(1);
    pub const KEYBOARD: Font = Font(2);
    pub const BIG_NUMBERS: Font = Font(3);
}



extern "C" {
    #[link_name = "canvas_draw_str"]
    pub fn draw_str(canvas: *mut Canvas, x: u8, y: u8, str: *const c_char);
    #[link_name = "canvas_draw_str_aligned"]
    pub fn draw_str_aligned(canvas: *mut Canvas, x: u8, y: u8, horizontal: Align, vertical: Align, str: *const c_char);
    #[link_name = "canvas_draw_icon"]
    pub fn draw_icon(canvas: *mut Canvas, x: u8, y: u8, icon: *const crate::icon::Icon);
    #[link_name = "canvas_draw_box"]
    pub fn draw_box(canvas: *mut Canvas, x: u8, y: u8, width: u8, height: u8);
    #[link_name = "canvas_draw_frame"]
    pub fn draw_frame(canvas: *mut Canvas, x: u8, y: u8, width: u8, height: u8);
    #[link_name = "canvas_draw_line"]
    pub fn draw_line(canvas: *mut Canvas, x1: u8, y1: u8, x2: u8, y2: u8);
    #[link_name = "canvas_draw_circle"]
    pub fn draw_circle(canvas: *mut Canvas, x: u8, y: u8, r: u8);
    #[link_name = "canvas_draw_disc"]
    pub fn draw_disc(canvas: *mut Canvas, x: u8, y: u8, r: u8);

    #[link_name = "canvas_draw_rframe"]
    pub fn draw_rounded_frame(canvas: *mut Canvas, x: u8, y: u8, width: u8, height: u8, radius: u8);
    #[link_name = "canvas_draw_rbox"]
    pub fn draw_rounded_box(canvas: *mut Canvas, x: u8, y: u8, width: u8, height: u8, radius: u8);

    #[link_name = "canvas_clear"]
    pub fn clear(canvas: *mut Canvas) -> u8;
    #[link_name = "canvas_set_font"]
    pub fn set_font(canvas: *mut Canvas, font: Font);

    #[link_name = "canvas_height"]
    pub fn height(canvas: *mut Canvas) -> u8;
    #[link_name = "canvas_width"]
    pub fn width(canvas: *mut Canvas) -> u8;
}
