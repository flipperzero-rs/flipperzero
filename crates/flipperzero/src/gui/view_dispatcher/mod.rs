mod r#type;

use alloc::collections::BTreeSet;
use core::{
    ffi::c_void,
    marker::PhantomData,
    num::NonZeroU32,
    ptr::{self, NonNull},
};

use flipperzero_sys::{self as sys, ViewDispatcher as SysViewDispatcher};
pub use r#type::*;

use crate::{
    gui::{view::View, Gui},
    internals::alloc::NonUniqueBox,
};

type ViewSet = BTreeSet<u32>;

pub mod view_id {

    /// Special view ID which hides drawing view_port.
    const NONE: u32 = 0xFFFFFFFF;

    /// Special view ID which ignores navigation event.
    const IGNORE: u32 = 0xFFFFFFFE;
}

pub struct ViewDispatcher<'a, C: ViewDispatcherCallbacks, const QUEUE: bool = true> {
    inner: ViewDispatcherInner,
    context: NonUniqueBox<Context<C>>,
    _phantom: PhantomData<&'a mut Gui>,
}

struct Context<C: ViewDispatcherCallbacks> {
    view_dispatcher: NonNull<SysViewDispatcher>,
    callbacks: C,
    // TODO: propose API to Flipper for checked view addition/removal
    views: ViewSet,
}

impl<'a, C: ViewDispatcherCallbacks, const QUEUE: bool> ViewDispatcher<'a, C, QUEUE> {
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::{
    /// #     gui::{
    /// #         view_dispatcher::{
    /// #             ViewDispatcher, ViewDispatcherCallbacks,
    /// #             ViewDispatcherRef, ViewDispatcherOps, ViewDispatcherType,
    /// #         },
    /// #         Gui,
    /// #     },
    /// #     log,
    /// # };
    /// struct MyCallbacks {
    ///     value: u32,
    /// }
    /// impl ViewDispatcherCallbacks for MyCallbacks {
    ///     fn on_custom(&mut self, view_dispatcher: ViewDispatcherRef<'_>, event: u32) -> bool {
    ///         log!("{} + {} = {}", self.value, event, self.value + event);
    ///         true
    ///     }
    /// }
    /// let mut gui = Gui::new();
    /// let mut view_dispatcher = ViewDispatcher::new(MyCallbacks {
    ///     value: 10
    /// }, &mut gui, ViewDispatcherType::Fullscreen);
    ///
    /// view_dispatcher.send_custom_event(20);
    /// // should print `10 + 20 = 30`
    /// ```
    pub fn new(callbacks: C, gui: &'a Gui, kind: ViewDispatcherType) -> Self {
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
        let context = NonUniqueBox::new(Context {
            view_dispatcher: inner.0,
            callbacks,
            views: BTreeSet::new(),
        });

        {
            let raw = inner.0.as_ptr();
            let gui = gui.as_raw();
            let kind = kind.into();
            // SAFETY: both pointers are valid and `kind` is a valid numeric value
            // and the newly created view dispatcher does not have a Gui yet
            unsafe { sys::view_dispatcher_attach_to_gui(raw, gui, kind) };
        }

        // SAFETY: both pointers are guaranteed to be non-null
        let view_dispatcher = Self {
            inner,
            context,
            _phantom: PhantomData,
        };

        let raw = view_dispatcher.as_raw();
        if QUEUE {
            // SAFETY: `raw` is a valid pointer
            // and corresponds to a newly created `ViewDispatcher`
            // which does not have a queue yet
            unsafe { sys::view_dispatcher_enable_queue(raw) };
        }

        // and store context if at least one event should be registered
        if register_custom_event || register_navigation_callback || tick_period.is_some() {
            let context = view_dispatcher.context.as_ptr().cast();
            // SAFETY: `raw` is valid
            // and `callbacks` is valid and lives with this struct
            unsafe { sys::view_dispatcher_set_event_callback_context(raw, context) };
        }

        if register_custom_event {
            pub unsafe extern "C" fn dispatch_custom<C: ViewDispatcherCallbacks>(
                context: *mut c_void,
                event: u32,
            ) -> bool {
                let context: *mut Context<C> = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewDispatcher`
                // and the callback is accessed exclusively by this function
                let context = unsafe { &mut *context };
                context.callbacks.on_custom(
                    ViewDispatcherRef {
                        raw: context.view_dispatcher,
                        views: &mut context.views,
                        _phantom: PhantomData,
                    },
                    event,
                )
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
                let context: *mut Context<C> = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewDispatcher`
                // and the callback is accessed exclusively by this function
                let context = unsafe { &mut *context };
                context.callbacks.on_navigation(ViewDispatcherRef {
                    raw: context.view_dispatcher,
                    views: &mut context.views,
                    _phantom: PhantomData,
                })
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
                let context: *mut Context<C> = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewDispatcher`
                // and the callback is accessed exclusively by this function
                let context = unsafe { &mut *context };
                context.callbacks.on_tick(ViewDispatcherRef {
                    raw: context.view_dispatcher,
                    views: &mut context.views,
                    _phantom: PhantomData,
                });
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
    pub const fn as_raw(&self) -> *mut SysViewDispatcher {
        self.inner.0.as_ptr()
    }
}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcher<'a, C, false> {
    // /// Creates a new view dispatcher without a queue.
    // ///
    // /// This is equivalent to calling: [`ViewDispatcher::new`] with `QUEUE` set to `false`.
    // pub fn with_no_queue(callbacks: C) -> Self {
    //     Self::new(callbacks)
    // }

    /// Enables the queue for this view dispatcher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use flipperzero::gui::view_dispatcher::ViewDispatcher;
    /// // create a view dispatcher with no queue
    /// let view_dispatcher = ViewDispatcher::with_no_queue(());
    /// // ... do something ...
    /// // and now enable the queue for the view dispatcher
    /// let view_dispatcher = view_dispatcher.enable_queue();
    /// ```
    pub fn enable_queue(self) -> ViewDispatcher<'a, C, true> {
        // SAFETY: `raw` is a valid pointer
        // and corresponds to a `ViewDispatcher`
        // which does not have a queue yet
        let raw = self.as_raw();
        unsafe { sys::view_dispatcher_enable_queue(raw) };

        ViewDispatcher {
            inner: self.inner,
            context: self.context,
            _phantom: self._phantom,
        }
    }
}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcher<'a, C, true> {
    /// Runs this view dispatcher.
    ///
    /// This will block until the view dispatcher gets stopped.
    pub fn run(self) -> Self {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        // and this is a `ViewDispatcher` with a queue
        unsafe { sys::view_dispatcher_run(raw) };
        self
    }
}

/// Reference to a ViewDispatcher.
pub struct ViewDispatcherRef<'a> {
    raw: NonNull<SysViewDispatcher>,
    views: &'a mut ViewSet,
    _phantom: PhantomData<&'a mut SysViewDispatcher>,
}

/// Operations on an initialized view dispatcher which has a queue associated with it.
pub trait ViewDispatcherOps: internals::InitViewDispatcherRaw {
    fn send_custom_event(&mut self, event: u32) {
        let raw = self.raw();
        // SAFETY: `raw` should be valid and point to a ViewDispatcher with a queue
        unsafe { sys::view_dispatcher_send_custom_event(raw, event) };
    }

    /// Stops this view dispatcher.
    ///
    /// This will make the [ViewDispatcher::<_, true>::run] caller unfreeze.
    fn stop(&mut self) {
        let raw = self.raw();
        // SAFETY: `raw` should be valid and point to a ViewDispatcher with a queue
        unsafe { sys::view_dispatcher_stop(raw) };
    }

    // fn add_view(&mut self, id: u32, view: &mut View<'_>) {
    //     if self.views().insert(id) {
    //         let raw = self.raw();
    //         unsafe { sys::view_dispatcher_add_view(raw, id) };
    //     }
    // }

    fn switch_to_view(&mut self, id: u32) {
        if self.views().contains(&id) {
            let raw = self.raw();
            unsafe { sys::view_dispatcher_switch_to_view(raw, id) };
        }
    }

    fn remove_view(&mut self, id: u32) -> Option<()> {
        if self.views_mut().remove(&id) {
            let raw = self.raw();
            unsafe { sys::view_dispatcher_remove_view(raw, id) }
            Some(())
        } else {
            None
        }
    }
}
impl<T: internals::InitViewDispatcherRaw> ViewDispatcherOps for T {}

unsafe impl<C: ViewDispatcherCallbacks> internals::InitViewDispatcherRaw
    for ViewDispatcher<'_, C, true>
{
    #[inline(always)]
    fn raw(&self) -> *mut SysViewDispatcher {
        self.inner.0.as_ptr()
    }

    #[inline(always)]
    fn views(&self) -> &ViewSet {
        let context = self.context.as_ptr();
        // SAFETY: if this method is accessed through `ViewDispatcher`
        // then no one else should be able to use it
        &unsafe { &*context }.views
    }

    #[inline(always)]
    fn views_mut(&mut self) -> &mut ViewSet {
        let context = self.context.as_ptr();
        // SAFETY: if this method is accessed through `ViewDispatcher`
        // then no one else should be able to use it
        &mut unsafe { &mut *context }.views
    }
}

unsafe impl internals::InitViewDispatcherRaw for ViewDispatcherRef<'_> {
    #[inline(always)]
    fn raw(&self) -> *mut SysViewDispatcher {
        self.raw.as_ptr()
    }

    #[inline(always)]
    fn views(&self) -> &ViewSet {
        self.views
    }

    #[inline(always)]
    fn views_mut(&mut self) -> &mut ViewSet {
        self.views
    }
}

/// Internal representation of view dispatcher.
/// This is a thin non-null pointer to [`SysViewDispatcher`]
/// which performs its automatic [allocation][Self::new] and [deallocation](Self::drop).
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

#[allow(unused_variables)]
pub trait ViewDispatcherCallbacks {
    /// Handles a custom event,
    ///
    ///
    fn on_custom(&mut self, view_dispatcher: ViewDispatcherRef<'_>, event: u32) -> bool {
        false
    }

    fn on_navigation(&mut self, view_dispatcher: ViewDispatcherRef<'_>) -> bool {
        false
    }

    fn on_tick(&mut self, view_dispatcher: ViewDispatcherRef<'_>) {}

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

mod internals {
    use super::{SysViewDispatcher, ViewSet};

    /// A structure wrapping a raw [`SysViewDispatcher`] with an initialized queue.
    ///
    /// # Safety
    ///
    /// This trait should be implemented so that the provided pointer is always valid
    /// and points to the [`SysViewDispatcher`] which has a queue.
    pub unsafe trait InitViewDispatcherRaw {
        fn raw(&self) -> *mut SysViewDispatcher;

        fn views(&self) -> &ViewSet;

        fn views_mut(&mut self) -> &mut ViewSet;
    }
}
