use super::{Light, NotificationMessage};

pub const DISPLAY_BACKLIGHT_ON: NotificationMessage = NotificationMessage::display_backlight(0xFF);
pub const DISPLAY_BACKLIGHT_OFF: NotificationMessage = NotificationMessage::display_backlight(0x00);
pub const DISPLAY_BACKLIGHT_ENFORCE_ON: NotificationMessage =
    NotificationMessage::display_backlight_enforce_on();
pub const DISPLAY_BACKLIGHT_ENFORCE_AUTO: NotificationMessage =
    NotificationMessage::display_backlight_enforce_auto();

pub const RED_255: NotificationMessage = NotificationMessage::led_red(255);
pub const GREEN_255: NotificationMessage = NotificationMessage::led_green(255);
pub const BLUE_255: NotificationMessage = NotificationMessage::led_blue(255);
pub const RED_0: NotificationMessage = NotificationMessage::led_red(0);
pub const GREEN_0: NotificationMessage = NotificationMessage::led_green(0);
pub const BLUE_0: NotificationMessage = NotificationMessage::led_blue(0);

pub const BLINK_START_10: NotificationMessage =
    NotificationMessage::led_blink_start(10, 100, Light::OFF);
pub const BLINK_START_100: NotificationMessage =
    NotificationMessage::led_blink_start(100, 1000, Light::OFF);
pub const BLINK_STOP: NotificationMessage = NotificationMessage::led_blink_stop();

pub const BLINK_SET_COLOR_RED: NotificationMessage =
    NotificationMessage::led_blink_color(Light::RED);
pub const BLINK_SET_COLOR_GREEN: NotificationMessage =
    NotificationMessage::led_blink_color(Light::GREEN);
pub const BLINK_SET_COLOR_BLUE: NotificationMessage =
    NotificationMessage::led_blink_color(Light::BLUE);
pub const BLINK_SET_COLOR_CYAN: NotificationMessage =
    NotificationMessage::led_blink_color(Light::CYAN);
pub const BLINK_SET_COLOR_MAGENTA: NotificationMessage =
    NotificationMessage::led_blink_color(Light::MAGENTA);
pub const BLINK_SET_COLOR_YELLOW: NotificationMessage =
    NotificationMessage::led_blink_color(Light::YELLOW);
pub const BLINK_SET_COLOR_WHITE: NotificationMessage =
    NotificationMessage::led_blink_color(Light::WHITE);

pub const DELAY_1: NotificationMessage = NotificationMessage::delay(1);
pub const DELAY_10: NotificationMessage = NotificationMessage::delay(10);
pub const DELAY_25: NotificationMessage = NotificationMessage::delay(25);
pub const DELAY_50: NotificationMessage = NotificationMessage::delay(50);
pub const DELAY_100: NotificationMessage = NotificationMessage::delay(100);
pub const DELAY_250: NotificationMessage = NotificationMessage::delay(250);
pub const DELAY_500: NotificationMessage = NotificationMessage::delay(500);
pub const DELAY_1000: NotificationMessage = NotificationMessage::delay(1000);

pub const SOUND_OFF: NotificationMessage = NotificationMessage::sound_off();

pub const VIBRO_ON: NotificationMessage = NotificationMessage::vibro(true);
pub const VIBRO_OFF: NotificationMessage = NotificationMessage::vibro(false);

pub const DO_NOT_RESET: NotificationMessage = NotificationMessage::do_not_reset();

pub const FORCE_SPEAKER_VOLUME_SETTING_1: NotificationMessage =
    NotificationMessage::force_speaker_volume_setting(1.0);
pub const FORCE_VIBRO_SETTING_ON: NotificationMessage =
    NotificationMessage::force_vibro_setting(true);
pub const FORCE_VIBRO_SETTING_OFF: NotificationMessage =
    NotificationMessage::force_vibro_setting(false);
pub const FORCE_DISPLAY_BRIGHTNESS_SETTING_1: NotificationMessage =
    NotificationMessage::force_display_brightness_setting(1.0);
