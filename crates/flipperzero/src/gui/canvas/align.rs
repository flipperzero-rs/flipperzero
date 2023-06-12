use crate::internals::macros::impl_std_error;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, Align as SysAlign};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

/// Alignment of an object on the canvas.
///
/// Corresponds to raw [`SysAlign`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Align {
    /// The values are aligned relative to the right.
    Left,
    /// The values are aligned relative to the left.
    Right,
    /// The values are aligned relative to the top.
    Top,
    /// The values are aligned relative to the bottom.
    Bottom,
    /// The values are aligned relative to the center.
    Center,
}

impl TryFrom<SysAlign> for Align {
    type Error = FromSysAlignError;

    fn try_from(value: SysAlign) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::Align_AlignLeft => Self::Left,
            sys::Align_AlignRight => Self::Right,
            sys::Align_AlignTop => Self::Top,
            sys::Align_AlignBottom => Self::Bottom,
            sys::Align_AlignCenter => Self::Center,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Align> for SysAlign {
    fn from(value: Align) -> Self {
        match value {
            Align::Left => sys::Align_AlignLeft,
            Align::Right => sys::Align_AlignRight,
            Align::Top => sys::Align_AlignTop,
            Align::Bottom => sys::Align_AlignBottom,
            Align::Center => sys::Align_AlignCenter,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysAlign`] to [`Align`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysAlignError {
    /// The [`SysAlign`] is an invalid value.
    Invalid(SysAlign),
}

impl Display for FromSysAlignError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self::Invalid(id) = self;
        write!(f, "align ID {id} is invalid")
    }
}

impl uDisplay for FromSysAlignError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "align ID {} is invalid", id)
    }
}

impl_std_error!(FromSysAlignError);
