//! ViewPort APIs

#[cfg(feature = "alloc")]
pub use self::alloc_features::*;

use core::mem::size_of_val;
use core::{
    ffi::c_void,
    mem::size_of,
    num::NonZeroU8,
    ptr::{null_mut, NonNull},
};
use flipperzero_sys::{
    self as sys, ViewPort as SysViewPort, ViewPortOrientation as SysViewPortOrientation,
};

/// System ViewPort.
pub struct ViewPort {
    raw: *mut SysViewPort,
    #[cfg(feature = "alloc")]
    draw_callback: Option<PinnedViewPortDrawCallback>,
}

impl ViewPort {
    /// Creates a new `ViewPort`.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// ```
    pub fn new() -> Self {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM
        let raw = unsafe { sys::view_port_alloc() };

        Self {
            raw,
            draw_callback: None,
        }
    }

    /// Construct a `ViewPort` from a raw non-null pointer.
    ///
    /// After calling this function, the raw pointer is owned by the resulting `ViewPort`.
    /// Specifically, the `ViewPort` destructor will free the allocated memory.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid pointer to [`SysViewPort`].
    ///
    /// # Examples
    ///
    /// Recreate a `ViewPort`
    /// which vas previously converted to a raw pointer using [`ViewPort::into_raw`].
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// let (raw, draw_callback) = view_port.into_raw();
    /// let view_port = unsafe { ViewPort::from_raw(raw, draw_callback) };
    /// ```
    pub unsafe fn from_raw(
        raw: NonNull<SysViewPort>,
        draw_callback: Option<PinnedViewPortDrawCallback>,
    ) -> Self {
        Self {
            raw: raw.as_ptr(),
            #[cfg(feature = "alloc")]
            draw_callback,
        }
    }

    /// Consumes this wrapper, returning a non-null raw pointer.
    ///
    /// After calling this function, the caller is responsible
    /// for the memory previously managed by the `ViewPort`.
    /// In particular, the caller should properly destroy `SysViewPort` and release the memory
    /// such as by calling [`sys::view_port_free`].
    /// The easiest way to do this is to convert the raw pointer
    /// back into a `ViewPort` with the [ViewPort::from_raw] function,
    /// allowing the `ViewPort` destructor to perform the cleanup.
    ///
    /// # Example
    ///
    /// Converting the raw pointer back into a `ViewPort`
    /// with [`ViewPort::from_raw`] for automatic cleanup:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// let (raw, draw_callback) = view_port.into_raw();
    /// let view_port = unsafe { ViewPort::from_raw(raw, draw_callback) };
    /// ```
    pub fn into_raw(mut self) -> (NonNull<SysViewPort>, Option<PinnedViewPortDrawCallback>) {
        let raw_pointer = core::mem::replace(&mut self.raw, null_mut());
        (
            // SAFETY: `self.raw` is guaranteed to be non-null
            // since it only becomes null after call to this function
            // which consumes the wrapper
            unsafe { NonNull::new_unchecked(raw_pointer) },
            self.draw_callback.take(),
        )
    }

    /// Creates a copy of the non-null raw pointer to the [`SysViewPort`].
    ///
    /// # Safety
    ///
    /// Caller must ensure that the provided pointer does not outlive this wrapper.
    pub unsafe fn as_raw(&self) -> NonNull<SysViewPort> {
        // SAFETY: the pointer is guaranteed to be non-null
        unsafe { NonNull::new_unchecked(self.raw) }
    }

    /// Sets the width of this `ViewPort`.
    /// Empty `width` means automatic.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::num::NonZeroU8;
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// view_port.set_width(NonZeroU8::new(128u8));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected width:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// view_port.set_width(None);
    /// ```
    pub fn set_width(&mut self, width: Option<NonZeroU8>) {
        let width = width.map_or(0u8, NonZeroU8::into);
        // SAFETY: `self.raw` is always valid
        // and there are no `width` constraints
        unsafe { sys::view_port_set_width(self.raw, width) }
    }

    /// Gets the width of this `ViewPort`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// let width = view_port.get_width();
    /// ```
    pub fn get_width(&self) -> NonZeroU8 {
        // SAFETY: `self.raw` is always valid
        unsafe { sys::view_port_get_width(self.raw) }
            .try_into()
            .expect("`view_port_get_width` should produce a positive value")
    }

    /// Sets the height of this `ViewPort`.
    /// Empty `height` means automatic.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::num::NonZeroU8;
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// view_port.set_height(NonZeroU8::new(128u8));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected height:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// view_port.set_height(None);
    /// ```
    pub fn set_height(&mut self, height: Option<NonZeroU8>) {
        let height = height.map_or(0u8, NonZeroU8::into);
        // SAFETY: `self.raw` is always valid
        // and there are no `height` constraints
        unsafe { sys::view_port_set_height(self.raw, height) }
    }

    /// Gets the height of this `ViewPort`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// let height = view_port.get_height();
    /// ```
    pub fn get_height(&self) -> NonZeroU8 {
        // SAFETY: `self.raw` is always valid
        unsafe { sys::view_port_get_height(self.raw) }
            .try_into()
            .expect("`view_port_get_height` should produce a positive value")
    }

    /// Sets the dimensions of this `ViewPort`.
    /// Empty `dimensions` means automatic.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::num::NonZeroU8;
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// view_port.set_dimensions(Some((NonZeroU8::new(120).unwrap(), NonZeroU8::new(80).unwrap())));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected dimensions:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// view_port.set_dimensions(None);
    /// ```
    pub fn set_dimensions(&mut self, dimensions: Option<(NonZeroU8, NonZeroU8)>) {
        match dimensions {
            Some((width, height)) => {
                self.set_width(Some(width));
                self.set_height(Some(height));
            }
            None => {
                self.set_width(None);
                self.set_height(None);
            }
        }
    }

    /// Gets the dimensions of this `ViewPort`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// let (width, height) = view_port.get_dimensions();
    /// ```
    pub fn get_dimensions(&self) -> (NonZeroU8, NonZeroU8) {
        (self.get_width(), self.get_height())
    }

    /// Sets the orientation of this `ViewPort`.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::{ViewPort, ViewPortOrientation};
    /// let mut view_port = ViewPort::new();
    /// view_port.set_orientation(ViewPortOrientation::Vertical);
    /// ```
    pub fn set_orientation(&mut self, orientation: ViewPortOrientation) {
        let orientation = SysViewPortOrientation::from(orientation);

        // SAFETY: `self.raw` is always valid
        // and `orientation` is guaranteed to be valid by `From` implementation
        unsafe { sys::view_port_set_orientation(self.raw, orientation) }
    }

    /// Gets the orientation of this `ViewPort`.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::num::NonZeroU8;
    /// use flipperzero_gui::view_port::{ViewPort, ViewPortOrientation};
    ///
    /// let mut view_port = ViewPort::new();
    /// let orientation = view_port.get_orientation();
    /// ```
    pub fn get_orientation(&self) -> ViewPortOrientation {
        // SAFETY: `self.raw` is always valid
        unsafe { sys::view_port_get_orientation(self.raw as *const _) }
            .try_into()
            .expect("`view_port_get_orientation` should produce a valid `ViewPort`")
    }

    /// Enables or disables this `ViewPort` rendering.
    ///
    /// `ViewPort` is enabled by default.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// view_port.set_enabled(false);
    /// ```
    pub fn set_enabled(&mut self, enabled: bool) {
        // SAFETY: `self.raw` is always valid
        unsafe { sys::view_port_enabled_set(self.raw, enabled) }
    }

    /// Checks if this `ViewPort` is enabled.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// let enabled = view_port.is_enabled();
    /// ```
    pub fn is_enabled(&self) -> bool {
        // SAFETY: `self.raw` is always valid
        unsafe { sys::view_port_is_enabled(self.raw) }
    }
    //
    // pub fn set_static_draw_callback<'a: 'b, 'b, F: Fn(*mut sys::Canvas)>(
    //     &'a mut self,
    //     callback: &'static fn(*mut sys::Canvas),
    // ) {
    //     pub unsafe extern "C" fn dispatch<F: Fn(*mut sys::Canvas)>(
    //         canvas: *mut sys::Canvas,
    //         context: *mut core::ffi::c_void,
    //     ) {
    //         // SAFETY: `context` is always a valid pointer
    //         let context = NonNull::new_unchecked(context as *const F as *mut F);
    //         // SAFETY: context is a valid pointer
    //         (unsafe { context.as_ref() })(canvas);
    //     }
    //     // FIXME: flipperzero-firmware: function pointer should be const
    //     let ptr = callback.as_ref().get_ref() as *const F as *mut c_void;
    //
    //     unsafe { sys::view_port_draw_callback_set(self.raw, Some(dispatch::<F>), ptr) }
    // }
}

impl Default for ViewPort {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ViewPort {
    fn drop(&mut self) {
        // `self.raw` is `null` iff it has been taken by call to `into_raw()`
        if !self.raw.is_null() {
            // FIXME: unregister from system
            // SAFETY: `self.raw` is always valid
            // and it should have been unregistered from the system by now
            unsafe { sys::view_port_free(self.raw) }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ViewPortOrientation {
    Horizontal,
    HorizontalFlip,
    Vertical,
    VerticalFlip,
}

#[derive(Clone, Copy, Debug)]
pub enum FromSysViewPortOrientationError {
    Max,
    Invalid(SysViewPortOrientation),
}

impl TryFrom<SysViewPortOrientation> for ViewPortOrientation {
    type Error = FromSysViewPortOrientationError;

    fn try_from(value: SysViewPortOrientation) -> Result<Self, Self::Error> {
        use sys::{
            ViewPortOrientation_ViewPortOrientationHorizontal as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL,
            ViewPortOrientation_ViewPortOrientationHorizontalFlip as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP,
            ViewPortOrientation_ViewPortOrientationMAX as SYS_VIEW_PORT_ORIENTATION_MAX,
            ViewPortOrientation_ViewPortOrientationVertical as SYS_VIEW_PORT_ORIENTATION_VERTICAL,
            ViewPortOrientation_ViewPortOrientationVerticalFlip as SYS_VIEW_PORT_ORIENTATION_VERTICAL_FLIP,
        };

        Ok(match value {
            SYS_VIEW_PORT_ORIENTATION_HORIZONTAL => Self::Horizontal,
            SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP => Self::HorizontalFlip,
            SYS_VIEW_PORT_ORIENTATION_VERTICAL => Self::Vertical,
            SYS_VIEW_PORT_ORIENTATION_VERTICAL_FLIP => Self::VerticalFlip,
            SYS_VIEW_PORT_ORIENTATION_MAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<ViewPortOrientation> for SysViewPortOrientation {
    fn from(value: ViewPortOrientation) -> Self {
        use sys::{
            ViewPortOrientation_ViewPortOrientationHorizontal as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL,
            ViewPortOrientation_ViewPortOrientationHorizontalFlip as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP,
            ViewPortOrientation_ViewPortOrientationVertical as SYS_VIEW_PORT_ORIENTATION_VERTICAL,
            ViewPortOrientation_ViewPortOrientationVerticalFlip as SYS_VIEW_PORT_ORIENTATION_VERTICAL_FLIP,
        };

        match value {
            ViewPortOrientation::Horizontal => SYS_VIEW_PORT_ORIENTATION_HORIZONTAL,
            ViewPortOrientation::HorizontalFlip => SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP,
            ViewPortOrientation::Vertical => SYS_VIEW_PORT_ORIENTATION_VERTICAL,
            ViewPortOrientation::VerticalFlip => SYS_VIEW_PORT_ORIENTATION_VERTICAL_FLIP,
        }
    }
}

/// Functionality only available when `alloc` feature is enabled.
#[cfg(feature = "alloc")]
mod alloc_features {
    use super::*;
    use alloc::boxed::Box;
    use core::pin::Pin;

    impl ViewPort {
        pub fn set_draw_callback(&mut self, mut callback: ViewPortDrawCallback) {
            type CallbackPtr = *mut ViewPortDrawCallback;

            pub unsafe extern "C" fn dispatch(canvas: *mut sys::Canvas, context: *mut c_void) {
                let context: CallbackPtr = context.cast();
                // SAFETY: `context` is stored in a pinned Box which is a member of `ViewPort`
                // and the callback is accessed exclusively by this function
                (unsafe { NonNull::new_unchecked(context).as_mut() }).0(canvas);
            }

            let mut callback = Box::pin(callback);
            let ptr = callback.as_mut().get_mut() as CallbackPtr as *mut c_void;
            // keep old cllback alive until the new one is written
            let old_callback = self.draw_callback.replace(callback);

            const _: () = assert!(
                size_of::<*const ViewPortDrawCallback>() == size_of::<*mut c_void>(),
                "`ViewPortDrawCallback` should be a thin pointer"
            );

            unsafe { sys::view_port_draw_callback_set(self.raw, Some(dispatch), ptr) }

            drop(old_callback);
        }
    }

    pub struct ViewPortDrawCallback(Box<dyn FnMut(*mut sys::Canvas)>);

    impl ViewPortDrawCallback {
        pub fn new(callback: Box<dyn FnMut(*mut sys::Canvas)>) -> Self {
            Self(callback)
        }

        pub fn boxed<F: FnMut(*mut sys::Canvas) + 'static>(callback: F) -> Self {
            Self::new(Box::new(callback))
        }
    }

    pub(super) type PinnedViewPortDrawCallback = Pin<Box<ViewPortDrawCallback>>;
}
