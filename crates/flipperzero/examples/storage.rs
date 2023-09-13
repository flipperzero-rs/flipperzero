//! Storage example for Flipper Zero.
//! This app writes "Hello, Rust!" to a file on the SD card. Then it opens a file browser dialog and
//! lets the user select the file. Finally, it reads the file back and prints the contents to the console.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::dialogs::{DialogFileBrowserOptions, DialogsApp};
use flipperzero::furi::string::FuriString;
use flipperzero::io::*;
use flipperzero::println;
use flipperzero::storage::*;
use flipperzero_rt::{entry, manifest};

manifest!(name = "Rust storage example");
entry!(main);

fn main(_args: *mut u8) -> i32 {
    // First, we'll create a file on the SD card and write "Hello, Rust!" to it.
    let path = CStr::from_bytes_with_nul(b"/ext/hello-rust.txt\0").unwrap();
    let file = OpenOptions::new()
        .write(true)
        .create_always(true)
        .open(path);

    match file {
        Ok(mut handle) => {
            if let Err(e) = handle.write(b"Hello, Rust!") {
                println!("couldn't write to file: {}", e);
            }
        }
        Err(e) => println!("couldn't open path: {}", e),
    }

    // Next, we'll open a file browser dialog and let the user select the file.
    let mut dialogs_app = DialogsApp::open();
    let file_browser_options = DialogFileBrowserOptions::new().set_hide_ext(false);
    let mut start_path = FuriString::from(path);
    let result_path =
        dialogs_app.show_file_browser(Some(&mut start_path), Some(&file_browser_options));
    if let Some(result_path) = result_path {
        println!("file selected {}", result_path);
        let path = result_path.as_c_str();

        // Now, we'll open it and read it back.
        let mut buffer: [u8; 16] = [0; 16];
        let file = OpenOptions::new().read(true).open(path);

        match file {
            Ok(mut handle) => match handle.read(&mut buffer) {
                Ok(n) => println!("Read from file: {:?}", &buffer[..n]),
                Err(e) => println!("couldn't read from file: {}", e),
            },
            Err(e) => println!("couldn't open path: {}", e),
        }
    } else {
        println!("no file selected");
    }

    0 // File is synchronized and closed when `file` goes out of scope.
}
