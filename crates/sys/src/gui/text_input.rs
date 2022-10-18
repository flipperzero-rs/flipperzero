//! Low-level bindings to the text-input widget.

use core::ffi::{c_char, c_void};
use crate::gui::view::View;
use crate::opaque;

opaque!(TextInput);

pub type OnDoneCallback = extern "C" fn(*mut c_void);

extern "C" {
    #[link_name = "text_input_alloc"]
    pub fn alloc() -> *mut TextInput;
    #[link_name = "text_input_free"]
    pub fn free(ti: *mut TextInput);
    #[link_name = "text_input_get_view"]
    pub fn get_view(vil: *mut TextInput) -> *mut View;

    #[link_name = "text_input_set_result_callback"]
    pub fn set_result_callback(ti: *mut TextInput, cb: OnDoneCallback, context: *mut c_void, buffer: *mut c_char, buffer_size: usize, clear_default_text: bool);

    #[link_name = "text_input_set_header_text"]
    pub fn set_header_text(ti: *mut TextInput, header: *const c_char);
}