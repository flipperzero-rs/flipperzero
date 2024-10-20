//! Example showcasing the use of a stream buffer on multiple threads.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

extern crate alloc;

use core::{ffi::CStr, num::NonZeroUsize};

use flipperzero::{furi, println};
use flipperzero_rt::{entry, manifest};

use core::time::Duration as CoreDuration;
use furi::time::Duration as FuriDuration;

// Define the FAP Manifest for this application
manifest!(name = "Stream buffer example");

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    // Create a stream buffer pair
    let size = NonZeroUsize::new(1024).unwrap();
    let trigger_level = 16;
    let stream_buffer = furi::stream_buffer::StreamBuffer::new(size, trigger_level);
    let (tx, rx) = stream_buffer.into_stream();

    let stream_buffer = tx.as_stream_buffer();

    // Stream buffer is empty
    assert_eq!(stream_buffer.spaces_available(), size.into());
    assert_eq!(stream_buffer.bytes_available(), 0);

    // Sending 4 bytes immediately
    assert_eq!(tx.send(&[1, 2, 3, 4]), 4);
    assert_eq!(stream_buffer.bytes_available(), 4);

    // Receive bytes
    let mut recv_buf = [0; 32];
    assert_eq!(rx.recv(&mut recv_buf), 4);
    assert_eq!(recv_buf[0..4], [1, 2, 3, 4]);

    // Move sender to another thread
    let tx_thread = furi::thread::spawn(move || {
        // Wait 2 seconds before we send some bytes
        furi::thread::sleep(CoreDuration::from_secs(2));
        assert_eq!(tx.send(&[5; 20]), 20);

        // Send some bytes in a loop to see how the receiver handles them
        for i in 4..20 {
            furi::thread::sleep(CoreDuration::from_millis(200));
            tx.send(&[i as u8; 3]);
        }

        0
    });

    // Move receiver to another thread
    let rx_thread = furi::thread::spawn(move || {
        let mut buf = [0; 32];

        // The sender waits 2 seconds, so we don't block and get no bytes
        assert_eq!(rx.recv(&mut buf), 0);

        // The sender sends 20 bytes after two seconds, that is more than the trigger, so we continue
        assert_eq!(rx.recv_blocking(&mut buf), 20);

        // Try to receive bytes as long as the sender is alive
        while rx.is_sender_alive() {
            let n = rx.recv_with_timeout(&mut buf, FuriDuration::from_secs(2));
            println!("got {} bytes: {:?}", n, buf[0..n]);
        }

        0
    });

    tx_thread.join();
    rx_thread.join();

    0
}
