//! Furi dialogs.

#[cfg(feature = "alloc")]
use alloc::ffi::CString;

use core::ffi::{c_char, CStr};
use core::marker::PhantomData;
use core::ptr;

use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

use super::canvas::Align;

const RECORD_DIALOGS: *const c_char = sys::c_string!("dialogs");

#[cfg(feature = "alloc")]
const BUTTON_OK: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"OK\0") };

/// A handle to the Dialogs app.
pub struct DialogsApp {
    data: UnsafeRecord<sys::DialogsApp>,
}

/// A dialog message.
pub struct DialogMessage<'a> {
    data: *mut sys::DialogMessage,
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
    /// Obtains a handle to the Dialogs app.
    pub fn open() -> Self {
        Self {
            data: unsafe { UnsafeRecord::open(RECORD_DIALOGS) },
        }
    }

    /// Displays a message.
    pub fn show(&mut self, message: &DialogMessage) -> DialogMessageButton {
        let data = self.data.as_raw();
        let button_sys = unsafe { sys::dialog_message_show(data, message.data) };

        DialogMessageButton::from_sys(button_sys).expect("Invalid button")
    }
}

impl<'a> DialogMessage<'a> {
    /// Allocates a new dialog message.
    pub fn new() -> Self {
        let data = unsafe { sys::dialog_message_alloc() };
        assert!(!data.is_null());

        Self {
            data,
            _phantom: PhantomData,
        }
    }

    /// Sets the labels of the buttons.
    pub fn set_buttons(
        &mut self,
        left: Option<&'a CStr>,
        center: Option<&'a CStr>,
        right: Option<&'a CStr>,
    ) {
        let left = left.map_or(ptr::null(), |l| l.as_ptr());
        let center = center.map_or(ptr::null(), |l| l.as_ptr());
        let right = right.map_or(ptr::null(), |l| l.as_ptr());

        unsafe {
            sys::dialog_message_set_buttons(self.data, left, center, right);
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
        unsafe {
            sys::dialog_message_set_header(
                self.data,
                header.as_ptr(),
                x,
                y,
                horizontal.into(),
                vertical.into(),
            );
        }
    }

    /// Sets the body text.
    pub fn set_text(&mut self, text: &'a CStr, x: u8, y: u8, horizontal: Align, vertical: Align) {
        unsafe {
            sys::dialog_message_set_text(
                self.data,
                text.as_ptr(),
                x,
                y,
                horizontal.into(),
                vertical.into(),
            );
        }
    }

    /// Clears the header text.
    pub fn clear_header(&mut self) {
        unsafe {
            sys::dialog_message_set_header(
                self.data,
                ptr::null(),
                0,
                0,
                sys::Align_AlignLeft,
                sys::Align_AlignTop,
            );
        }
    }

    /// Clears the body text.
    pub fn clear_text(&mut self) {
        unsafe {
            sys::dialog_message_set_text(
                self.data,
                ptr::null(),
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
        unsafe {
            sys::dialog_message_free(self.data);
        }
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
    let text = CString::new(text.as_bytes()).unwrap();

    let mut dialogs = DialogsApp::open();
    let mut message = DialogMessage::new();

    message.set_text(&text, 0, 0, Align::Left, Align::Top);
    message.set_buttons(None, Some(BUTTON_OK), None);

    dialogs.show(&message);
}
