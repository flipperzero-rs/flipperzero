//! Demonstrates use of the Flipper Zero GUI.
//!
//! This app write "Hello, Rust!" to the display.

#![no_main]
#![no_std]
#![forbid(unsafe_code)]

mod xbms;

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate alloc;
extern crate flipperzero_alloc;

use alloc::{ffi::CString, string::ToString};
use core::{ffi::CStr, time::Duration};

use flipperzero::{
    furi::message_queue::MessageQueue,
    gui::xbm::{ByteArray, XbmImage},
    gui::{
        canvas::CanvasView,
        view_port::{ViewPort, ViewPortCallbacks},
        Gui, GuiLayer,
    },
    input::{InputEvent, InputKey, InputType},
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys::furi::Status;

manifest!(name = "Rust GUI example");
entry!(main);

const PLUS_IMAGE: XbmImage<ByteArray<8>> = XbmImage::new_from_array::<8, 8>([
    0b00_11_11_00,
    0b00_11_11_00,
    0b11_11_11_11,
    0b11_11_11_11,
    0b11_11_11_11,
    0b11_11_11_11,
    0b00_11_11_00,
    0b10_11_11_01,
]);

const RS_IMAGE: XbmImage<ByteArray<8>> = XbmImage::new_from_array::<8, 8>([
    0b11100000u8.reverse_bits(),
    0b10010000u8.reverse_bits(),
    0b11100000u8.reverse_bits(),
    0b10100110u8.reverse_bits(),
    0b10011000u8.reverse_bits(),
    0b00000110u8.reverse_bits(),
    0b00000001u8.reverse_bits(),
    0b00000110u8.reverse_bits(),
]);

fn main(_args: *mut u8) -> i32 {
    let exit_event_queue = MessageQueue::new(32);

    struct State<'a> {
        text: &'a CStr,
        exit_event_queue: &'a MessageQueue<()>,
        counter: u8,
    }

    impl ViewPortCallbacks for State<'_> {
        fn on_draw(&mut self, mut canvas: CanvasView) {
            canvas.draw_xbm(2, 2, &PLUS_IMAGE);
            canvas.draw_str(10, 31, self.text);
            let bottom_text = CString::new(self.counter.to_string().as_bytes())
                .expect("should be a valid string");
            canvas.draw_str(80, 10, bottom_text);
            canvas.draw_xbm(100, 50, &RS_IMAGE);
            canvas.draw_xbm(0, 32, &xbms::ferris::IMAGE);
        }

        fn on_input(&mut self, event: InputEvent) {
            if event.r#type == InputType::Press {
                match event.key {
                    InputKey::Up => {
                        self.counter = (self.counter + 1) % 10;
                    }
                    InputKey::Down => {
                        self.counter = if self.counter == 0 {
                            10
                        } else {
                            self.counter - 1
                        };
                    }
                    InputKey::Back => {
                        self.exit_event_queue
                            .put((), Duration::MAX)
                            .expect("failed to put event into the queue");
                    }
                    _ => {}
                }
            }
        }
    }
    let view_port = ViewPort::new(State {
        text: CStr::from_bytes_with_nul(b"Hi there!\0").expect("correct string"),
        exit_event_queue: &exit_event_queue,
        counter: 0,
    });

    let mut gui = Gui::new();
    let mut view_port = gui.add_view_port(view_port, GuiLayer::Fullscreen);

    let status = loop {
        match exit_event_queue.get(Duration::from_millis(100)) {
            Ok(()) => {
                println!("Exit pressed");
                break 0;
            }
            Err(e) => {
                if e != Status::ERR_TIMEOUT {
                    println!("ERROR while receiving event: {:?}", e);
                    break 1;
                }
            }
        }
    };
    view_port.view_port_mut().set_enabled(false);

    status
}
