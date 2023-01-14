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
    /// use flipperzero::gui::view::View;
    ///
    /// let view = View::new();
    /// ```
    pub fn new() -> View {
        // SAFETY: allocation either succeeds producing the valid pointer
        // or stops the system on OOM
        let view = unsafe { sys::view_alloc() };
        Self { raw: view }
    }

    /// Creates a copy of raw pointer to the [`SysView`].
    pub unsafe fn as_raw(&self) -> *mut SysView {
        self.raw
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
