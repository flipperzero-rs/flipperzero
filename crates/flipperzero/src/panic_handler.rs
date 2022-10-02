//! Panic handler for Furi applications.
//! This will print the panic info to stdout and then trigger a crash.

use core::ffi::CStr;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::str;

use flipperzero_sys::{c_string, furi};

use crate::furi::io::Stdout;

#[panic_handler]
pub fn panic(panic_info: &PanicInfo<'_>) -> ! {
    let mut stdout = Stdout;

    let thread_name = unsafe {
        let thread_id = furi::thread::get_current_id();
        let thread_name = furi::thread::get_name(thread_id);

        if thread_name.is_null() {
            "<unknown>"
        } else {
            str::from_utf8_unchecked(CStr::from_ptr(thread_name).to_bytes())
        }
    };

    // Format: "thread: 'App Name' paniced at 'panic!', panic.rs:5"
    let _ = write!(
        &mut stdout,
        "\x1b[0;31mthread '{thread_name}' {panic_info}\x1b[0m\r\n"
    );
    let _ = stdout.flush();

    unsafe {
        furi::thread::yield_(); // Allow console to flush
        furi::check::crash(c_string!("Rust panic\r\n"))
    }
}
