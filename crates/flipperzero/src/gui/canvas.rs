//! Canvases.

use flipperzero_sys as sys;

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl Align {
    pub fn to_sys(&self) -> sys::Align {
        match self {
            Self::Left => sys::AlignLeft,
            Self::Right => sys::AlignRight,
            Self::Top => sys::AlignTop,
            Self::Bottom => sys::AlignBottom,
            Self::Center => sys::AlignCenter,
        }
    }
}
