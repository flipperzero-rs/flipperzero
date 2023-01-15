//! Furi notifications.

use core::ffi::c_char;

use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

///Default notification messages.
//TODO pub mod messages;
///Default notification notes.
//TODO pub mod notes;
///Default notification sequences.
//TODO pub mod sequences;

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
    pub fn notify(&mut self, sequence: NotificationSequence) {
        unsafe { sys::notification_message(self.data.as_ptr(), sequence.to_sys()) };
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub const fn to_sys(self) -> sys::Light {
        self as sys::Light
    }
}

/// A notification message.
#[repr(transparent)]
pub struct NotificationMessage(pub(super) sys::NotificationMessage);

impl NotificationMessage {
    pub const fn vibro(on: bool) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeVibro,
            data: sys::NotificationMessageData {
                vibro: sys::NotificationMessageDataVibro { on },
            },
        })
    }
    pub const fn sound_on(frequency: f32, volume: f32) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeSoundOn,
            data: sys::NotificationMessageData {
                sound: sys::NotificationMessageDataSound { frequency, volume },
            },
        })
    }

    pub const fn sound_off() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeSoundOn,
            data: sys::NotificationMessageData {
                sound: sys::NotificationMessageDataSound {
                    frequency: 0.0,
                    volume: 0.0,
                },
            },
        })
    }

    pub const fn led_red(value: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedRed,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value },
            },
        })
    }

    pub const fn led_green(value: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedGreen,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value },
            },
        })
    }

    pub const fn led_blue(value: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedBlue,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value },
            },
        })
    }

    pub const fn led_blink_start(on_time: u16, period: u16, color: Light) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedBlinkStart,
            data: sys::NotificationMessageData {
                led_blink: sys::NotificationMessageDataLedBlink {
                    on_time,
                    period,
                    color: color.to_sys(),
                },
            },
        })
    }

    pub const fn led_blink_stop() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedBlinkStop,
            data: sys::NotificationMessageData {
                led_blink: sys::NotificationMessageDataLedBlink {
                    on_time: 0,
                    period: 0,
                    color: 0,
                },
            },
        })
    }

    pub const fn led_blink_color(color: Light) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedBlinkColor,
            data: sys::NotificationMessageData {
                led_blink: sys::NotificationMessageDataLedBlink {
                    on_time: 0,
                    period: 0,
                    color: color.to_sys(),
                },
            },
        })
    }

    pub const fn delay(length: u32) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeDelay,
            data: sys::NotificationMessageData {
                delay: sys::NotificationMessageDataDelay { length },
            },
        })
    }

    pub const fn display_backlight(brightness: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedDisplayBacklight,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: brightness },
            },
        })
    }

    pub const fn display_backlight_enforce_on() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeLedDisplayBacklightEnforceOn,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: 0xff },
            },
        })
    }

    pub const fn display_backlight_enforce_auto() -> Self {
        Self(sys::NotificationMessage {
            type_:
                sys::NotificationMessageType_NotificationMessageTypeLedDisplayBacklightEnforceAuto,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: 0x00 },
            },
        })
    }

    pub const fn do_not_reset() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeDoNotReset,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: 0x00 },
            },
        })
    }

    pub const fn force_speaker_volume_setting(speaker_volume: f32) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeForceSpeakerVolumeSetting,
            data: sys::NotificationMessageData {
                forced_settings: sys::NotificationMessageDataForcedSettings {
                    //cant be clamped const due to restrictions on floats in const contexts
                    speaker_volume,
                    vibro: false,
                    display_brightness: 0.0,
                },
            },
        })
    }

    pub const fn force_vibro_setting(vibro: bool) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageType_NotificationMessageTypeForceVibroSetting,
            data: sys::NotificationMessageData {
                forced_settings: sys::NotificationMessageDataForcedSettings {
                    speaker_volume: 0.0,
                    vibro,
                    display_brightness: 0.0,
                },
            },
        })
    }

    pub const fn force_display_bightness_setting(display_brightness: f32) -> Self {
        Self(sys::NotificationMessage {
            type_:
                sys::NotificationMessageType_NotificationMessageTypeForceDisplayBrightnessSetting,
            data: sys::NotificationMessageData {
                forced_settings: sys::NotificationMessageDataForcedSettings {
                    speaker_volume: 0.0,
                    vibro: false,
                    //cant be clamped const due to restrictions on floats in const contexts
                    display_brightness,
                },
            },
        })
    }
}

pub struct NotificationSequence(&'static [*const NotificationMessage]);

impl NotificationSequence {
    #[doc(hidden)]
    pub const fn constsruct(sequence: &'static [*const NotificationMessage]) -> Self {
        Self(sequence)
    }

    pub fn to_sys(&self) -> *const sys::NotificationSequence {
        self.0.as_ptr() as *const _
    }
}

#[macro_export]
macro_rules! notification_sequence {
    ($($x:expr),+ $(,)?) => {
        {
            const S: &[*const $crate::notification::NotificationMessage] = &[
                $(&$x as *const _),*,
                ::core::ptr::null()
            ];
            $crate::notification::NotificationSequence::constsruct(S)
        }
    };
}
