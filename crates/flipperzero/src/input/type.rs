use crate::internals::macros::impl_std_error;
use core::ffi::CStr;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, InputType as SysInputType};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum InputType {
    Press,
    Release,
    Short,
    Long,
    Repeat,
}

impl InputType {
    pub fn name(self) -> &'static CStr {
        let this = SysInputType::from(self);
        // SAFETY: `this` is always a valid enum value
        // and the returned string is a static string
        unsafe { CStr::from_ptr(sys::input_get_type_name(this)) }
    }
}

impl TryFrom<SysInputType> for InputType {
    type Error = FromSysInputTypeError;

    fn try_from(value: SysInputType) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::InputType_InputTypePress => Self::Press,
            sys::InputType_InputTypeRelease => Self::Release,
            sys::InputType_InputTypeShort => Self::Short,
            sys::InputType_InputTypeLong => Self::Long,
            sys::InputType_InputTypeRepeat => Self::Repeat,
            sys::InputType_InputTypeMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<InputType> for SysInputType {
    fn from(value: InputType) -> Self {
        match value {
            InputType::Press => sys::InputType_InputTypePress,
            InputType::Release => sys::InputType_InputTypeRelease,
            InputType::Short => sys::InputType_InputTypeShort,
            InputType::Long => sys::InputType_InputTypeLong,
            InputType::Repeat => sys::InputType_InputTypeRepeat,
        }
    }
}

#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysInputTypeError {
    Max,
    Invalid(SysInputType),
}

impl Display for FromSysInputTypeError {
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

impl uDisplay for FromSysInputTypeError {
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

impl_std_error!(FromSysInputTypeError);
