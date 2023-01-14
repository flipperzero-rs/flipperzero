use crate::gui::icon::Icon;
use alloc::boxed::Box;
use core::{
    ffi::c_void,
    marker::PhantomData,
    ptr::{self, NonNull},
};
use flipperzero_sys::{self as sys, IconAnimation as SysIconAnimation};

/// System Icon Animation wrapper.
pub struct IconAnimation<'a, C: IconAnimationCallbacks> {
    raw: NonNull<SysIconAnimation>,
    callbacks: NonNull<C>,
    _parent_lifetime: PhantomData<&'a ()>,
}

impl<'a, C: IconAnimationCallbacks> IconAnimation<'a, C> {
    pub fn new<'b: 'a>(icon: &'b Icon, callbacks: C) -> Self {
        let icon = icon.as_raw();
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM,
        // `icon` is a valid pointer and `icon` outlives this animation
        let raw = unsafe { NonNull::new_unchecked(sys::icon_animation_alloc(icon)) };
        let callbacks = NonNull::from(Box::leak(Box::new(callbacks)));

        let icon_animation = Self {
            raw,
            callbacks,
            _parent_lifetime: PhantomData,
        };

        pub unsafe extern "C" fn dispatch_update<C: IconAnimationCallbacks>(
            instance: *mut SysIconAnimation,
            context: *mut c_void,
        ) {
            // SAFETY: `icon_anination` is guaranteed to be a valid pointer
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
            let context = icon_animation.callbacks.as_ptr().cast();
            let raw = raw.as_ptr();
            // SAFETY: `raw` is valid
            // and `callbacks` is valid and lives with this struct
            unsafe {
                sys::icon_animation_set_update_callback(raw, Some(dispatch_update::<C>), context)
            }
        }

        icon_animation
    }

    pub fn as_raw(&self) -> *mut SysIconAnimation {
        self.raw.as_ptr()
    }

    pub fn get_width(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_get_width(raw) }
    }

    pub fn get_height(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_get_height(raw) }
    }

    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.get_width(), self.get_height())
    }

    pub fn start(&mut self) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_start(raw) }
    }

    pub fn stop(&mut self) {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_stop(raw) }
    }

    pub fn is_last_frame(&self) -> bool {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_is_last_frame(raw) }
    }
}

impl<C: IconAnimationCallbacks> Drop for IconAnimation<'_, C> {
    fn drop(&mut self) {
        let raw = self.raw.as_ptr();
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
    _lifetime: PhantomData<&'a ()>,
}

impl IconAnimationView<'_> {
    /// Construct an `IconAnimationView` from a raw pointer.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid non-null pointer to [`SysCanvas`]
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
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_get_width(raw) }
    }

    pub fn get_height(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_get_height(raw) }
    }

    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.get_width(), self.get_height())
    }

    // TODO: decide if these methods should be available in view,
    //  i.e. if it is sound to call start/stop from callbacks
    // pub fn start(&mut self) {
    //     let raw = self.raw.as_ptr();
    //     // SAFETY: `raw` is always valid
    //     unsafe { sys::icon_animation_start(raw) }
    // }
    //
    // pub fn stop(&mut self) {
    //     let raw = self.raw.as_ptr();
    //     // SAFETY: `raw` is always valid
    //     unsafe { sys::icon_animation_stop(raw) }
    // }

    pub fn is_last_frame(&self) -> bool {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_animation_is_last_frame(raw) }
    }
}

pub trait IconAnimationCallbacks {
    fn on_update(&mut self, _icon_animation: IconAnimationView) {}
}

impl IconAnimationCallbacks for () {}
