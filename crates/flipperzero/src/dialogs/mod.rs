//! Flipper Zero dialogs.

#[cfg(feature = "alloc")]
use alloc::ffi::CString;

use crate::gui::canvas::Align;
use core::{
    ffi::{c_char, CStr},
    marker::PhantomData,
    ptr,
    ptr::NonNull,
};
use flipperzero_sys::{self as sys, furi::UnsafeRecord};

/// A handle to the Dialogs app.
pub struct DialogsApp {
    data: UnsafeRecord<sys::DialogsApp>,
}

/// A dialog message.
pub struct DialogMessage<'a> {
    raw: NonNull<sys::DialogMessage>,
    _phantom: PhantomData<&'a CStr>,
}

/// Button pressed on a dialog.
pub enum DialogMessageButton {
    Back,
    Left,
    Right,
    Center,
}

impl DialogsApp {
    const RECORD_DIALOGS: *const c_char = sys::c_string!("dialogs");

    /// Obtains a handle to the Dialogs app.
    pub fn open() -> Self {
        // SAFETY: `RECORD_DIALOGS` is a constant
        let data = unsafe { UnsafeRecord::open(Self::RECORD_DIALOGS) };
        Self { data }
    }

    /// Displays a message.
    pub fn show(&mut self, message: &DialogMessage) -> DialogMessageButton {
        let data = self.data.as_raw();
        let message = message.as_raw();
        let button_sys = unsafe { sys::dialog_message_show(data, message) };

        DialogMessageButton::from_sys(button_sys).expect("Invalid button")
    }
}

impl<'a> DialogMessage<'a> {
    /// Allocates a new dialog message.
    pub fn new() -> Self {
        // SAFETY: allocation either suceeds producing a valid pointer or terminates execution
        let data = unsafe { NonNull::new_unchecked(sys::dialog_message_alloc()) };

        Self {
            raw: data,
            _phantom: PhantomData,
        }
    }

    pub fn as_raw(&self) -> *mut sys::DialogMessage {
        self.raw.as_ptr()
    }

    /// Sets the labels of the buttons.
    pub fn set_buttons(
        &mut self,
        left: Option<&'a CStr>,
        center: Option<&'a CStr>,
        right: Option<&'a CStr>,
    ) {
        let data = self.raw.as_ptr();
        let left = left.map_or(ptr::null(), |c_str| c_str.as_ptr());
        let center = center.map_or(ptr::null(), |c_str| c_str.as_ptr());
        let right = right.map_or(ptr::null(), |c_str| c_str.as_ptr());

        // SAFTY: `data` is always a valid pointer
        // and all other pointers are valid or null
        unsafe {
            sys::dialog_message_set_buttons(data, left, center, right);
        }
    }

    /// Sets the header text.
    pub fn set_header(
        &mut self,
        header: &'a CStr,
        x: u8,
        y: u8,
        horizontal: Align,
        vertical: Align,
    ) {
        let data = self.raw.as_ptr();
        let header = header.as_ptr();
        let horizontal = horizontal.into();
        let vertical = vertical.into();
        // SAFTY: `data` and `header` are always valid pointers
        // and all values are corrrect
        unsafe {
            sys::dialog_message_set_header(data, header, x, y, horizontal, vertical);
        }
    }

    /// Sets the body text.
    pub fn set_text(&mut self, text: &'a CStr, x: u8, y: u8, horizontal: Align, vertical: Align) {
        let data = self.raw.as_ptr();
        let text = text.as_ptr();
        let horizontal = horizontal.into();
        let vertical = vertical.into();
        // SAFTY: `data` and `text` are always valid pointers
        // and all values are corrrect
        unsafe {
            sys::dialog_message_set_text(data, text, x, y, horizontal, vertical);
        }
    }

    /// Clears the header text.
    pub fn clear_header(&mut self) {
        let data = self.raw.as_ptr();
        let text = ptr::null();
        // SAFTY: `data` is always a valid pointer
        // and all values are corrrect
        unsafe {
            sys::dialog_message_set_header(
                data,
                text,
                0,
                0,
                sys::Align_AlignLeft,
                sys::Align_AlignTop,
            );
        }
    }

    /// Clears the body text.
    pub fn clear_text(&mut self) {
        let data = self.raw.as_ptr();
        let text = ptr::null();
        // SAFTY: `data` is always a valid pointer and all values are corrrect
        unsafe {
            sys::dialog_message_set_text(
                data,
                text,
                0,
                0,
                sys::Align_AlignLeft,
                sys::Align_AlignTop,
            );
        }
    }
}

impl<'a> Drop for DialogMessage<'a> {
    fn drop(&mut self) {
        let data = self.raw.as_ptr();
        // SAFETY: `data` is a valid pointer
        // which has been created by a call to `dialog_message_alloc`
        unsafe { sys::dialog_message_free(data) };
    }
}

impl DialogMessageButton {
    fn from_sys(sys: sys::DialogMessageButton) -> Option<Self> {
        match sys {
            sys::DialogMessageButton_DialogMessageButtonBack => Some(Self::Back),
            sys::DialogMessageButton_DialogMessageButtonLeft => Some(Self::Left),
            sys::DialogMessageButton_DialogMessageButtonCenter => Some(Self::Center),
            sys::DialogMessageButton_DialogMessageButtonRight => Some(Self::Right),
            _ => None,
        }
    }
}

impl Default for DialogMessage<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Displays a simple dialog.
#[cfg(feature = "alloc")]
pub fn alert(text: &str) {
    // SAFETY: string is known to end with NUL
    const BUTTON_OK: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"OK\0") };

    let text = CString::new(text.as_bytes()).unwrap();

    let mut dialogs = DialogsApp::open();
    let mut message = DialogMessage::new();

    message.set_text(&text, 0, 0, Align::Left, Align::Top);
    message.set_buttons(None, Some(BUTTON_OK), None);

    dialogs.show(&message);
}
