use super::{Light, NotificationMessage};

pub const DISPLAY_BACKLIGHT_ON: NotificationMessage = NotificationMessage::DisplayBacklight { brightness: 0xff };
pub const DISPLAY_BACKLIGHT_OFF: NotificationMessage = NotificationMessage::DisplayBacklight { brightness: 0x00 };
pub const RED_255: NotificationMessage = NotificationMessage::LedRed(255);
pub const GREEN_255: NotificationMessage = NotificationMessage::LedGreen(255);
pub const BLUE_255: NotificationMessage = NotificationMessage::LedBlue(255);
pub const RED_0: NotificationMessage = NotificationMessage::LedRed(0);
pub const GREEN_0: NotificationMessage = NotificationMessage::LedGreen(0);
pub const BLUE_0: NotificationMessage = NotificationMessage::LedBlue(0);
pub const BLINK_START_10: NotificationMessage = NotificationMessage::LedBlinkStart {
    on_time: 10,
    period: 100,
    color: Light::Off,
};
pub const BLINK_START_100: NotificationMessage = NotificationMessage::LedBlinkStart {
    on_time: 100,
    period: 1000,
    color: Light::Off,
};
pub const BLINK_SET_COLOR_RED: NotificationMessage = NotificationMessage::LedBlinkColor(Light::Red);
pub const BLINK_SET_COLOR_GREEN: NotificationMessage = NotificationMessage::LedBlinkColor(Light::Green);
pub const BLINK_SET_COLOR_BLUE: NotificationMessage = NotificationMessage::LedBlinkColor(Light::Blue);
pub const BLINK_SET_COLOR_CYAN: NotificationMessage = NotificationMessage::LedBlinkColor(Light::Cyan);
pub const BLINK_SET_COLOR_MAGENTA: NotificationMessage = NotificationMessage::LedBlinkColor(Light::Magenta);
pub const BLINK_SET_COLOR_YELLOW: NotificationMessage = NotificationMessage::LedBlinkColor(Light::Yellow);
pub const BLINK_SET_COLOR_WHITE: NotificationMessage = NotificationMessage::LedBlinkColor(Light::White);

pub const DELAY_1: NotificationMessage = NotificationMessage::Delay(1);
pub const DELAY_10: NotificationMessage = NotificationMessage::Delay(10);
pub const DELAY_25: NotificationMessage = NotificationMessage::Delay(25);
pub const DELAY_50: NotificationMessage = NotificationMessage::Delay(50);
pub const DELAY_100: NotificationMessage = NotificationMessage::Delay(100);
pub const DELAY_250: NotificationMessage = NotificationMessage::Delay(250);
pub const DELAY_500: NotificationMessage = NotificationMessage::Delay(500);
pub const DELAY_1000: NotificationMessage = NotificationMessage::Delay(1000);

pub const SOUND_OFF: NotificationMessage = NotificationMessage::SoundOff;

pub const VIBRO_ON: NotificationMessage = NotificationMessage::Vibro(true);
pub const VIBRO_OFF: NotificationMessage = NotificationMessage::Vibro(false);

pub const FORCE_SPEAKER_VOLUME_SETTING_1: NotificationMessage = NotificationMessage::ForceSpeakerVolumeSetting(1.0);
pub const FORCE_VIBRO_SETTING_ON: NotificationMessage = NotificationMessage::ForceVibroSetting(true);
pub const FORCE_VIBRO_SETTING_OFF: NotificationMessage = NotificationMessage::ForceVibroSetting(false);
pub const FORCE_DISPLAY_BRIGHTNESS_SETTING_1: NotificationMessage =
    NotificationMessage::ForceDisplayBrightnessSetting(1.0);

//Don't force users to import NotificationMessage just for this if they only use default messages

pub const LED_BLINK_STOP: NotificationMessage = NotificationMessage::LedBlinkStop;
pub const DISPLAY_BACKLIGHT_ENFORCE_ON: NotificationMessage = NotificationMessage::DisplayBacklightEnforceOn;
pub const DISPLAY_BACKLIGHT_ENFORCE_AUTO: NotificationMessage = NotificationMessage::DisplayBacklightEnforceAuto;
pub const DO_NOT_RESET: NotificationMessage = NotificationMessage::DoNotReset;
pub const END: NotificationMessage = NotificationMessage::End;
