//! Demonstrates use of threads on the Flipper Zero by spawning two threads and dropping the first +
//! while waiting for the second to complete.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

extern crate alloc;

use alloc::borrow::ToOwned;
use core::time::Duration;

use flipperzero::{furi::thread, println};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(name = "Threads example");

// Define the entry function
entry!(main);

// Entry point
fn main(_args: *mut u8) -> i32 {
    println!("Main app started!");

    let first = thread::spawn(|| {
        println!("First thread started!");
        thread::sleep(Duration::from_secs(5));
        println!("First thread finished!");
        0
    });

    thread::sleep(Duration::from_secs(1));

    let second = thread::Builder::new()
        .name("Flipper".to_owned())
        .expect("name is valid")
        .spawn(|| {
            println!("Second thread started!");
            thread::sleep(Duration::from_secs(2));
            println!("Second thread finished!");
            0
        });

    for (i, thread) in [&thread::current(), first.thread(), second.thread()]
        .into_iter()
        .enumerate()
    {
        if let Some(name) = thread.name() {
            println!("Running thread {} ({})", i, name);
        } else {
            println!("Running unnamed thread {}", i);
        }
    }

    // We can either drop the `JoinHandle` and let the thread complete in the background,
    // or we can join the thread which blocks until its completion. Because `first` will
    // outlive this `main` function, the app will block just after `main` returns until
    // the thread completes.
    drop(first);
    second.join();

    println!("Main app finished!");
    0
}
