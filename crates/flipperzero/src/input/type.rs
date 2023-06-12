use crate::internals::macros::impl_std_error;
use core::ffi::CStr;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, InputType as SysInputType};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

/// Input type of a Flipper's button describing
/// the kind of action on it (physical or logical).
///
/// Corresponds to raw [`SysInputType`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum InputType {
    /// Press event, emitted after debounce.
    Press,
    /// Release event, emitted after debounce.
    Release,
    /// Short event, emitted after [`InputType::Release`]
    /// done within `INPUT_LONG_PRESS` interval.
    Short,
    /// Long event, emitted after `INPUT_LONG_PRESS_COUNTS` interval,
    /// asynchronous to [`InputTypeRelease`].
    Long,
    /// Repeat event, emitted with `INPUT_LONG_PRESS_COUNTS` period
    /// after [InputType::Long] event.
    Repeat,
}

impl InputType {
    /// Gets the name of this input type.
    /// Unlike `Debug` and `uDebug` which use Rust enu name,
    /// this relies on Flipper's API intended for this purpose.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::input::InputType;
    /// assert_eq!(InputType::Release.name(), "Release");
    /// ```
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

/// An error which may occur while trying
/// to convert raw [`SysInputType`] to [`InputType`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysInputTypeError {
    /// The [`SysInputType`] is [`MAX`][sys::InputType_InputTypeMAX]
    /// which is a meta-value used to track enum size.
    Max,
    /// The [`SysInputType`] is an invalid value
    /// other than [`MAX`][sys::InputType_InputTypeMAX].
    Invalid(SysInputType),
}

impl Display for FromSysInputTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max => write!(
                f,
                "input key ID {} (Max) is a meta-value",
                sys::InputType_InputTypeMAX,
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
                sys::InputType_InputTypeMAX,
            ),
            Self::Invalid(id) => uwrite!(f, "input key ID {} is invalid", id),
        }
    }
}

impl_std_error!(FromSysInputTypeError);
