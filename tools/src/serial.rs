//! UART serial interface.

use serialport::{SerialPortType, SerialPortInfo};

/// STMicroelectronics Virtual COM Port
const HWID: (u16, u16) = (0x0483, 0x5740);
pub const BAUD_115200: u32 = 115200;

/// Try to find the Flipper Zero USB serial port.
pub fn find_flipperzero() -> Option<SerialPortInfo> {
    let ports = serialport::available_ports().ok()?;
    
    ports.into_iter().find(|p| {
        match &p.port_type {
            SerialPortType::UsbPort(usb) if (usb.vid, usb.pid) == HWID => true,
            _ => false,
        }
    })
}
