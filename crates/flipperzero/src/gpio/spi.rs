//! SPI interface for the Flipper Zero.

use flipperzero_sys as sys;

use crate::furi::time::Duration;

/// The external SPI bus.
///
/// - Connect `SCK` to pin `B3`.
/// - Connect `CS` to pin `A4` (software controlled).
/// - Connect `MISO` to pin `A6`.
/// - Connect `MOSI` to pin `A7`.
///
/// Preset: `furi_hal_spi_preset_1edge_low_2m`
///
/// Bus pins are floating on inactive state, and `CS` is high after acquiring the bus.
///
/// # Warning
///
/// Only connect 3.3V peripherals directly to your Flipper Zero, or you risk damaging
/// it. For SPI devices that operate at different voltages, use a level shifter.
pub const EXTERNAL: Sub = Sub(SubDevice::External);

/// An SPI sub device inside the Flipper Zero.
#[derive(Clone, Copy)]
pub enum Internal {
    /// The SubGhz chip.
    ///
    /// This is an [CC1101].
    ///
    /// [CC1101]: https://www.ti.com/lit/ds/symlink/cc1101.pdf
    SubGhz,

    /// The NFC chip.
    ///
    /// This is an [ST25R3916].
    ///
    /// [ST25R3916]: https://www.st.com/resource/en/datasheet/st25r3916.pdf
    Nfc,

    /// The display.
    ///
    /// This is an [ST7567].
    ///
    /// [ST7567]: https://www.crystalfontz.com/controllers/Sitronix/ST7567/303/
    Display,

    /// The SD card in fast mode.
    SdFast,

    /// The SD card in slow mode.
    SdSlow,
}

/// An SPI sub device connected to (or inside) the Flipper Zero.
#[derive(Clone, Copy)]
enum SubDevice {
    External,
    Internal(Internal),
}

/// An SPI sub device connected to (or inside) the Flipper Zero.
#[derive(Clone, Copy)]
pub struct Sub(SubDevice);

impl From<Internal> for Sub {
    fn from(sub: Internal) -> Self {
        Sub(SubDevice::Internal(sub))
    }
}

impl Sub {
    /// Acquires a handle to the SPI bus that this sub device is connected to.
    ///
    /// Blocks indefinitely until the bus is available.
    pub fn acquire(self) -> BusHandle {
        match self.0 {
            SubDevice::External => {
                BusHandle::acquire(unsafe { &mut sys::furi_hal_spi_bus_handle_external }, true)
            }
            SubDevice::Internal(sub) => BusHandle::acquire(
                match sub {
                    Internal::SubGhz => unsafe { &mut sys::furi_hal_spi_bus_handle_subghz },
                    Internal::Nfc => unsafe { &mut sys::furi_hal_spi_bus_handle_nfc },
                    Internal::Display => unsafe { &mut sys::furi_hal_spi_bus_handle_display },
                    Internal::SdFast => unsafe { &mut sys::furi_hal_spi_bus_handle_sd_fast },
                    Internal::SdSlow => unsafe { &mut sys::furi_hal_spi_bus_handle_sd_slow },
                },
                false,
            ),
        }
    }

    /// Acquires a handle to the the SPI bus that this sub device is connected to, and
    /// then runs the given function.
    ///
    /// Blocks indefinitely until the bus is available.
    pub fn with_handle<T>(self, f: impl FnOnce(BusHandle) -> T) -> T {
        f(self.acquire())
    }
}

/// A handle to an SPI bus on the Flipper Zero.
pub struct BusHandle {
    handle: &'static mut sys::FuriHalSpiBusHandle,
    needs_init: bool,
}

impl Drop for BusHandle {
    fn drop(&mut self) {
        unsafe { sys::furi_hal_spi_release(self.handle) };
        if self.needs_init {
            unsafe { sys::furi_hal_spi_bus_handle_deinit(self.handle) };
        }
    }
}

impl BusHandle {
    /// Acquires a handle to the given SPI bus.
    ///
    /// Blocks indefinitely until the Flipper Zero bus is locally available.
    fn acquire(handle: &'static mut sys::FuriHalSpiBusHandle, needs_init: bool) -> Self {
        if needs_init {
            unsafe { sys::furi_hal_spi_bus_handle_init(handle) };
        }
        // TODO: Do we need to do anything about the CS transition?
        unsafe { sys::furi_hal_spi_acquire(handle) };
        Self { handle, needs_init }
    }

    /// Receives data.
    pub fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_spi_bus_rx(
                self.handle,
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

    /// Transmits data.
    pub fn write_all(&mut self, data: &[u8], timeout: Duration) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_spi_bus_tx(
                self.handle,
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

    /// Transmits and receives data simultaneously.
    pub fn write_and_read(
        &mut self,
        tx_data: &[u8],
        rx_buf: &mut [u8],
        timeout: Duration,
    ) -> Result<(), Error> {
        if tx_data.len() != rx_buf.len() {
            Err(Error::DataLengthMismatch)
        } else if unsafe {
            sys::furi_hal_spi_bus_trx(
                self.handle,
                tx_data.as_ptr(),
                rx_buf.as_mut_ptr(),
                tx_data.len(),
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Transmits and receives data simultaneously with DMA.
    pub fn write_and_read_dma(
        &mut self,
        tx_buf: &mut [u8],
        rx_buf: &mut [u8],
        timeout: Duration,
    ) -> Result<(), Error> {
        if tx_buf.len() != rx_buf.len() {
            Err(Error::DataLengthMismatch)
        } else if unsafe {
            sys::furi_hal_spi_bus_trx_dma(
                self.handle,
                tx_buf.as_mut_ptr(),
                rx_buf.as_mut_ptr(),
                tx_buf.len(),
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
    DataLengthMismatch,
    TransferFailed,
}
