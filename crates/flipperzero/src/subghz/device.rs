//! Safe wrapper utilizing the API provided in `lib/subghz/devices/devices.h`

use super::{error::SubGhzError, preset::SubGhzBuiltinPreset};
use core::{ffi::CStr, ptr::null_mut};
use flipperzero_sys::{
    subghz_devices_begin, subghz_devices_end, subghz_devices_flush_rx, subghz_devices_flush_tx,
    subghz_devices_get_by_name, subghz_devices_idle, subghz_devices_is_connect,
    subghz_devices_is_frequency_valid, subghz_devices_is_rx_data_crc_valid,
    subghz_devices_load_preset, subghz_devices_read_packet, subghz_devices_reset,
    subghz_devices_rx_pipe_not_empty, subghz_devices_set_frequency, subghz_devices_set_rx,
    subghz_devices_set_tx, subghz_devices_sleep, subghz_devices_write_packet, SubGhzDevice,
};
use uom::si::{frequency::hertz, u32::Frequency};

#[cfg(feature = "alloc")]
use super::preset::SubGhzPreset;

pub struct SubGhz {
    device: &'static SubGhzDevice,
}

impl SubGhz {
    pub fn subghz_devices_get_by_name(name: &CStr) -> Option<SubGhz> {
        // Safety: input type enforeced by CStr type.
        let dev = unsafe { subghz_devices_get_by_name(name.as_ptr()) };
        // Safety: This pointer should either be null or return a reference, I don't see how it could not.
        // Further, the reference should be static as it relates to the variable:
        // `static SubGhzDeviceRegistry* subghz_device_registry = NULL;` in `lib/subghz/devices/registry.c``
        let dev = unsafe { dev.as_ref()? };
        Some(Self { device: dev })
    }

    // No clue what sort of error may occur, if using the internal device, it will always return Ok()
    pub fn begin(&mut self) -> Result<(), SubGhzError> {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        let result = unsafe { subghz_devices_begin(self.device) };

        // False indicates Ok
        if result {
            Err(SubGhzError::UnableToOpenDevice)
        } else {
            Ok(())
        }
    }

    pub fn is_connect(&mut self) -> bool {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe { subghz_devices_is_connect(self.device) }
    }

    pub fn reset(&mut self) {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe {
            subghz_devices_reset(self.device);
        }
    }

    pub fn sleep(&mut self) {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe {
            subghz_devices_sleep(self.device);
        }
    }

    pub fn idle(&mut self) {
        unsafe {
            subghz_devices_idle(self.device);
        }
    }

    #[cfg(feature = "alloc")]
    pub fn load_preset(&mut self, preset: SubGhzPreset) {
        let furi_preset = preset.into_furi_preset();

        let preset_data = if let SubGhzPreset::Custom(data) = preset {
            data.as_mut_ptr()
        } else {
            null_mut::<u8>()
        };

        // Safety: Type enforcement of the builtin presets, so a valid pointer is passed based on the input Preset.
        unsafe {
            subghz_devices_load_preset(self.device, furi_preset, preset_data);
        }
    }

    pub fn load_builtin_preset(&mut self, preset: SubGhzBuiltinPreset) {
        let furi_preset = preset.into_furi_preset();

        // Safety: Type enforcement of the builtin presets, so it is Ok to pass in the null pointer here.
        unsafe {
            subghz_devices_load_preset(self.device, furi_preset, null_mut::<u8>());
        }
    }

    // If the returned frequency from the internal API function call is 0, this function returns an error.
    pub fn set_frequency(&mut self, freq: Frequency) -> Result<Frequency, SubGhzError> {
        let freq_u32 = freq.value;
        let actual_freq = unsafe { subghz_devices_set_frequency(self.device, freq_u32) };

        if actual_freq != 0 {
            Ok(Frequency::new::<hertz>(actual_freq))
        } else {
            Err(SubGhzError::UnableToSetFrequency)
        }
    }

    pub fn is_frequency_valid(&mut self, freq: Frequency) -> bool {
        unsafe { subghz_devices_is_frequency_valid(self.device, freq.value) }
    }

    pub fn set_async_mirror_pin(&mut self) {
        unimplemented!("Should probably create safe abstraction around GpioPin first, have not investigated if this is needed");
    }

    pub fn get_data_gpio(&mut self) {
        unimplemented!("Should probably create safe abstraction around GpioPin first, have not investigated if this is needed");
    }

    pub fn set_tx(&mut self) -> Result<(), SubGhzError> {
        let result = unsafe { subghz_devices_set_tx(self.device) };

        // Returns true if Ok
        if result {
            Ok(())
        } else {
            Err(SubGhzError::CannotTxOnFrequency)
        }
    }

    pub fn flush_tx(&mut self) {
        unsafe { subghz_devices_flush_tx(self.device) };
    }

    pub fn set_rx(&mut self) {
        unsafe {
            subghz_devices_set_rx(self.device);
        }
    }

    pub fn flush_rx(&mut self) {
        unsafe {
            subghz_devices_flush_rx(self.device);
        }
    }

    pub fn get_rssi(&mut self) -> Option<f32> {
        let rssi_function = unsafe { self.device.interconnect.as_ref() }?.get_rssi?;
        Some(unsafe { rssi_function() })
    }

    pub fn get_lqi(&mut self) -> Option<u8> {
        let lqi_function = unsafe { self.device.interconnect.as_ref()?.get_lqi? };
        Some(unsafe { lqi_function() })
    }

    pub fn rx_pipe_not_empty(&mut self) -> bool {
        unsafe { subghz_devices_rx_pipe_not_empty(self.device) }
    }

    pub fn is_rx_crc_valid(&mut self) -> bool {
        unsafe { subghz_devices_is_rx_data_crc_valid(self.device) }
    }

    pub fn read_packet(&mut self, buf: &mut [u8]) -> usize {
        let mut size = if buf.len() > 255 {
            255_u8
        } else {
            buf.len() as u8
        };

        unsafe {
            subghz_devices_read_packet(self.device, buf.as_mut_ptr(), &mut size);
        };
        size as usize
    }

    pub fn write_packet(&mut self, buf: &[u8]) -> Result<(), SubGhzError> {
        if buf.len() > 255 {
            return Err(SubGhzError::PacketTooLong);
        }

        let size = buf.len() as u8;

        unsafe {
            subghz_devices_write_packet(self.device, buf.as_ptr(), size);
        }
        Ok(())
    }
}

impl Drop for SubGhz {
    fn drop(&mut self) {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe {
            subghz_devices_end(self.device);
        }
    }
}
