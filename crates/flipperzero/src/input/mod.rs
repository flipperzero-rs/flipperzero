use core::ffi::CStr;
use flipperzero_sys::{
    self as sys, InputEvent as SysInputEvent, InputKey as SysInputKey, InputType as SysInputType,
};
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
        use sys::{
            InputType_InputTypeLong as SYS_INPUT_TYPE_LONG,
            InputType_InputTypeMAX as SYS_INPUT_TYPE_MAX,
            InputType_InputTypePress as SYS_INPUT_TYPE_PRESS,
            InputType_InputTypeRelease as SYS_INPUT_TYPE_RELEASE,
            InputType_InputTypeRepeat as SYS_INPUT_TYPE_REPEAT,
            InputType_InputTypeShort as SYS_INPUT_TYPE_SHORT,
        };

        Ok(match value {
            SYS_INPUT_TYPE_PRESS => Self::Press,
            SYS_INPUT_TYPE_RELEASE => Self::Release,
            SYS_INPUT_TYPE_SHORT => Self::Short,
            SYS_INPUT_TYPE_LONG => Self::Long,
            SYS_INPUT_TYPE_REPEAT => Self::Repeat,
            SYS_INPUT_TYPE_MAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<InputType> for SysInputType {
    fn from(value: InputType) -> Self {
        use sys::{
            InputType_InputTypeLong as SYS_INPUT_TYPE_LONG,
            InputType_InputTypePress as SYS_INPUT_TYPE_PRESS,
            InputType_InputTypeRelease as SYS_INPUT_TYPE_RELEASE,
            InputType_InputTypeRepeat as SYS_INPUT_TYPE_REPEAT,
            InputType_InputTypeShort as SYS_INPUT_TYPE_SHORT,
        };

        match value {
            InputType::Press => SYS_INPUT_TYPE_PRESS,
            InputType::Release => SYS_INPUT_TYPE_RELEASE,
            InputType::Short => SYS_INPUT_TYPE_SHORT,
            InputType::Long => SYS_INPUT_TYPE_LONG,
            InputType::Repeat => SYS_INPUT_TYPE_REPEAT,
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
        use sys::{
            InputKey_InputKeyBack as SYS_INPUT_KEY_BACK,
            InputKey_InputKeyDown as SYS_INPUT_KEY_DOWN,
            InputKey_InputKeyLeft as SYS_INPUT_KEY_LEFT, InputKey_InputKeyMAX as SYS_INPUT_KEY_MAX,
            InputKey_InputKeyOk as SYS_INPUT_KEY_OK, InputKey_InputKeyRight as SYS_INPUT_KEY_RIGHT,
            InputKey_InputKeyUp as SYS_INPUT_KEY_UP,
        };

        Ok(match value {
            SYS_INPUT_KEY_UP => Self::Up,
            SYS_INPUT_KEY_DOWN => Self::Down,
            SYS_INPUT_KEY_RIGHT => Self::Right,
            SYS_INPUT_KEY_LEFT => Self::Left,
            SYS_INPUT_KEY_OK => Self::Ok,
            SYS_INPUT_KEY_BACK => Self::Back,
            SYS_INPUT_KEY_MAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<InputKey> for SysInputKey {
    fn from(value: InputKey) -> Self {
        use sys::{
            InputKey_InputKeyBack as SYS_INPUT_KEY_BACK,
            InputKey_InputKeyDown as SYS_INPUT_KEY_DOWN,
            InputKey_InputKeyLeft as SYS_INPUT_KEY_LEFT, InputKey_InputKeyOk as SYS_INPUT_KEY_OK,
            InputKey_InputKeyRight as SYS_INPUT_KEY_RIGHT, InputKey_InputKeyUp as SYS_INPUT_KEY_UP,
        };

        match value {
            InputKey::Up => SYS_INPUT_KEY_UP,
            InputKey::Down => SYS_INPUT_KEY_DOWN,
            InputKey::Right => SYS_INPUT_KEY_RIGHT,
            InputKey::Left => SYS_INPUT_KEY_LEFT,
            InputKey::Ok => SYS_INPUT_KEY_OK,
            InputKey::Back => SYS_INPUT_KEY_BACK,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysInputKeyError {
    Max,
    Invalid(SysInputKey),
}
