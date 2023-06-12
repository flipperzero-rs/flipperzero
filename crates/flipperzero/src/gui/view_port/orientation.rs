use crate::internals::macros::impl_std_error;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, ViewPortOrientation as SysViewPortOrientation};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ViewPortOrientation {
    Horizontal,
    HorizontalFlip,
    Vertical,
    VerticalFlip,
}

impl TryFrom<SysViewPortOrientation> for ViewPortOrientation {
    type Error = FromSysViewPortOrientationError;

    fn try_from(value: SysViewPortOrientation) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::ViewPortOrientation_ViewPortOrientationHorizontal => Self::Horizontal,
            sys::ViewPortOrientation_ViewPortOrientationHorizontalFlip => Self::HorizontalFlip,
            sys::ViewPortOrientation_ViewPortOrientationVertical => Self::Vertical,
            sys::ViewPortOrientation_ViewPortOrientationVerticalFlip => Self::VerticalFlip,
            sys::ViewPortOrientation_ViewPortOrientationMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<ViewPortOrientation> for SysViewPortOrientation {
    fn from(value: ViewPortOrientation) -> Self {
        match value {
            ViewPortOrientation::Horizontal => {
                sys::ViewPortOrientation_ViewPortOrientationHorizontal
            }
            ViewPortOrientation::HorizontalFlip => {
                sys::ViewPortOrientation_ViewPortOrientationHorizontalFlip
            }
            ViewPortOrientation::Vertical => sys::ViewPortOrientation_ViewPortOrientationVertical,
            ViewPortOrientation::VerticalFlip => {
                sys::ViewPortOrientation_ViewPortOrientationVerticalFlip
            }
        }
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysViewPortOrientationError {
    Max,
    Invalid(SysViewPortOrientation),
}

impl Display for FromSysViewPortOrientationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max => write!(
                f,
                "view port orientation ID {} (MAX) is a meta-value",
                sys::GuiLayer_GuiLayerMAX,
            ),
            Self::Invalid(id) => write!(f, "view port orientation ID {id} is invalid"),
        }
    }
}

impl uDisplay for FromSysViewPortOrientationError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(
                f,
                "view port orientation ID {} (MAX) is a meta-value",
                sys::GuiLayer_GuiLayerMAX,
            ),
            Self::Invalid(id) => uwrite!(f, "view port orientation ID {} is invalid", id),
        }
    }
}

impl_std_error!(FromSysViewPortOrientationError);
