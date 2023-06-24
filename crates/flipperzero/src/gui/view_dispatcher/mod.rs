mod r#type;

use crate::internals::alloc::NonUniqueBox;
use core::{
    ffi::c_void,
    num::NonZeroU32,
    ptr::{self, NonNull},
};
use flipperzero_sys::{self as sys, ViewDispatcher as SysViewDispatcher};

pub use r#type::*;

pub struct ViewDispatcher<C: ViewDispatcherCallbacks, const QUEUE: bool> {
    inner: ViewDispatcherInner,
    callbacks: NonUniqueBox<C>,
}

impl<C: ViewDispatcherCallbacks, const QUEUE: bool> ViewDispatcher<C, QUEUE> {
    pub fn new(callbacks: C) -> Self {
        // discover which callbacks should be registered
        let register_custom_event = !ptr::eq(
            C::on_custom as *const c_void,
            <() as ViewDispatcherCallbacks>::on_custom as *const c_void,
        );
        let register_navigation_callback = !ptr::eq(
            C::on_navigation as *const c_void,
            <() as ViewDispatcherCallbacks>::on_navigation as *const c_void,
        );
        let tick_period = (!ptr::eq(
            C::on_tick as *const c_void,
            <() as ViewDispatcherCallbacks>::on_tick as *const c_void,
        ))
        .then(|| callbacks.tick_period());

        let inner = ViewDispatcherInner::new();
        let callbacks = NonUniqueBox::new(callbacks);

        // SAFETY: both pointers are guaranteed to be non-null
        let view_dispatcher = Self { inner, callbacks };

        let raw = view_dispatcher.as_raw();
        if QUEUE {
            // SAFETY: `raw` is a valid pointer
            // and corresponds to a newly created `ViewPort`
            // which does not have a queue yet
            unsafe { sys::view_dispatcher_enable_queue(raw) };
        }

        // and store context if at least one event should be registered
        if register_custom_event || register_navigation_callback || tick_period.is_some() {
            let context = view_dispatcher.callbacks.as_ptr().cast();
            // SAFETY: `raw` is valid
            // and `callbacks` is valid and lives with this struct
            unsafe { sys::view_dispatcher_set_event_callback_context(raw, context) };
        }

        if register_custom_event {
            pub unsafe extern "C" fn dispatch_custom<C: ViewDispatcherCallbacks>(
                context: *mut c_void,
                event: u32,
            ) -> bool {
                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewPort`
                // and the callback is accessed exclusively by this function
                unsafe { &mut *context }.on_custom(event)
            }

            let callback = Some(dispatch_custom::<C> as _);
            // SAFETY: `raw` is valid
            // and `callbacks` is valid and lives with this struct
            unsafe { sys::view_dispatcher_set_custom_event_callback(raw, callback) };
        }
        if register_navigation_callback {
            pub unsafe extern "C" fn dispatch_navigation<C: ViewDispatcherCallbacks>(
                context: *mut c_void,
            ) -> bool {
                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewPort`
                // and the callback is accessed exclusively by this function
                unsafe { &mut *context }.on_navigation()
            }

            let callback = Some(dispatch_navigation::<C> as _);
            // SAFETY: `raw` is valid
            // and `callbacks` is valid and lives with this struct
            unsafe { sys::view_dispatcher_set_navigation_event_callback(raw, callback) };
        }
        if let Some(tick_period) = tick_period {
            pub unsafe extern "C" fn dispatch_tick<C: ViewDispatcherCallbacks>(
                context: *mut c_void,
            ) {
                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewPort`
                // and the callback is accessed exclusively by this function
                unsafe { &mut *context }.on_tick();
            }

            let tick_period = tick_period.get();
            let callback = Some(dispatch_tick::<C> as _);
            // SAFETY: `raw` is valid
            // and `callbacks` is valid and lives with this struct
            unsafe { sys::view_dispatcher_set_tick_event_callback(raw, callback, tick_period) };
        }

        view_dispatcher
    }

    #[inline]
    #[must_use]
    pub fn as_raw(&self) -> *mut SysViewDispatcher {
        self.inner.0.as_ptr()
    }
}

impl<C: ViewDispatcherCallbacks> ViewDispatcher<C, true> {
    pub fn run(&mut self) {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        // and this is a `ViewDispatcher` with a queue
        unsafe { sys::view_dispatcher_run(raw) };
    }

    pub fn stop(&mut self) {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        // and this is a `ViewDispatcher` with a queue
        unsafe { sys::view_dispatcher_stop(raw) };
    }

    pub fn send_custom_event(&mut self, event: u32) {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        // and this is a `ViewDispatcher` with a queue
        unsafe { sys::view_dispatcher_send_custom_event(raw, event) };
    }
}

struct ViewDispatcherInner(NonNull<SysViewDispatcher>);

impl ViewDispatcherInner {
    fn new() -> Self {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM,
        Self(unsafe { NonNull::new_unchecked(sys::view_dispatcher_alloc()) })
    }
}

impl Drop for ViewDispatcherInner {
    fn drop(&mut self) {
        let raw = self.0.as_ptr();
        // SAFETY: `raw` is valid
        unsafe { sys::view_dispatcher_free(raw) };
    }
}

pub trait ViewDispatcherCallbacks {
    fn on_custom(&mut self, _event: u32) -> bool {
        true
    }
    fn on_navigation(&mut self) -> bool {
        true
    }
    fn on_tick(&mut self) {}

    #[must_use]
    fn tick_period(&self) -> NonZeroU32 {
        // Some arbitrary default
        NonZeroU32::new(100).unwrap()
    }
}

impl ViewDispatcherCallbacks for () {
    // use MAX value since this should never be used normally
    fn tick_period(&self) -> NonZeroU32 {
        NonZeroU32::MAX
    }
}
