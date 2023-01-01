//! Canvases.

use flipperzero_sys::{self as sys, Align as SysAlign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysAlign {
    Invalid(SysAlign),
}

impl TryFrom<SysAlign> for Align {
    type Error = FromSysAlign;

    fn try_from(value: SysAlign) -> Result<Self, Self::Error> {
        use sys::{
            Align_AlignBottom as SYS_ALIGN_BOTTOM, Align_AlignCenter as SYS_ALIGN_CENTER,
            Align_AlignLeft as SYS_ALIGN_LEFT, Align_AlignRight as SYS_ALIGN_RIGHT,
            Align_AlignTop as SYS_ALIGN_TOP,
        };

        Ok(match value {
            SYS_ALIGN_LEFT => Self::Left,
            SYS_ALIGN_RIGHT => Self::Right,
            SYS_ALIGN_TOP => Self::Top,
            SYS_ALIGN_BOTTOM => Self::Bottom,
            SYS_ALIGN_CENTER => Self::Center,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Align> for SysAlign {
    fn from(value: Align) -> Self {
        use sys::{
            Align_AlignBottom as SYS_ALIGN_BOTTOM, Align_AlignCenter as SYS_ALIGN_CENTER,
            Align_AlignLeft as SYS_ALIGN_LEFT, Align_AlignRight as SYS_ALIGN_RIGHT,
            Align_AlignTop as SYS_ALIGN_TOP,
        };

        match value {
            Align::Left => SYS_ALIGN_LEFT,
            Align::Right => SYS_ALIGN_RIGHT,
            Align::Top => SYS_ALIGN_TOP,
            Align::Bottom => SYS_ALIGN_BOTTOM,
            Align::Center => SYS_ALIGN_CENTER,
        }
    }
}
