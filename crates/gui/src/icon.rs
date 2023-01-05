use crate::xbm::XbmImage;
use core::ptr::NonNull;
use flipperzero_sys::{self as sys, Icon as SysIcon};

pub struct Icon {
    raw: NonNull<SysIcon>,
}

impl Icon {
    /// Construct an `Icon` from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid pointer to [`SysCanvas`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::icon::Icon;
    ///
    /// let ptr = todo!();
    /// let canvas = unsafe { Icon::from_raw(ptr) };
    /// ```
    pub unsafe fn from_raw(raw: NonNull<SysIcon>) -> Self {
        Self { raw }
    }

    pub fn as_raw(&self) -> *mut SysIcon {
        self.raw.as_ptr()
    }

    pub fn get_width(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_get_width(raw) }
    }

    pub fn get_height(&self) -> u8 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_get_height(raw) }
    }

    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.get_width(), self.get_height())
    }

    pub fn get_data(&self) -> XbmImage<&'_ [u8]> {
        let (width, height) = self.get_dimensions();

        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        // and `width` and `height` are always in sync with data
        unsafe { XbmImage::from_raw(width, height, sys::icon_get_data(raw)) }
    }
}
