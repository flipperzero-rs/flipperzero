//! Furi HAL RTC bindings.

#[repr(C)]
pub struct FuriHalRtcDateTime {
    /// Hour in 24H format: 0-23
    hour: u8,
    /// Minute: 0-59
    minute: u8,
    /// Second: 0-59
    second: u8,
    /// Current day: 1-31
    day: u8,
    /// Current month: 1-12
    month: u8,
    /// Current year: 2000-2099
    year: u16,
    /// Current weekday: 1-7
    weekday: u8,
}

pub type FuriHalRtcFlag = u32;
pub type FuriHalRtcBootMode = u32;
pub type FuriHalRtcRegister = u32;

pub const FLAG_DEBUG: FuriHalRtcFlag = 1 << 0;
pub const FLAG_FACTORY_RESET: FuriHalRtcFlag = 1 << 1;
pub const FLAG_LOCK: FuriHalRtcFlag = 1 << 2;
pub const FLAG_C2_UPDATE: FuriHalRtcFlag = 1 << 3;

/// Normal boot mode, default value.
pub const BOOT_MODE_NORMAL: FuriHalRtcBootMode = 0;
/// Boot to DFU (MCU bootloader by ST).
pub const BOOT_MODE_DFU: FuriHalRtcBootMode = 1;
/// Boot to Update, pre update.
pub const BOOT_MODE_PRE_UPDATE: FuriHalRtcBootMode = 2;
/// Boot to Update, main.
pub const BOOT_MODE_UPDATE: FuriHalRtcBootMode = 3;
/// Boot to Update, post update.
pub const BOOT_MODE_POST_UPDATE: FuriHalRtcBootMode = 4;

/// RTC structure header.
pub const REGISTER_HEADER: FuriHalRtcRegister = 0;
/// Various system bits.
pub const REGISTER_SYSTEM: FuriHalRtcRegister = 1;
/// Pointer to Version.
pub const REGISTER_VERSION: FuriHalRtcRegister = 2;
/// LFS geometry fingerprint.
pub const REGISTER_LFS_FINGERPRINT: FuriHalRtcRegister = 3;
/// Pointer to last fault message
pub const REGISTER_FAULT_DATA: FuriHalRtcRegister = 4;
/// Failed pins count.
pub const REGISTER_PIN_FAILS: FuriHalRtcRegister = 5;
/// Index of FS directory entry corresponding to FW update to be applied.
pub const REGISTER_UPDATE_FOLDER_FS_INDEX: FuriHalRtcRegister = 6;

extern "C" {
    #[link_name = "furi_hal_rtc_datetime_to_timestamp"]
    pub fn datetime_to_timestamp(datetime: *mut FuriHalRtcDateTime) -> u32;
    #[link_name = "furi_hal_rtc_get_boot_mode"]
    pub fn get_boot_mode() -> FuriHalRtcBootMode;
    #[link_name = "furi_hal_rtc_get_datetime"]
    pub fn get_datetime(datetime: *mut FuriHalRtcDateTime);
    #[link_name = "furi_hal_rtc_get_fault_data"]
    pub fn get_fault_data() -> u32;
    #[link_name = "furi_hal_rtc_get_log_level"]
    pub fn get_log_level() -> u8;
    #[link_name = "furi_hal_rtc_get_pin_fails"]
    pub fn get_pin_fails() -> u32;
    #[link_name = "furi_hal_rtc_get_register"]
    pub fn get_register(reg: FuriHalRtcRegister) -> u32;
    #[link_name = "furi_hal_rtc_is_flag_set"]
    pub fn is_flag_set(flag: FuriHalRtcFlag) -> bool;
    #[link_name = "furi_hal_rtc_reset_flag"]
    pub fn reset_flag(flag: FuriHalRtcFlag);
    #[link_name = "furi_hal_rtc_set_boot_mode"]
    pub fn set_boot_mode(mode: FuriHalRtcBootMode);
    #[link_name = "furi_hal_rtc_set_datetime"]
    pub fn set_datetime(datetime: *mut FuriHalRtcDateTime);
    #[link_name = "furi_hal_rtc_set_fault_data"]
    pub fn set_fault_data(value: u32);
    #[link_name = "furi_hal_rtc_set_flag"]
    pub fn set_flag(flag: FuriHalRtcFlag);
    #[link_name = "furi_hal_rtc_set_log_level"]
    pub fn set_log_level(level: u8);
    #[link_name = "furi_hal_rtc_set_pin_fails"]
    pub fn set_pin_fails(value: u32);
    #[link_name = "furi_hal_rtc_set_register"]
    pub fn set_register(reg: FuriHalRtcRegister, val: u32);
    #[link_name = "furi_hal_rtc_validate_datetime"]
    pub fn validate_datetime(datetime: *mut FuriHalRtcDateTime) -> bool;
}
