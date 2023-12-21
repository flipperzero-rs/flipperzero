use core::ptr::NonNull;

use flipperzero_sys::{self as sys, View as SysView};

use crate::{gui::canvas::CanvasView, input::InputEvent, internals::alloc::NonUniqueBox};

/// UI view.
pub struct View<C: ViewCallbacks> {
    inner: ViewInner,
    callbacks: NonUniqueBox<C>,
}

impl<C: ViewCallbacks> View<C> {
    pub fn new(callbacks: C) -> Self {
        let inner = ViewInner::new();
        let callbacks = NonUniqueBox::new(callbacks);

        Self { inner, callbacks }
    }

    /// Creates a copy of raw pointer to the [`sys::View`].
    #[inline]
    #[must_use]
    pub fn as_raw(&self) -> *mut SysView {
        self.inner.0.as_ptr()
    }
}

/// Plain alloc-free wrapper over a [`SysView`].
struct ViewInner(NonNull<SysView>);

impl ViewInner {
    fn new() -> Self {
        // SAFETY: allocation either succeeds producing a valid non-null pointer
        // or stops the system on OOM
        Self(unsafe { NonNull::new_unchecked(sys::view_alloc()) })
    }
}

impl Drop for ViewInner {
    fn drop(&mut self) {
        let raw = self.0.as_ptr();
        // SAFETY: `raw` is valid
        unsafe { sys::view_free(raw) }
    }
}

#[allow(unused_variables)]
pub trait ViewCallbacks {
    fn on_draw(&mut self, canvas: CanvasView) {}
    fn on_input(&mut self, event: InputEvent) {}
    // TODO: the remaining callbacks and actual usage of callbacks
}

impl ViewCallbacks for () {}
