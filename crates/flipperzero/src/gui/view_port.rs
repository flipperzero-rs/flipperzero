//! ViewPort APIs

use crate::{gui::canvas::CanvasView, input::InputEvent};
use alloc::boxed::Box;
use core::{
    ffi::c_void,
    num::NonZeroU8,
    ptr::{self, NonNull},
};
use flipperzero_sys::{
    self as sys, Canvas as SysCanvas, ViewPort as SysViewPort,
    ViewPortOrientation as SysViewPortOrientation,
};

/// System ViewPort.
pub struct ViewPort<C: ViewPortCallbacks> {
    raw: NonNull<SysViewPort>,
    callbacks: Box<C>,
}

impl<C: ViewPortCallbacks> ViewPort<C> {
    /// Creates a new `ViewPort`.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new(());
    /// ```
    pub fn new(callbacks: C) -> Self {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM
        let raw = unsafe { NonNull::new_unchecked(sys::view_port_alloc()) };
        let callbacks = Box::new(callbacks);

        let view_port = Self { raw, callbacks };

        {
            pub unsafe extern "C" fn dispatch_draw<C: ViewPortCallbacks>(
                canvas: *mut SysCanvas,
                context: *mut c_void,
            ) {
                // SAFETY: `canvas` is guaranteed to be a valid pointer
                let canvas = unsafe { CanvasView::from_raw(canvas) };

                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `ViewPort`
                // and the callback is accessed exclusively by this function
                unsafe { &mut *context }.on_draw(canvas);
            }

            if !ptr::eq(
                C::on_draw as *const c_void,
                <() as ViewPortCallbacks>::on_draw as *const c_void,
            ) {
                let context = (&*view_port.callbacks as *const C).cast_mut().cast();
                let raw = raw.as_ptr();
                let callback = Some(dispatch_draw::<C> as _);
                // SAFETY: `raw` is valid
                // and `callbacks` is valid and lives with this struct
                unsafe { sys::view_port_draw_callback_set(raw, callback, context) };
            }
        }
        {
            pub unsafe extern "C" fn dispatch_input<C: ViewPortCallbacks>(
                input_event: *mut sys::InputEvent,
                context: *mut c_void,
            ) {
                let input_event: InputEvent = (&unsafe { *input_event })
                    .try_into()
                    .expect("`input_event` should be a valid event");

                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a pinned Box which is a member of `ViewPort`
                // and the callback is accessed exclusively by this function
                unsafe { &mut *context }.on_input(input_event);
            }

            if !ptr::eq(
                C::on_input as *const c_void,
                <() as ViewPortCallbacks>::on_input as *const c_void,
            ) {
                let context = (&*view_port.callbacks as *const C).cast_mut().cast();
                let raw = raw.as_ptr();
                let callback = Some(dispatch_input::<C> as _);

                // SAFETY: `raw` is valid
                // and `callbacks` is valid and lives with this struct
                unsafe { sys::view_port_input_callback_set(raw, callback, context) };
            }
        }

        view_port
    }

    /// Creates a copy of the raw pointer to the [`sys::ViewPort`].
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
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
    /// view_port.set_width(NonZeroU8::new(128u8));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected width:
    ///
    /// ```
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
    /// view_port.set_height(NonZeroU8::new(128u8));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected height:
    ///
    /// ```
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
    /// view_port.set_dimensions(Some((NonZeroU8::new(120).unwrap(), NonZeroU8::new(80).unwrap())));
    /// ```
    ///
    /// Resize `ViewPort` to automatically selected dimensions:
    ///
    /// ```
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
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

    pub fn update(&mut self) {
        let raw = self.as_raw();
        // SAFETY: `raw` is always valid
        unsafe { sys::view_port_update(raw) }
    }

    /// Gets the dimensions of this `ViewPort`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::{ViewPort, ViewPortOrientation};
    /// let mut view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::{ViewPort, ViewPortOrientation};
    ///
    /// let mut view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
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
    /// use flipperzero::gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new(());
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
        unsafe {
            sys::view_port_enabled_set(raw, false);
            sys::view_port_free(raw);
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ViewPortOrientation {
    Horizontal,
    HorizontalFlip,
    Vertical,
    VerticalFlip,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysViewPortOrientationError {
    Max,
    Invalid(SysViewPortOrientation),
}

impl TryFrom<SysViewPortOrientation> for ViewPortOrientation {
    type Error = FromSysViewPortOrientationError;

    fn try_from(value: SysViewPortOrientation) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::ViewPortOrientation_ViewPortOrientationHorizontal => Self::Horizontal,
            sys::ViewPortOrientation_ViewPortOrientationHorizontalFlip => Self::HorizontalFlip,
            sys::ViewPortOrientation_ViewPortOrientationVertical => Self::Vertical,
            sys::ViewPortOrientation_ViewPortOrientationVerticalFlip => Self::VerticalFlip,
            sys::ViewPortOrientation_ViewPortOrientationMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<ViewPortOrientation> for SysViewPortOrientation {
    fn from(value: ViewPortOrientation) -> Self {
        match value {
            ViewPortOrientation::Horizontal => {
                sys::ViewPortOrientation_ViewPortOrientationHorizontal
            }
            ViewPortOrientation::HorizontalFlip => {
                sys::ViewPortOrientation_ViewPortOrientationHorizontalFlip
            }
            ViewPortOrientation::Vertical => sys::ViewPortOrientation_ViewPortOrientationVertical,
            ViewPortOrientation::VerticalFlip => {
                sys::ViewPortOrientation_ViewPortOrientationVerticalFlip
            }
        }
    }
}

pub trait ViewPortCallbacks {
    fn on_draw(&mut self, _canvas: CanvasView) {}
    fn on_input(&mut self, _event: InputEvent) {}
}

impl ViewPortCallbacks for () {}