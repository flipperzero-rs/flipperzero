//! Furi HAL CDC API.

use core::ffi::c_void;

use crate::opaque;

opaque!(UsbCdcLineCoding);

#[repr(C)]
pub struct CdcCallbacks {
    tx_ep_callback: extern "C" fn(context: *mut c_void),
    rx_ep_callback: extern "C" fn(context: *mut c_void),
    state_callback: extern "C" fn(context: *mut c_void, state: u8),
    ctrl_line_callback: extern "C" fn(context: *mut c_void, state: u8),
    config_callback: extern "C" fn(context: *mut c_void, config: *mut UsbCdcLineCoding),
}

extern "C" {
    #[link_name = "furi_hal_cdc_get_ctrl_line_state"]
    pub fn get_ctrl_line_state(if_num: u8) -> u8;
    #[link_name = "furi_hal_cdc_get_port_settings"]
    pub fn get_port_settings(if_num: u8) -> *mut UsbCdcLineCoding;
    #[link_name = "furi_hal_cdc_receive"]
    pub fn receive(if_num: u8, buf: *mut u8, max_len: u16) -> i32;
    #[link_name = "furi_hal_cdc_send"]
    pub fn send(if_num: u8, buf: *mut u8, len: u16);
    #[link_name = "furi_hal_cdc_set_callbacks"]
    pub fn set_callbacks(if_num: u8, callbacks: *mut CdcCallbacks, context: *mut c_void);
}
