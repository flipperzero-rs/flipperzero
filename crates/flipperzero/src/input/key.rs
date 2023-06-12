use crate::internals::macros::impl_std_error;
use core::ffi::CStr;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, InputKey as SysInputKey};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum InputKey {
    Up,
    Down,
    Right,
    Left,
    Ok,
    Back,
}

impl InputKey {
    pub fn name(self) -> &'static CStr {
        let this = SysInputKey::from(self);
        // SAFETY: `this` is always a valid enum value
        // and the returned string is a static string
        unsafe { CStr::from_ptr(sys::input_get_key_name(this)) }
    }
}

impl TryFrom<SysInputKey> for InputKey {
    type Error = FromSysInputKeyError;

    fn try_from(value: SysInputKey) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::InputKey_InputKeyUp => Self::Up,
            sys::InputKey_InputKeyDown => Self::Down,
            sys::InputKey_InputKeyRight => Self::Right,
            sys::InputKey_InputKeyLeft => Self::Left,
            sys::InputKey_InputKeyOk => Self::Ok,
            sys::InputKey_InputKeyBack => Self::Back,
            sys::InputKey_InputKeyMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<InputKey> for SysInputKey {
    fn from(value: InputKey) -> Self {
        match value {
            InputKey::Up => sys::InputKey_InputKeyUp,
            InputKey::Down => sys::InputKey_InputKeyDown,
            InputKey::Right => sys::InputKey_InputKeyRight,
            InputKey::Left => sys::InputKey_InputKeyLeft,
            InputKey::Ok => sys::InputKey_InputKeyOk,
            InputKey::Back => sys::InputKey_InputKeyBack,
        }
    }
}

#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysInputKeyError {
    Max,
    Invalid(SysInputKey),
}

impl Display for FromSysInputKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max => write!(
                f,
                "input key ID {} (Max) is a meta-value",
                sys::Font_FontTotalNumber,
            ),
            Self::Invalid(id) => write!(f, "input key ID {id} is invalid"),
        }
    }
}

impl uDisplay for FromSysInputKeyError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(
                f,
                "input key ID {} (Max) is a meta-value",
                sys::Font_FontTotalNumber,
            ),
            Self::Invalid(id) => uwrite!(f, "input key ID {} is invalid", id),
        }
    }
}

impl_std_error!(FromSysInputKeyError);
