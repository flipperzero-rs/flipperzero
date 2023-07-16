use core::{
    ffi::c_void,
    marker::PhantomData,
    ptr::{self, NonNull},
};

use flipperzero_sys::{self as sys, Icon as SysIcon, IconAnimation as SysIconAnimation};

use crate::{gui::icon::Icon, internals::alloc::NonUniqueBox};

/// Icon Animation
/// which can be [started](IconAnimation::start) and [stopped](IconAnimation::stop).
pub struct IconAnimation<'a, C: IconAnimationCallbacks> {
    inner: IconAnimationInner,
    callbacks: NonUniqueBox<C>,
    _parent_lifetime: PhantomData<&'a mut (IconAnimationInner, C)>,
}

impl<'a, C: IconAnimationCallbacks> IconAnimation<'a, C> {
    /// Creates a new icon animation from the specified [icon](`Icon`).
    pub fn new<'b: 'a>(icon: &'b Icon, callbacks: C) -> Self {
        let icon = icon.as_raw().cast_const();
        // SAFETY: `icon` is a valid pointer and will outlive `inner` while remaining const
        let inner = unsafe { IconAnimationInner::new(icon) };
        let callbacks = NonUniqueBox::new(callbacks);

        let icon_animation = Self {
            inner,
            callbacks,
            _parent_lifetime: PhantomData,
        };

        {
            pub unsafe extern "C" fn dispatch_update<C: IconAnimationCallbacks>(
                instance: *mut SysIconAnimation,
                context: *mut c_void,
            ) {
                // SAFETY: `icon_animation` is guaranteed to be a valid pointer
                let instance = unsafe { IconAnimationView::from_raw(instance) };

                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewPort`
                // and the callback is accessed exclusively by this function
                unsafe { &mut *context }.on_update(instance);
            }

            if !ptr::eq(
                C::on_update as *const c_void,
                <() as IconAnimationCallbacks>::on_update as *const c_void,
            ) {
                let raw = icon_animation.as_raw();
                let callback = Some(dispatch_update::<C> as _);
                let context = icon_animation.callbacks.as_ptr().cast();

                // SAFETY: `raw` and `callback` are valid
                // and `context` is valid as the box lives with this struct
                unsafe { sys::icon_animation_set_update_callback(raw, callback, context) };
            }
        }

        icon_animation
    }

    #[inline]
    #[must_use]
    pub fn as_raw(&self) -> *mut SysIconAnimation {
        self.inner.0.as_ptr()
    }

    /// Gets the width of this icon animation.
    pub fn get_width(&self) -> u8 {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_get_width(raw) }
    }

    /// Gets the height of this icon animation.
    pub fn get_height(&self) -> u8 {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_get_height(raw) }
    }

    /// Gets the dimensions of this icon animation.
    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.get_width(), self.get_height())
    }

    /// Starts this icon animation.
    pub fn start(&mut self) {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_start(raw) }
    }

    /// Stops this icon animation.
    pub fn stop(&mut self) {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_stop(raw) }
    }

    /// Checks if the current frame is the last one.
    pub fn is_last_frame(&self) -> bool {
        let raw = self.as_raw();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_is_last_frame(raw) }
    }
}

/// Plain alloc-free wrapper over a [`SysIconAnimation`].
struct IconAnimationInner(NonNull<SysIconAnimation>);

impl IconAnimationInner {
    /// Creates a new icon animation wrapper for the specified icon.
    ///
    /// # Safety
    ///
    /// `icon` should outlive the created wrapper
    /// and should not mutate during this wrapper's existence.
    unsafe fn new(icon: *const SysIcon) -> Self {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM,
        // `icon` is a valid pointer and `icon` outlives this animation
        Self(unsafe { NonNull::new_unchecked(sys::icon_animation_alloc(icon)) })
    }
}

impl Drop for IconAnimationInner {
    fn drop(&mut self) {
        let raw = self.0.as_ptr();
        // SAFETY: `raw` is a valid pointer
        // which should have been created via `icon_animation_alloc`
        unsafe { sys::icon_animation_free(raw) }
    }
}

/// View over system Icon Animation.
///
/// This is passed to [callbacks](IconAnimationCallbacks) of [`IconAnimation`].
pub struct IconAnimationView<'a> {
    raw: NonNull<SysIconAnimation>,
    _lifetime: PhantomData<&'a SysIconAnimation>,
}

impl IconAnimationView<'_> {
    /// Construct an `IconAnimationView` from a raw pointer.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid non-null pointer to [`sys::Canvas`]
    /// and the lifetime should be outlived by `raw` validity scope.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero::gui::icon_animation::IconAnimationView;
    ///
    /// let ptr = todo!();
    /// let icon_animation = unsafe { IconAnimationView::from_raw(ptr) };
    /// ```
    pub unsafe fn from_raw(raw: *mut SysIconAnimation) -> Self {
        Self {
            // SAFETY: caller should provide a valid pointer
            raw: unsafe { NonNull::new_unchecked(raw) },
            _lifetime: PhantomData,
        }
    }

    pub fn get_width(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_get_width(raw) }
    }

    pub fn get_height(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_get_height(raw) }
    }

    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.get_width(), self.get_height())
    }

    // TODO: decide if these methods should be available in view,
    //  i.e. if it is sound to call start/stop from callbacks
    // pub fn start(&mut self) {
    //     let raw = self.raw.as_ptr();
    //     // SAFETY: `raw` is valid
    //     unsafe { sys::icon_animation_start(raw) }
    // }
    //
    // pub fn stop(&mut self) {
    //     let raw = self.raw.as_ptr();
    //     // SAFETY: `raw` is valid
    //     unsafe { sys::icon_animation_stop(raw) }
    // }

    pub fn is_last_frame(&self) -> bool {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is valid
        unsafe { sys::icon_animation_is_last_frame(raw) }
    }
}

/// Callbacks of the [`IconAnimation`].
#[allow(unused_variables)]
pub trait IconAnimationCallbacks {
    fn on_update(&mut self, icon_animation: IconAnimationView) {}
}

/// Stub implementation, use it whenever callbacks are not needed.
impl IconAnimationCallbacks for () {}
