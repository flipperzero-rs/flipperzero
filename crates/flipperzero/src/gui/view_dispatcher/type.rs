use crate::internals::macros::impl_std_error;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, ViewDispatcherType as SysViewDispatcherType};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

/// View dispatcher view port placement.
///
/// Corresponds to raw [`SysViewDispatcherType`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ViewDispatcherType {
    /// Desktop layer: fullscreen with status bar on top of it. For internal usage.
    Desktop,
    /// Window layer: with status bar.
    Window,
    /// Fullscreen layer: without status bar.
    Fullscreen,
}

impl TryFrom<SysViewDispatcherType> for ViewDispatcherType {
    type Error = FromSysViewDispatcherTypeError;

    fn try_from(value: SysViewDispatcherType) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::ViewDispatcherType_ViewDispatcherTypeDesktop => ViewDispatcherType::Desktop,
            sys::ViewDispatcherType_ViewDispatcherTypeWindow => ViewDispatcherType::Window,
            sys::ViewDispatcherType_ViewDispatcherTypeFullscreen => ViewDispatcherType::Fullscreen,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<ViewDispatcherType> for SysViewDispatcherType {
    fn from(value: ViewDispatcherType) -> Self {
        match value {
            ViewDispatcherType::Desktop => sys::ViewDispatcherType_ViewDispatcherTypeDesktop,
            ViewDispatcherType::Window => sys::ViewDispatcherType_ViewDispatcherTypeWindow,
            ViewDispatcherType::Fullscreen => sys::ViewDispatcherType_ViewDispatcherTypeFullscreen,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysViewDispatcherType`] to [`ViewDispatcherType`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysViewDispatcherTypeError {
    /// The [`SysViewDispatcherType`] is an invalid value.
    Invalid(SysViewDispatcherType),
}

impl Display for FromSysViewDispatcherTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self::Invalid(id) = self;
        write!(f, "view dispatcher type ID {id} is invalid")
    }
}

impl uDisplay for FromSysViewDispatcherTypeError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "view dispatcher type ID {} is invalid", id)
    }
}

impl_std_error!(FromSysInputTypeError);
