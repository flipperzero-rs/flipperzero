use super::{messages, sounds, NotificationSequence};
use crate::notification_sequence;

pub const RESET_RED: NotificationSequence = notification_sequence![messages::RED_0];
pub const RESET_GREEN: NotificationSequence = notification_sequence![messages::GREEN_0];
pub const RESET_BLUE: NotificationSequence = notification_sequence![messages::BLUE_0];
pub const RESET_RGB: NotificationSequence =
    notification_sequence![messages::RED_0, messages::GREEN_0, messages::BLUE_0];
pub const RESET_DISPLAY: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_OFF];
pub const RESET_SOUND: NotificationSequence = notification_sequence![messages::SOUND_OFF];
pub const RESET_VIBRO: NotificationSequence = notification_sequence![messages::VIBRO_OFF];

pub const VIBRO_ON: NotificationSequence = notification_sequence![messages::VIBRO_ON];

pub const DISPLAY_BACKLIGHT_ON: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_ON];
pub const DISPLAY_BACKLIGHT_OFF: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_OFF];

pub const DISPLAY_BACKLIGHT_ENFORCE_ON: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_ENFORCE_ON];
pub const DISPLAY_BACKLIGHT_ENFORCE_AUTO: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_ENFORCE_AUTO];

pub const DISPLAY_BACKLIGHT_OFF_DELAY_1000: NotificationSequence =
    notification_sequence![messages::DELAY_1000, messages::DISPLAY_BACKLIGHT_OFF];

pub const CHARGING: NotificationSequence =
    notification_sequence![messages::RED_255, messages::GREEN_0];
pub const CHARGED: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::RED_0];
pub const NOT_CHARGING: NotificationSequence =
    notification_sequence![messages::RED_0, messages::GREEN_0];

pub const ONLY_RED: NotificationSequence = notification_sequence![
    messages::RED_255,
    messages::GREEN_0,
    messages::BLUE_0,
    messages::DO_NOT_RESET,
];
pub const ONLY_GREEN: NotificationSequence = notification_sequence![
    messages::RED_0,
    messages::GREEN_255,
    messages::BLUE_0,
    messages::DO_NOT_RESET,
];
pub const ONLY_BLUE: NotificationSequence = notification_sequence![
    messages::RED_0,
    messages::GREEN_0,
    messages::BLUE_255,
    messages::DO_NOT_RESET,
];

pub const SET_RED: NotificationSequence =
    notification_sequence![messages::RED_255, messages::DO_NOT_RESET];
pub const SET_GREEN: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::DO_NOT_RESET];
pub const SET_BLUE: NotificationSequence =
    notification_sequence![messages::BLUE_255, messages::DO_NOT_RESET];

pub const SOLID_YELLOW: NotificationSequence = notification_sequence![
    messages::RED_255,
    messages::GREEN_255,
    messages::BLUE_0,
    messages::DO_NOT_RESET,
];

pub const BLINK_RED_10: NotificationSequence =
    notification_sequence![messages::RED_255, messages::DELAY_10];
pub const BLINK_GREEN_10: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::DELAY_10];
pub const BLINK_BLUE_10: NotificationSequence =
    notification_sequence![messages::BLUE_255, messages::DELAY_10];
pub const BLINK_YELLOW_10: NotificationSequence =
    notification_sequence![messages::RED_255, messages::GREEN_255, messages::DELAY_10,];
pub const BLINK_CYAN_10: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::BLUE_255, messages::DELAY_10,];
pub const BLINK_MAGNENTA_10: NotificationSequence =
    notification_sequence![messages::RED_255, messages::BLUE_255, messages::DELAY_10];

pub const BLINK_RED_100: NotificationSequence =
    notification_sequence![messages::RED_255, messages::DELAY_100];
pub const BLINK_GREEN_100: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::DELAY_100];
pub const BLINK_BLUE_100: NotificationSequence =
    notification_sequence![messages::BLUE_255, messages::DELAY_100];
pub const BLINK_YELLOW_100: NotificationSequence =
    notification_sequence![messages::RED_255, messages::GREEN_255, messages::DELAY_100,];
pub const BLINK_CYAN_100: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::BLUE_255, messages::DELAY_100,];
pub const BLINK_MAGNENTA_100: NotificationSequence =
    notification_sequence![messages::RED_255, messages::BLUE_255, messages::DELAY_100,];

pub const BLINK_START_RED: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_RED,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_GREEN: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_GREEN,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_BLUE: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_BLUE,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_YELLOW: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_YELLOW,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_CYAN: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_CYAN,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_MAGENTA: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_MAGENTA,
    messages::DO_NOT_RESET,
];
pub const BLINK_STOP: NotificationSequence = notification_sequence![messages::BLINK_STOP];

pub const SINGLE_VIBRO: NotificationSequence =
    notification_sequence![messages::VIBRO_ON, messages::DELAY_100, messages::VIBRO_OFF,];

pub const DOUBLE_VIBRO: NotificationSequence = notification_sequence![
    messages::VIBRO_ON,
    messages::DELAY_100,
    messages::VIBRO_OFF,
    messages::DELAY_100,
    messages::VIBRO_ON,
    messages::DELAY_100,
    messages::VIBRO_OFF,
];

pub const SUCCESS: NotificationSequence = notification_sequence![
    messages::DISPLAY_BACKLIGHT_ON,
    messages::GREEN_255,
    messages::VIBRO_ON,
    sounds::C5,
    messages::DELAY_50,
    messages::VIBRO_OFF,
    sounds::E5,
    messages::DELAY_50,
    sounds::G5,
    messages::DELAY_50,
    sounds::C6,
    messages::DELAY_50,
    messages::SOUND_OFF,
];
