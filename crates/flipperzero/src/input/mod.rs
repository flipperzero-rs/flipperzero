mod key;
mod r#type;

use flipperzero_sys::{self as sys, InputEvent as SysInputEvent};
// public type alias for an anonymous union
pub use sys::InputEvent__bindgen_ty_1 as SysInputEventSequence;

pub use key::*;
pub use r#type::*;

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
