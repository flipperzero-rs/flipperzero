//! Bluetooth APIs for the Flipper Zero.

use flipperzero_sys as sys;

use crate::furi::string::FuriString;

pub mod beacon;

/// Returns `true` if core2 (which runs Bluetooth) is alive.
pub fn is_alive() -> bool {
    unsafe { sys::furi_hal_bt_is_alive() }
}

/// Checks if BLE state is active.
///
/// Returns `true` if the device is connected or advertising.
pub fn is_active() -> bool {
    unsafe { sys::furi_hal_bt_is_active() }
}

/// Returns a string containing the BT/BLE system component state.
pub fn dump_state() -> FuriString {
    let mut buffer = FuriString::new();
    unsafe { sys::furi_hal_bt_dump_state(buffer.as_mut_ptr()) }
    buffer
}
