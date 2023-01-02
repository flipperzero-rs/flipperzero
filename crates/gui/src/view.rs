use core::ptr::{null_mut, NonNull};
use flipperzero_sys::{self as sys, View as SysView};

pub struct View {
    raw: *mut SysView,
}

impl View {
    /// Creates a new `View`.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero_gui::view::View;
    ///
    /// let view = View::new();
    /// ```
    pub fn new() -> View {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM
        let view = unsafe { sys::view_alloc() };
        Self { raw: view }
    }

    /// Construct a `View` from a raw non-null pointer.
    ///
    /// After calling this function, the raw pointer is owned by the resulting `View`.
    /// Specifically, the `View` destructor will free the allocated memory.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid pointer to [`SysView`].
    ///
    /// # Examples
    ///
    /// Recreate a `View`
    /// which vas previously converted to a raw pointer using [`View::into_raw`].
    ///
    /// ```
    /// use flipperzero_gui::view::View;
    ///
    /// let view = View::new();
    /// let ptr = view.into_raw();
    /// let view = unsafe { View::from_raw(ptr) };
    /// ```
    pub unsafe fn from_raw(raw: NonNull<SysView>) -> Self {
        Self { raw: raw.as_ptr() }
    }

    /// Consumes this wrapper, returning a non-null raw pointer.
    ///
    /// After calling this function, the caller is responsible
    /// for the memory previously managed by the `View`.
    /// In particular, the caller should properly destroy `SysView` and release the memory
    /// such as by calling [`sys::view_free`].
    /// The easiest way to do this is to convert the raw pointer
    /// back into a `View` with the [View::from_raw] function,
    /// allowing the `View` destructor to perform the cleanup.
    ///
    /// # Example
    ///
    /// Converting the raw pointer back into a `ViewPort`
    /// with [`View::from_raw`] for automatic cleanup:
    ///
    /// ```
    /// use flipperzero_gui::view::View;
    ///
    /// let view = View::new();
    /// let ptr = view.into_raw();
    /// let view = unsafe { View::from_raw(ptr) };
    /// ```
    pub fn into_raw(mut self) -> NonNull<SysView> {
        let raw_pointer = core::mem::replace(&mut self.raw, null_mut());
        // SAFETY: `self.raw` is guaranteed to be non-null
        // since it only becomes null after call to this function
        // which consumes the wrapper
        unsafe { NonNull::new_unchecked(raw_pointer) }
    }

    /// Creates a copy of the non-null raw pointer to the [`SysView`].
    ///
    /// # Safety
    ///
    /// Caller must ensure that the provided pointer does not outlive this wrapper.
    pub unsafe fn as_raw(&self) -> NonNull<SysView> {
        // SAFETY: the pointer is guaranteed to be non-null
        unsafe { NonNull::new_unchecked(self.raw) }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for View {
    fn drop(&mut self) {
        // `self.raw` is `null` iff it has been taken by call to `into_raw()`
        if !self.raw.is_null() {
            // SAFETY: `self.raw` is always valid
            // and it should have been unregistered from the system by now
            unsafe { sys::view_free(self.raw) }
        }
    }
}
