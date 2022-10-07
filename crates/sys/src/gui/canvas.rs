//! Low-level bindings to the Canvas API.

use core::ffi::c_char;
use core::fmt::Display;

use crate::opaque;

opaque!(Canvas);

/// Alignment.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Align(pub u8);

impl Align {
    pub const LEFT: Align = Self(0);
    pub const RIGHT: Align = Self(1);
    pub const TOP: Align = Self(2);
    pub const BOTTOM: Align = Self(3);
    pub const CENTER: Align = Self(4);

    /// Describes the alignment indicated.
    pub fn description(self) -> &'static str {
        match self {
            Self::LEFT => "Left",
            Self::RIGHT => "Right",
            Self::TOP => "Top",
            Self::BOTTOM => "Bottom",
            Self::CENTER => "Center",
            _ => "Unknown",
        }
    }
}

impl Display for Align {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Fonts.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Font(pub u8);

impl Font {
    pub const PRIMARY: Font = Self(0);
    pub const SECONDARY: Font = Self(1);
    pub const KEYBOARD: Font = Self(2);
    pub const BIG_NUMBERS: Font = Self(3);

    /// Describes the font selected.
    pub fn description(self) -> &'static str {
        match self {
            Self::PRIMARY => "Primary",
            Self::SECONDARY => "Secondary",
            Self::KEYBOARD => "Keyboard",
            Self::BIG_NUMBERS => "Big numbers",
            _ => "Unknown",
        }
    }
}

impl Display for Font {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

extern "C" {
    #[link_name = "canvas_draw_str"]
    pub fn draw_str(canvas: *mut Canvas, x: u8, y: u8, str: *const c_char);
    #[link_name = "canvas_draw_str_aligned"]
    pub fn draw_str_aligned(
        canvas: *mut Canvas,
        x: u8,
        y: u8,
        horizontal: Align,
        vertical: Align,
        str: *const c_char,
    );
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
