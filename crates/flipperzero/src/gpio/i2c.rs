//! I2C interface for the Flipper Zero.

use core::fmt;

use flipperzero_sys as sys;

use crate::furi::time::Duration;

/// The address of the internal LED controller.
///
/// This is an [LP5562].
///
/// [LP5562]: https://www.ti.com/lit/ds/symlink/lp5562.pdf
pub const INTERNAL_LED_CONTROLLER: DeviceAddress = DeviceAddress::new(0x30);

/// The address of the internal battery fuel gauge.
///
/// This is a [BQ27220].
///
/// [BQ27220]: https://www.ti.com/lit/ds/symlink/bq27220.pdf
pub const INTERNAL_BATTERY_FUEL_GAUGE: DeviceAddress = DeviceAddress::new(0x55);

/// The address of the internal battery charger.
///
/// This is a [BQ25896].
///
/// [BQ25896]: https://www.ti.com/lit/ds/symlink/bq25896.pdf
pub const INTERNAL_BATTERY_CHARGER: DeviceAddress = DeviceAddress::new(0x6B);

/// A 7-bit I2C device address.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DeviceAddress(u8);

impl fmt::Debug for DeviceAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DeviceAddress")
            .field(&(self.0 >> 1))
            .finish()
    }
}

impl ufmt::uDebug for DeviceAddress {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.debug_tuple("DeviceAddress")?
            .field(&(self.0 >> 1))?
            .finish()
    }
}

impl DeviceAddress {
    /// Constructs a `DeviceAddress` from its 7-bit value.
    ///
    /// The upper bit of `addr` is ignored; `0x00` and `0x80` are treated as the same
    /// address.
    pub const fn new(addr: u8) -> Self {
        // The SDK takes addresses in 8-bit form, so we store them that way to reduce the
        // number of places we need to handle the shift.
        Self(addr << 1)
    }
}

/// A handle to an I2C bus.
#[derive(Clone, Copy)]
enum BusKind {
    Internal,
    External,
}

/// An I2C bus on the Flipper Zero.
#[derive(Clone, Copy)]
pub struct Bus(BusKind);

impl Bus {
    /// The internal (power) I2C bus.
    ///
    /// This has three devices:
    /// - [`INTERNAL_LED_CONTROLLER`]
    /// - [`INTERNAL_BATTERY_FUEL_GAUGE`]
    /// - [`INTERNAL_BATTERY_CHARGER`]
    pub const INTERNAL: Self = Self(BusKind::Internal);

    /// The external I2C bus.
    ///
    /// - Connect `SCL` to pin `C0`.
    /// - Connect `SDA` to pin `C1`.
    ///
    /// # Warning
    ///
    /// Only connect 3.3V peripherals directly to your Flipper Zero, or you risk damaging
    /// it. For I2C devices that operate at different voltages, use a level shifter.
    pub const EXTERNAL: Self = Self(BusKind::External);

    /// Acquires a handle to the given I2C bus.
    ///
    /// Blocks indefinitely until the bus is available.
    pub fn acquire(self) -> BusHandle {
        BusHandle::acquire(match self.0 {
            BusKind::Internal => unsafe { &mut sys::furi_hal_i2c_handle_power },
            BusKind::External => unsafe { &mut sys::furi_hal_i2c_handle_external },
        })
    }

    /// Acquires a handle to the given I2C bus and then runs the given function.
    ///
    /// Blocks indefinitely until the bus is available.
    pub fn with_handle<T>(self, f: impl FnOnce(BusHandle) -> T) -> T {
        f(self.acquire())
    }
}

/// A handle to an I2C bus on the Flipper Zero.
pub struct BusHandle {
    handle: &'static mut sys::FuriHalI2cBusHandle,
}

impl Drop for BusHandle {
    fn drop(&mut self) {
        unsafe { sys::furi_hal_i2c_release(self.handle) };
    }
}

impl BusHandle {
    /// Acquires a handle to the given I2C bus.
    ///
    /// Blocks indefinitely until the Flipper Zero bus is locally available.
    fn acquire(handle: &'static mut sys::FuriHalI2cBusHandle) -> Self {
        unsafe { sys::furi_hal_i2c_acquire(handle) };
        Self { handle }
    }

    /// Enumerates the devices that are present and ready on this bus.
    ///
    /// `per_device_timeout` is in milliseconds.
    pub fn enumerate_devices(
        &mut self,
        per_device_timeout: Duration,
    ) -> impl Iterator<Item = DeviceAddress> + '_ {
        (0x00..0x80).filter_map(move |addr| {
            let device = DeviceAddress::new(addr);
            self.is_device_ready(device, per_device_timeout)
                .then_some(device)
        })
    }

    /// Checks if the device with address `i2c_addr` is present and ready on the bus.
    ///
    /// `timeout` is in milliseconds.
    ///
    /// Returns `true` if the device is present and ready, false otherwise.
    pub fn is_device_ready(&mut self, device: DeviceAddress, timeout: Duration) -> bool {
        unsafe {
            sys::furi_hal_i2c_is_device_ready(self.handle, device.0, timeout.as_millis() as u32)
        }
    }

    /// Reads the 8-bit register at `reg_addr` on `device`.
    ///
    /// `timeout` is in milliseconds.
    pub fn read_u8(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        timeout: Duration,
    ) -> Result<u8, Error> {
        let mut data = 0;
        if unsafe {
            sys::furi_hal_i2c_read_reg_8(
                self.handle,
                device.0,
                reg_addr,
                &mut data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(data)
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Reads the 16-bit register at `reg_addr` on `device`.
    ///
    /// `timeout` is in milliseconds.
    pub fn read_u16(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        timeout: Duration,
    ) -> Result<u16, Error> {
        let mut data = 0;
        if unsafe {
            sys::furi_hal_i2c_read_reg_16(
                self.handle,
                device.0,
                reg_addr,
                &mut data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(data)
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Reads `device`'s memory starting at `mem_addr` into the given buffer.
    ///
    /// `timeout` is in milliseconds.
    pub fn read_exact(
        &mut self,
        device: DeviceAddress,
        mem_addr: u8,
        buf: &mut [u8],
        timeout: Duration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_read_mem(
                self.handle,
                device.0,
                mem_addr,
                buf.as_mut_ptr(),
                buf.len(),
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Writes the given value into the 8-bit register at `reg_addr` on `device`.
    ///
    /// `timeout` is in milliseconds.
    pub fn write_u8(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        data: u8,
        timeout: Duration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_write_reg_8(
                self.handle,
                device.0,
                reg_addr,
                data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Writes the given value into the 16-bit register at `reg_addr` on `device`.
    ///
    /// `timeout` is in milliseconds.
    pub fn write_u16(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        data: u16,
        timeout: Duration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_write_reg_16(
                self.handle,
                device.0,
                reg_addr,
                data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Writes the given data into `device`'s memory starting at `mem_addr`.
    ///
    /// `timeout` is in milliseconds.
    pub fn write_all(
        &mut self,
        device: DeviceAddress,
        mem_addr: u8,
        data: &[u8],
        timeout: Duration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_write_mem(
                self.handle,
                device.0,
                mem_addr,
                data.as_ptr(),
                data.len(),
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TransferFailed,
}

#[flipperzero_test::tests]
mod tests {
    use super::{
        Bus, DeviceAddress, INTERNAL_BATTERY_CHARGER, INTERNAL_BATTERY_FUEL_GAUGE,
        INTERNAL_LED_CONTROLLER,
    };
    use crate::furi::time::Duration;

    #[test]
    fn enumerate_devices() {
        const INTERNAL_DEVICES: &[DeviceAddress] = &[
            INTERNAL_LED_CONTROLLER,
            INTERNAL_BATTERY_FUEL_GAUGE,
            INTERNAL_BATTERY_CHARGER,
        ];

        let mut bus = Bus::INTERNAL.acquire();
        for (i, device) in bus.enumerate_devices(Duration::from_millis(50)).enumerate() {
            if let Some(&expected) = INTERNAL_DEVICES.get(i) {
                assert_eq!(expected, device);
            }
        }
    }
}
