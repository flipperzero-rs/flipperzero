//! Furi dialogs.

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use core::ffi::c_char;
use core::ptr;

use flipperzero_sys as sys;
use flipperzero_sys::furi::{Status, UnsafeRecord};

///Default notification messages.
pub mod messages;
///Default notification notes.
pub mod notes;
///Default notification sequences.
pub mod sequences;

const RECORD_NOTIFICATION: *const c_char = sys::c_string!("notification");

/// A handle to the Notification app.
pub struct NotificationApp {
    data: UnsafeRecord<sys::NotificationApp>,
}

impl NotificationApp {
    /// Obtains a handle to the Notifications app.
    pub fn open() -> Self {
        Self {
            data: unsafe { UnsafeRecord::open(RECORD_NOTIFICATION) },
        }
    }

    /// Runs a notification sequence.
    #[cfg(feature = "alloc")]
    pub fn notify<const N: usize>(&mut self, sequence: [&'static NotificationMessage; N]) {
        unsafe {
            sys::notification_message(
                self.data.as_ptr(),
                sequence
                    .map(|msg| match msg {
                        NotificationMessage::End => ptr::null(),
                        _ => Box::leak(Box::new(msg.to_sys())) as *const _,
                    })
                    .as_ptr() as *const _,
            )
        };
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Light {
    Off = 0,

    Red = 0b0001,
    Green = 0b0010,
    Blue = 0b0100,
    Backlight = 0b1000,

    Cyan = 0b0110,
    Magenta = 0b0101,
    Yellow = 0b0011,

    White = 0b0111,
}

impl core::ops::BitOr for Light {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Off, l) | (l, Self::Off) => l,

            (Self::Red, Self::Red) => Self::Red,
            (Self::Green, Self::Green) => Self::Green,
            (Self::Blue, Self::Blue) => Self::Blue,
            (Self::Backlight, Self::Backlight) => Self::Backlight,
            (Self::Cyan, Self::Cyan) => Self::Cyan,
            (Self::Magenta, Self::Magenta) => Self::Magenta,
            (Self::Yellow, Self::Yellow) => Self::Yellow,
            (Self::White, Self::White) => Self::White,

            (Self::Red, Self::Green) | (Self::Green, Self::Red) => Self::Yellow,
            (Self::Red, Self::Blue) | (Self::Blue, Self::Red) => Self::Magenta,
            (Self::Green, Self::Blue) | (Self::Blue, Self::Green) => Self::Cyan,

            _ => Self::White, //FIXME undefined?
        }
    }
}

impl Light {
    pub fn to_sys(&self) -> sys::Light {
        match self {
            Self::Off => 0,

            Self::Red => 0b0001,
            Self::Green => 0b0010,
            Self::Blue => 0b0100,
            Self::Backlight => 0b1000,

            Self::Cyan => 0b0110,
            Self::Magenta => 0b0101,
            Self::Yellow => 0b0011,

            Self::White => 0b0111,
        }
    }
}

/// A notification message.
pub enum NotificationMessage {
    Vibro(bool),
    SoundOn { frequency: f32, volume: f32 },
    SoundOff,

    LedRed(u8),
    LedGreen(u8),
    LedBlue(u8),

    LedBlinkStart { on_time: u16, period: u16, color: Light },
    LedBlinkStop,
    LedBlinkColor(Light),

    Delay(u32),

    DisplayBacklight { brightness: u8 },
    DisplayBacklightEnforceOn,
    DisplayBacklightEnforceAuto,

    DoNotReset,

    ForceSpeakerVolumeSetting(f32),
    ForceVibroSetting(bool),
    ForceDisplayBrightnessSetting(f32),

    End,
}

impl NotificationMessage {
    pub fn to_sys(&self) -> sys::NotificationMessage {
        match *self {
            Self::Vibro(on) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeVibro,
                data: sys::NotificationMessageData {
                    vibro: sys::NotificationMessageDataVibro { on },
                },
            },
            Self::SoundOn { frequency, volume } => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeSoundOn,
                data: sys::NotificationMessageData {
                    sound: sys::NotificationMessageDataSound { frequency, volume },
                },
            },
            Self::SoundOff => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeSoundOn,
                data: sys::NotificationMessageData {
                    sound: sys::NotificationMessageDataSound {
                        frequency: 0.0,
                        volume: 0.0,
                    },
                },
            },

            Self::LedRed(value) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedRed,
                data: sys::NotificationMessageData {
                    led: sys::NotificationMessageDataLed { value },
                },
            },
            Self::LedGreen(value) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedGreen,
                data: sys::NotificationMessageData {
                    led: sys::NotificationMessageDataLed { value },
                },
            },
            Self::LedBlue(value) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedBlue,
                data: sys::NotificationMessageData {
                    led: sys::NotificationMessageDataLed { value },
                },
            },

            Self::LedBlinkStart { on_time, period, color } => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedBlinkStart,
                data: sys::NotificationMessageData {
                    led_blink: sys::NotificationMessageDataLedBlink {
                        on_time,
                        period,
                        color: color.to_sys(),
                    },
                },
            },
            Self::LedBlinkStop => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedBlinkStop,
                data: sys::NotificationMessageData {
                    led_blink: sys::NotificationMessageDataLedBlink {
                        on_time: 0,
                        period: 0,
                        color: 0,
                    },
                },
            },
            Self::LedBlinkColor(color) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedBlinkColor,
                data: sys::NotificationMessageData {
                    led_blink: sys::NotificationMessageDataLedBlink {
                        on_time: 0,
                        period: 0,
                        color: color.to_sys(),
                    },
                },
            },

            Self::Delay(length) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeDelay,
                data: sys::NotificationMessageData {
                    delay: sys::NotificationMessageDataDelay { length },
                },
            },

            Self::DisplayBacklight { brightness } => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedDisplayBacklight,
                data: sys::NotificationMessageData {
                    led: sys::NotificationMessageDataLed { value: brightness },
                },
            },
            Self::DisplayBacklightEnforceOn => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedDisplayBacklightEnforceOn,
                data: sys::NotificationMessageData {
                    led: sys::NotificationMessageDataLed { value: 0xff },
                },
            },
            Self::DisplayBacklightEnforceAuto => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeLedDisplayBacklightEnforceAuto,
                data: sys::NotificationMessageData {
                    led: sys::NotificationMessageDataLed { value: 0x00 },
                },
            },

            Self::DoNotReset => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeDoNotReset,
                data: sys::NotificationMessageData {
                    led: sys::NotificationMessageDataLed { value: 0x00 },
                },
            },

            Self::ForceSpeakerVolumeSetting(speaker_volume) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeForceSpeakerVolumeSetting,
                data: sys::NotificationMessageData {
                    forced_settings: sys::NotificationMessageDataForcedSettings {
                        speaker_volume: speaker_volume.clamp(0.0, 1.0),
                        vibro: false,
                        display_brightness: 0.0,
                    },
                },
            },
            Self::ForceVibroSetting(vibro) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeForceVibroSetting,
                data: sys::NotificationMessageData {
                    forced_settings: sys::NotificationMessageDataForcedSettings {
                        speaker_volume: 0.0,
                        vibro,
                        display_brightness: 0.0,
                    },
                },
            },
            Self::ForceDisplayBrightnessSetting(display_brightness) => sys::NotificationMessage {
                type_: sys::NotificationMessageType_NotificationMessageTypeForceDisplayBrightnessSetting,
                data: sys::NotificationMessageData {
                    forced_settings: sys::NotificationMessageDataForcedSettings {
                        speaker_volume: 0.0,
                        vibro: false,
                        display_brightness: display_brightness.clamp(0.0, 1.0),
                    },
                },
            },
            Self::End => panic!("NotificationMessage::End has no sys representation"),
        }
    }
}
