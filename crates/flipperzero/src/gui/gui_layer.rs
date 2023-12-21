use core::fmt::{self, Display, Formatter};

use flipperzero_sys::{self as sys, GuiLayer as SysGuiLayer};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

use crate::internals::macros::impl_std_error;

/// The font used to draw text.
///
/// Corresponds to raw [`SysGuiLayer`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GuiLayer {
    /// Desktop layer for internal use. Like fullscreen but with status bar.
    Desktop,
    /// Window layer, status bar is shown.
    Window,
    /// Status bar left-side layer, auto-layout.
    StatusBarLeft,
    /// Status bar right-side layer, auto-layout
    StatusBarRight,
    /// Fullscreen layer, no status bar.
    Fullscreen,
}

impl TryFrom<SysGuiLayer> for GuiLayer {
    type Error = FromSysGuiLayerError;

    fn try_from(value: SysGuiLayer) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::GuiLayer_GuiLayerDesktop => Self::Desktop,
            sys::GuiLayer_GuiLayerWindow => Self::Window,
            sys::GuiLayer_GuiLayerStatusBarLeft => Self::StatusBarLeft,
            sys::GuiLayer_GuiLayerStatusBarRight => Self::StatusBarRight,
            sys::GuiLayer_GuiLayerFullscreen => Self::Fullscreen,
            sys::GuiLayer_GuiLayerMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<GuiLayer> for SysGuiLayer {
    fn from(value: GuiLayer) -> Self {
        match value {
            GuiLayer::Desktop => sys::GuiLayer_GuiLayerDesktop,
            GuiLayer::Window => sys::GuiLayer_GuiLayerWindow,
            GuiLayer::StatusBarLeft => sys::GuiLayer_GuiLayerStatusBarLeft,
            GuiLayer::StatusBarRight => sys::GuiLayer_GuiLayerStatusBarRight,
            GuiLayer::Fullscreen => sys::GuiLayer_GuiLayerFullscreen,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysGuiLayer`] to [`GuiLayer`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysGuiLayerError {
    /// The [`SysGuiLayer`] is [`MAX`][sys::GuiLayer_GuiLayerMAX]
    /// which is a meta-value used to track enum size.
    Max,
    /// The [`SysGuiLayer`] is an invalid value
    /// other than [`MAX`][sys::GuiLayer_GuiLayerMAX].
    Invalid(SysGuiLayer),
}

impl Display for FromSysGuiLayerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max => write!(
                f,
                "gui layer ID {} (MAX) is a meta-value",
                sys::GuiLayer_GuiLayerMAX,
            ),
            Self::Invalid(id) => write!(f, "gui layer ID {id} is invalid"),
        }
    }
}

impl uDisplay for FromSysGuiLayerError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(
                f,
                "gui layer ID {} (MAX) is a meta-value",
                sys::GuiLayer_GuiLayerMAX,
            ),
            Self::Invalid(id) => uwrite!(f, "gui layer ID {} is invalid", id),
        }
    }
}

impl_std_error!(FromSysGuiLayerError);
