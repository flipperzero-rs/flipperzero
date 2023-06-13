use crate::{gui::canvas::CanvasView, input::InputEvent};
use alloc::boxed::Box;
use core::ptr::NonNull;
use flipperzero_sys::{self as sys, View as SysView};

pub struct View<C: ViewCallbacks> {
    raw: NonNull<SysView>,
    callbacks: NonNull<C>,
}

impl<C: ViewCallbacks> View<C> {
    pub fn new(callbacks: C) -> Self {
        // SAFETY: allocation either succeeds producing a valid non-null pointer
        // or stops the system on OOM
        let raw = unsafe { NonNull::new_unchecked(sys::view_alloc()) };
        let callbacks = Box::into_raw(Box::new(callbacks));
        // SAFETY: callbacks is a valid non-null pointer
        let callbacks = unsafe { NonNull::new_unchecked(callbacks) };

        Self { raw, callbacks }
    }

    /// Creates a copy of raw pointer to the [`sys::View`].
    pub fn as_raw(&self) -> *mut SysView {
        self.raw.as_ptr()
    }
}

impl<C: ViewCallbacks> Drop for View<C> {
    fn drop(&mut self) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::view_free(raw) }

        let callbacks = self.callbacks.as_ptr();
        // SAFETY: `callbacks` has been created via `Box`
        let _ = unsafe { Box::from_raw(callbacks) };
    }
}

pub trait ViewCallbacks {
    fn on_draw(&mut self, _canvas: CanvasView) {}
    fn on_input(&mut self, _event: InputEvent) {}
    // TODO: the remaining callbacks and actual usage of callbacks
}

impl ViewCallbacks for () {}
