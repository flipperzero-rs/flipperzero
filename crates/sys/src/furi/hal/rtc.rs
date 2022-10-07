//! Furi HAL RTC bindings.

use core::ops;

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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuriHalRtcFlag(u32);

impl FuriHalRtcFlag {
    pub const DEBUG: FuriHalRtcFlag = Self(1 << 0);
    pub const FACTORY_RESET: FuriHalRtcFlag = Self(1 << 1);
    pub const LOCK: FuriHalRtcFlag = Self(1 << 2);
    pub const C2_UPDATE: FuriHalRtcFlag = Self(1 << 3);
}

impl ops::BitOr for FuriHalRtcFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::BitOrAssign for FuriHalRtcFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuriHalRtcBootMode(u32);

impl FuriHalRtcBootMode {
    /// Normal boot mode, default value.
    pub const NORMAL: FuriHalRtcBootMode = Self(0);
    /// Boot to DFU (MCU bootloader by ST).
    pub const DFU: FuriHalRtcBootMode = Self(1);
    /// Boot to Update, pre update.
    pub const PRE_UPDATE: FuriHalRtcBootMode = Self(2);
    /// Boot to Update, main.
    pub const UPDATE: FuriHalRtcBootMode = Self(3);
    /// Boot to Update, post update.
    pub const POST_UPDATE: FuriHalRtcBootMode = Self(4);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuriHalRtcRegister(u32);

impl FuriHalRtcRegister {
    /// RTC structure header.
    pub const HEADER: FuriHalRtcRegister = Self(0);
    /// Various system bits.
    pub const SYSTEM: FuriHalRtcRegister = Self(1);
    /// Pointer to Version.
    pub const VERSION: FuriHalRtcRegister = Self(2);
    /// LFS geometry fingerprint.
    pub const LFS_FINGERPRINT: FuriHalRtcRegister = Self(3);
    /// Pointer to last fault message
    pub const FAULT_DATA: FuriHalRtcRegister = Self(4);
    /// Failed pins count.
    pub const PIN_FAILS: FuriHalRtcRegister = Self(5);
    /// Index of FS directory entry corresponding to FW update to be applied.
    pub const UPDATE_FOLDER_FS_INDEX: FuriHalRtcRegister = Self(6);
}

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
