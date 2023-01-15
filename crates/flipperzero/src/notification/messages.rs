use super::NotificationMessage;
use flipperzero_sys as sys;

pub static DISPLAY_BACKLIGHT_ON: NotificationMessage =
    NotificationMessage(unsafe { sys::message_display_backlight_on });
pub static DISPLAY_BACKLIGHT_OFF: NotificationMessage =
    unsafe { NotificationMessage(sys::message_display_backlight_off) };
pub static DISPLAY_BACKLIGHT_ENFORCE_ON: NotificationMessage =
    unsafe { NotificationMessage(sys::message_display_backlight_enforce_on) };
pub static DISPLAY_BACKLIGHT_ENFORCE_AUTO: NotificationMessage =
    unsafe { NotificationMessage(sys::message_display_backlight_enforce_auto) };

pub static RED_255: NotificationMessage = unsafe { NotificationMessage(sys::message_red_255) };
pub static GREEN_255: NotificationMessage = unsafe { NotificationMessage(sys::message_green_255) };
pub static BLUE_255: NotificationMessage = unsafe { NotificationMessage(sys::message_blue_255) };
pub static RED_0: NotificationMessage = unsafe { NotificationMessage(sys::message_red_0) };
pub static GREEN_0: NotificationMessage = unsafe { NotificationMessage(sys::message_green_0) };
pub static BLUE_0: NotificationMessage = unsafe { NotificationMessage(sys::message_blue_0) };

pub static BLINK_START_10: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_start_10) };
pub static BLINK_START_100: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_start_100) };
pub static BLINK_STOP: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_stop) };

pub static BLINK_SET_COLOR_RED: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_set_color_red) };
pub static BLINK_SET_COLOR_GREEN: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_set_color_green) };
pub static BLINK_SET_COLOR_BLUE: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_set_color_blue) };
pub static BLINK_SET_COLOR_CYAN: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_set_color_cyan) };
pub static BLINK_SET_COLOR_MAGENTA: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_set_color_magenta) };
pub static BLINK_SET_COLOR_YELLOW: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_set_color_yellow) };
pub static BLINK_SET_COLOR_WHITE: NotificationMessage =
    unsafe { NotificationMessage(sys::message_blink_set_color_white) };

pub static DELAY_1: NotificationMessage = unsafe { NotificationMessage(sys::message_delay_1) };
pub static DELAY_10: NotificationMessage = unsafe { NotificationMessage(sys::message_delay_10) };
pub static DELAY_25: NotificationMessage = unsafe { NotificationMessage(sys::message_delay_25) };
pub static DELAY_50: NotificationMessage = unsafe { NotificationMessage(sys::message_delay_50) };
pub static DELAY_100: NotificationMessage = unsafe { NotificationMessage(sys::message_delay_100) };
pub static DELAY_250: NotificationMessage = unsafe { NotificationMessage(sys::message_delay_250) };
pub static DELAY_500: NotificationMessage = unsafe { NotificationMessage(sys::message_delay_500) };
pub static DELAY_1000: NotificationMessage =
    unsafe { NotificationMessage(sys::message_delay_1000) };

pub static SOUND_OFF: NotificationMessage = unsafe { NotificationMessage(sys::message_sound_off) };

pub static VIBRO_ON: NotificationMessage = unsafe { NotificationMessage(sys::message_vibro_on) };
pub static VIBRO_OFF: NotificationMessage = unsafe { NotificationMessage(sys::message_vibro_off) };

pub static DO_NOT_RESET: NotificationMessage =
    unsafe { NotificationMessage(sys::message_do_not_reset) };

pub static FORCE_SPEAKER_VOLUME_SETTING_1: NotificationMessage =
    unsafe { NotificationMessage(sys::message_force_speaker_volume_setting_1f) };
pub static FORCE_VIBRO_SETTING_ON: NotificationMessage =
    unsafe { NotificationMessage(sys::message_force_vibro_setting_on) };
pub static FORCE_VIBRO_SETTING_OFF: NotificationMessage =
    unsafe { NotificationMessage(sys::message_force_vibro_setting_off) };
pub static FORCE_DISPLAY_BRIGHTNESS_SETTING_1: NotificationMessage =
    unsafe { NotificationMessage(sys::message_force_display_brightness_setting_1f) };
