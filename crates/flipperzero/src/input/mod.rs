use core::ffi::CStr;
use flipperzero_sys::{
    self as sys, InputEvent as SysInputEvent, InputKey as SysInputKey, InputType as SysInputType,
};
// public type alias for an anonymous union
pub use sys::InputEvent__bindgen_ty_1 as SysInputEventSequence;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct InputEvent {
    pub sequence: InputEventSequence,
    pub key: InputKey,
    pub r#type: InputType,
}

impl<'a> TryFrom<&'a SysInputEvent> for InputEvent {
    type Error = FromSysInputEventError;

    fn try_from(value: &'a SysInputEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            sequence: value.__bindgen_anon_1.into(),
            key: value.key.try_into()?,
            r#type: value.type_.try_into()?,
        })
    }
}

impl From<InputEvent> for SysInputEvent {
    fn from(value: InputEvent) -> Self {
        Self {
            __bindgen_anon_1: value.sequence.into(),
            key: value.key.into(),
            type_: value.r#type.into(),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct InputEventSequence(u32);

impl InputEventSequence {
    const fn source(&self) -> u8 {
        ((self.0 >> 30) & 0b11) as u8
    }

    const fn counter(&self) -> u32 {
        self.0 & !(0b11 << 30)
    }
}

impl From<u32> for InputEventSequence {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<InputEventSequence> for u32 {
    fn from(value: InputEventSequence) -> Self {
        value.0
    }
}

impl From<SysInputEventSequence> for InputEventSequence {
    fn from(value: SysInputEventSequence) -> Self {
        // SAFETY: both union variants are always valid
        // and the bit-field one is just a typed view over the plain one
        Self(unsafe { value.sequence })
    }
}

impl From<InputEventSequence> for SysInputEventSequence {
    fn from(value: InputEventSequence) -> Self {
        Self { sequence: value.0 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysInputEventError {
    InvalidKey(FromSysInputKeyError),
    InvalidType(FromSysInputTypeError),
}

impl From<FromSysInputKeyError> for FromSysInputEventError {
    fn from(value: FromSysInputKeyError) -> Self {
        Self::InvalidKey(value)
    }
}

impl From<FromSysInputTypeError> for FromSysInputEventError {
    fn from(value: FromSysInputTypeError) -> Self {
        Self::InvalidType(value)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysInputTypeError {
    Max,
    Invalid(SysInputType),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysInputKeyError {
    Max,
    Invalid(SysInputKey),
}
