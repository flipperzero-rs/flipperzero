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
            Self::Left => sys::Align_AlignLeft,
            Self::Right => sys::Align_AlignRight,
            Self::Top => sys::Align_AlignTop,
            Self::Bottom => sys::Align_AlignBottom,
            Self::Center => sys::Align_AlignCenter,
        }
    }
}
