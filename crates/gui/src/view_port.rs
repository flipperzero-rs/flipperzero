//! ViewPort APIs

use alloc::boxed::Box;
use core::{ffi::c_void, num::NonZeroU8, ptr::NonNull};
use flipperzero_sys::{
    self as sys, ViewPort as SysViewPort, ViewPortOrientation as SysViewPortOrientation,
};

/// System ViewPort.
pub struct ViewPort<C: ViewPortCallbacks> {
    raw: NonNull<SysViewPort>,
    callbacks: NonNull<C>,
}

impl<C: ViewPortCallbacks> ViewPort<C> {
    /// Creates a new `ViewPort`.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new(todo!());
    /// ```
    pub fn new(callbacks: C) -> Self {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM
        let raw = unsafe { NonNull::new_unchecked(sys::view_port_alloc()) };
        let callbacks = NonNull::from(Box::leak(Box::new(callbacks)));

        let mut view_port = Self { raw, callbacks };

        pub unsafe extern "C" fn dispatch_draw<C: ViewPortCallbacks>(
            canvas: *mut sys::Canvas,
            context: *mut c_void,
        ) {
            let context: *mut C = context.cast();
            // SAFETY: `context` is stored in a `Box` which is a member of `ViewPort`
            // and the callback is accessed exclusively by this function
            (unsafe { &mut *context }).on_draw(canvas);
        }
        pub unsafe extern "C" fn dispatch_input<C: ViewPortCallbacks>(
            canvas: *mut sys::InputEvent,
            context: *mut c_void,
        ) {
            let context: *mut C = context.cast();
            // SAFETY: `context` is stored in a pinned Box which is a member of `ViewPort`
            // and the callback is accessed exclusively by this function
            (unsafe { &mut *context }).on_input(canvas);
        }

        // SAFETY: `callbacks` is a valid pointer and
        let context = unsafe { view_port.callbacks.as_ptr() }.cast();

        let raw = raw.as_ptr();
        unsafe {
            sys::view_port_draw_callback_set(raw, Some(dispatch_draw::<C>), context);
            sys::view_port_input_callback_set(raw, Some(dispatch_input::<C>), context);
        };

        view_port
    }

    /// Creates a copy of the raw pointer to the [`SysViewPort`].
    pub fn as_raw(&self) -> *mut SysViewPort {
        self.raw.as_ptr()
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
    /// let mut view_port = ViewPort::new(todo!());
    /// view_port.set_width(NonZeroU8::new(128u8));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected width:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(todo!());
    /// view_port.set_width(None);
    /// ```
    pub fn set_width(&mut self, width: Option<NonZeroU8>) {
        let width = width.map_or(0u8, NonZeroU8::into);
        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        // and there are no `width` constraints
        unsafe { sys::view_port_set_width(raw, width) }
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
    /// let view_port = ViewPort::new(todo!());
    /// let width = view_port.get_width();
    /// ```
    pub fn get_width(&self) -> Option<NonZeroU8> {
        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        NonZeroU8::new(unsafe { sys::view_port_get_width(raw) })
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
    /// let mut view_port = ViewPort::new(todo!());
    /// view_port.set_height(NonZeroU8::new(128u8));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected height:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(todo!());
    /// view_port.set_height(None);
    /// ```
    pub fn set_height(&mut self, height: Option<NonZeroU8>) {
        let height = height.map_or(0u8, NonZeroU8::into);
        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        // and there are no `height` constraints
        unsafe { sys::view_port_set_height(raw, height) }
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
    /// let view_port = ViewPort::new(todo!());
    /// let height = view_port.get_height();
    /// ```
    pub fn get_height(&self) -> Option<NonZeroU8> {
        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        NonZeroU8::new(unsafe { sys::view_port_get_height(raw) })
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
    /// let mut view_port = ViewPort::new(todo!());
    /// view_port.set_dimensions(Some((NonZeroU8::new(120).unwrap(), NonZeroU8::new(80).unwrap())));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected dimensions:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(todo!());
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
    /// let view_port = ViewPort::new(todo!());
    /// let (width, height) = view_port.get_dimensions();
    /// ```
    pub fn get_dimensions(&self) -> (Option<NonZeroU8>, Option<NonZeroU8>) {
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
    /// let mut view_port = ViewPort::new(todo!());
    /// view_port.set_orientation(ViewPortOrientation::Vertical);
    /// ```
    pub fn set_orientation(&mut self, orientation: ViewPortOrientation) {
        let orientation = SysViewPortOrientation::from(orientation);

        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        // and `orientation` is guaranteed to be valid by `From` implementation
        unsafe { sys::view_port_set_orientation(raw, orientation) }
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
    /// let mut view_port = ViewPort::new(todo!());
    /// let orientation = view_port.get_orientation();
    /// ```
    pub fn get_orientation(&self) -> ViewPortOrientation {
        let raw = self.as_raw().cast_const();
        // SAFETY: `raw` is always valid
        unsafe { sys::view_port_get_orientation(raw) }
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
    /// let mut view_port = ViewPort::new(todo!());
    /// view_port.set_enabled(false);
    /// ```
    pub fn set_enabled(&mut self, enabled: bool) {
        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        unsafe { sys::view_port_enabled_set(raw, enabled) }
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
    /// let mut view_port = ViewPort::new(todo!());
    /// let enabled = view_port.is_enabled();
    /// ```
    pub fn is_enabled(&self) -> bool {
        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        unsafe { sys::view_port_is_enabled(raw) }
    }
}

impl<C: ViewPortCallbacks> Drop for ViewPort<C> {
    fn drop(&mut self) {
        // FIXME: unregister from system (whatever this means)

        let raw = self.raw.as_ptr();
        // SAFETY: `self.raw` is always valid
        // and it should have been unregistered from the system by now
        unsafe { sys::view_port_free(raw) }

        let callbacks = self.callbacks.as_ptr();
        // SAFETY: `callbacks` was created using `Box::into_raw()` on `ViewPort` creation
        let _ = unsafe { Box::from_raw(callbacks) };
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

pub trait ViewPortCallbacks {
    fn on_draw(&mut self, _canvas: *mut sys::Canvas) {}
    fn on_input(&mut self, _event: *mut sys::InputEvent) {}
}
