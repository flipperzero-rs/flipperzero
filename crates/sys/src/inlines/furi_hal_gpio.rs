//! Inlines for Furi HAL GPIO interface.
//!
//! See: [`furi_hal_gpio.h`][1]
//!
//! [1]: https://github.com/flipperdevices/flipperzero-firmware/blob/release/firmware/targets/f7/furi_hal/furi_hal_gpio.h

use crate as sys;

/// Number of GPIO on one port.
pub const GPIO_NUMBER: usize = 16;

/// GPIO write pin.
///
/// # Safety
///
/// `gpio` must be non-null, and the memory it points to must be initialized.
#[inline]
pub unsafe extern "C" fn furi_hal_gpio_write(gpio: *const sys::GpioPin, state: bool) {
    let port = (*gpio).port;
    let pin = (*gpio).pin;

    furi_hal_gpio_write_port_pin(port, pin, state)
}

/// GPIO write pin.
///
/// # Safety
///
/// `port` must be non-null, and the memory it points to must be initialized.
#[inline]
pub unsafe extern "C" fn furi_hal_gpio_write_port_pin(
    port: *mut sys::GPIO_TypeDef,
    pin: u16,
    state: bool,
) {
    // writing to BSSR is an atomic operation
    core::ptr::write_volatile(
        &mut (*port).BSRR,
        (pin as u32) << if state { 0 } else { GPIO_NUMBER },
    );
}

/// GPIO read pin.
///
/// # Safety
///
/// `gpio` must be non-null, and the memory it points to must be initialized.
#[inline]
pub unsafe extern "C" fn furi_hal_gpio_read(gpio: *const sys::GpioPin) -> bool {
    let port = (*gpio).port;
    let pin = (*gpio).pin;

    furi_hal_gpio_read_port_pin(port, pin)
}

/// GPIO read pin.
///
/// # Safety
///
/// `port` must be non-null, and the memory it points to must be initialized.
#[inline]
pub unsafe extern "C" fn furi_hal_gpio_read_port_pin(
    port: *mut sys::GPIO_TypeDef,
    pin: u16,
) -> bool {
    core::ptr::read_volatile(&(*port).IDR) & pin as u32 != 0x00
}
