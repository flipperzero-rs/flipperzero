//! ViewPort APIs

use core::{
    num::NonZeroU8,
    ptr::{null_mut, NonNull},
};
use flipperzero_sys::{
    self as sys, ViewPort as SysViewPort, ViewPortOrientation as SysViewPortOrientation,
};

/// System ViewPort.
pub struct ViewPort {
    view_port: *mut SysViewPort,
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
    pub fn new() -> ViewPort {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM
        let view_port = unsafe { sys::view_port_alloc() };
        Self { view_port }
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
        // SAFETY: `self.view_port` is always valid
        // and there are no `width` constraints
        unsafe { sys::view_port_set_width(self.view_port, width) }
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
        // SAFETY: `self.view_port` is always valid
        unsafe { sys::view_port_get_width(self.view_port) }
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
        // SAFETY: `self.view_port` is always valid
        // and there are no `height` constraints
        unsafe { sys::view_port_set_height(self.view_port, height) }
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
        // SAFETY: `self.view_port` is always valid
        unsafe { sys::view_port_get_height(self.view_port) }
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

        // SAFETY: `self.view_port` is always valid
        // and `orientation` is guaranteed to be valid by `From` implementation
        unsafe { sys::view_port_set_orientation(self.view_port, orientation) }
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
        // SAFETY: `self.view_port` is always valid
        unsafe { sys::view_port_get_orientation(self.view_port) }
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
        // SAFETY: `self.view_port` is always valid
        unsafe { sys::view_port_enabled_set(self.view_port, enabled) }
    }

    /// Checks if this `ViewPort` is enabled.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let mut view_port = ViewPort::new();
    /// let enabled = view_port.is_enabled();
    /// ```
    pub fn is_enabled(&self) -> bool {
        // SAFETY: `self.view_port` is always valid
        unsafe { sys::view_port_is_enabled(self.view_port) }
    }

    /// Construct a `ViewPort` from a raw non-null pointer.
    ///
    /// After calling this function, the raw pointer is owned by the resulting `ViewPort`.
    /// Specifically, the `ViewPort` destructor wil free the allocated memory.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid pointer to [`SysViewPort`].
    ///
    /// # Examples
    ///
    /// Recreate a `ViewPort`
    /// which was preciously converted to a raw pointer using [`ViewPort::into_raw`].
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// let ptr = view_port.into_raw();
    /// let view_port = unsafe { ViewPort::from_raw(ptr) };
    /// ```
    pub unsafe fn from_raw(raw: NonNull<SysViewPort>) -> Self {
        Self {
            view_port: raw.as_ptr(),
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
    /// allowing the `ViewPort`` destructor to perform the cleanup.
    ///
    /// # Example
    ///
    /// Converting the raw pointer back into a `ViewPort`
    /// with `ViewPort::from_raw` for automatic cleanup:
    ///
    /// ```
    /// use flipperzero_gui::view_port::ViewPort;
    ///
    /// let view_port = ViewPort::new();
    /// let ptr = view_port.into_raw();
    /// let view_port = unsafe { ViewPort::from_raw(ptr) };
    /// ```
    pub fn into_raw(mut self) -> NonNull<SysViewPort> {
        let raw_pointer = core::mem::replace(&mut self.view_port, null_mut());
        // SAFETY: `self.view_port` is guaranteed to be non-null
        // since it only becomes null after call to this function
        // which consumes the wrapper
        unsafe { NonNull::new_unchecked(raw_pointer) }
    }
}

impl Default for ViewPort {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ViewPort {
    fn drop(&mut self) {
        // `self.view_port` is `null` iff it has been taken by call to `into_raw()`
        if !self.view_port.is_null() {
            // FIXME: unregister from system
            // SAFETY: `self.view_port` is always valid
            // and it should have been unregistered from the system by now
            unsafe { sys::view_port_free(self.view_port) }
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

    fn try_from(value: SysViewPortOrientation) -> Result<ViewPortOrientation, Self::Error> {
        use sys::{
            ViewPortOrientation_ViewPortOrientationHorizontal as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL,
            ViewPortOrientation_ViewPortOrientationHorizontalFlip as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP,
            ViewPortOrientation_ViewPortOrientationMAX as SYS_VIEW_PORT_ORIENTATION_MAX,
            ViewPortOrientation_ViewPortOrientationVertical as SYS_VIEW_PORT_ORIENTATION_VERTICAL,
            ViewPortOrientation_ViewPortOrientationVerticalFlip as SYS_VIEW_PORT_ORIENTATION_VERTICAL_FLIP,
        };

        match value {
            SYS_VIEW_PORT_ORIENTATION_HORIZONTAL => Ok(Self::Horizontal),
            SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP => Ok(Self::HorizontalFlip),
            SYS_VIEW_PORT_ORIENTATION_VERTICAL => Ok(Self::Vertical),
            SYS_VIEW_PORT_ORIENTATION_VERTICAL_FLIP => Ok(Self::VerticalFlip),
            SYS_VIEW_PORT_ORIENTATION_MAX => Err(Self::Error::Max),
            invalid => Err(Self::Error::Invalid(invalid)),
        }
    }
}

impl From<ViewPortOrientation> for SysViewPortOrientation {
    fn from(value: ViewPortOrientation) -> Self {
        use sys::{
            ViewPortOrientation_ViewPortOrientationHorizontal as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL,
            ViewPortOrientation_ViewPortOrientationHorizontalFlip as SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP,
            ViewPortOrientation_ViewPortOrientationMAX as SYS_VIEW_PORT_ORIENTATION_MAX,
            ViewPortOrientation_ViewPortOrientationVertical as SYS_VIEW_PORT_ORIENTATION_VERTICAL,
        };

        match value {
            ViewPortOrientation::Horizontal => SYS_VIEW_PORT_ORIENTATION_HORIZONTAL,
            ViewPortOrientation::HorizontalFlip => SYS_VIEW_PORT_ORIENTATION_HORIZONTAL_FLIP,
            ViewPortOrientation::Vertical => SYS_VIEW_PORT_ORIENTATION_MAX,
            ViewPortOrientation::VerticalFlip => SYS_VIEW_PORT_ORIENTATION_VERTICAL,
        }
    }
}
