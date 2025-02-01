//! Furi Power service.

use core::ffi::CStr;
use core::mem;

use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

pub type PowerBootMode = sys::PowerBootMode;
pub type PowerInfo = sys::PowerInfo;

/// Handle to the Power service.
#[derive(Clone)]
pub struct Power {
    record: UnsafeRecord<sys::Power>,
}

impl Power {
    pub const NAME: &CStr = c"power";

    /// Open handle to Power service.
    pub fn open() -> Self {
        Self {
            record: unsafe { UnsafeRecord::open(Self::NAME) },
        }
    }

    /// Get handle to raw [`sys::Power`] record.
    ///
    /// This pointer must not be `free`d or otherwise invalidated.
    /// It must not be referenced after [`Power`] has been dropped.
    #[inline]
    pub fn as_ptr(&self) -> *mut sys::Power {
        self.record.as_ptr()
    }

    /// Power off device.
    pub fn power_off(&self) {
        unsafe { sys::power_off(self.as_ptr()) }
    }

    /// Reboot device.
    pub fn reboot(&self, mode: PowerBootMode) {
        unsafe { sys::power_reboot(self.as_ptr(), mode) }
    }

    /// Get power info.
    pub fn get_info(&self) -> PowerInfo {
        unsafe {
            let mut power_info = mem::zeroed();
            sys::power_get_info(self.as_ptr(), &raw mut power_info);

            power_info
        }
    }

    /// Check battery health.
    pub fn is_battery_healthy(&self) -> bool {
        unsafe { sys::power_is_battery_healthy(self.as_ptr()) }
    }

    /// Enable or disable battery low level notification message.
    pub fn enable_low_battery_level_notification(&self, enable: bool) {
        unsafe {
            sys::power_enable_low_battery_level_notification(self.as_ptr(), enable);
        }
    }
}
