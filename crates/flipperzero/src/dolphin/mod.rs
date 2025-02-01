//! Interact with your Dolphin!

use core::ffi::CStr;

use flipperzero_sys::{self as sys, furi::UnsafeRecord};

pub use sys::DolphinStats as Stats;

mod deed;
pub use deed::{App, Deed};

/// The dolphin in your FlipperZero!
#[derive(Clone)]
pub struct Dolphin {
    record: UnsafeRecord<sys::Dolphin>,
}

impl Dolphin {
    pub const NAME: &CStr = c"dolphin";

    /// Obtains a handle to the dolphin.
    pub fn open() -> Self {
        Self {
            record: unsafe { UnsafeRecord::open(Self::NAME) },
        }
    }

    /// Get pointer to raw [`sys::Dolphin`] record.
    ///
    /// This must not be `freed` or otherwise invalidated.
    /// It must not be referenced after [`Dolphin`] has been dropped.
    #[inline]
    pub fn as_ptr(&self) -> *mut sys::Dolphin {
        self.record.as_ptr()
    }

    /// Notifies the dolphin of deed completion.
    ///
    /// In future it will become part of assets. Thread safe, async.
    pub fn deed(&mut self, deed: Deed) {
        unsafe { sys::dolphin_deed(deed.to_raw()) };
    }

    /// Retrieves the dolphin's current stats.
    pub fn stats(&mut self) -> Stats {
        unsafe { sys::dolphin_stats(self.as_ptr()) }
    }

    /// Upgrades the level of the dolphin, if it is ready.
    ///
    /// Returns `true` if the dolphin's level was upgraded, or `false` if it was not ready.
    pub fn upgrade_level(&mut self) -> bool {
        let ready = self.stats().level_up_is_pending;
        if ready {
            unsafe { sys::dolphin_upgrade_level(self.as_ptr()) };
        }

        ready
    }

    /// Flushes dolphin queue and saves state.
    ///
    /// Thread safe, blocking.
    pub fn flush(&mut self) {
        unsafe { sys::dolphin_flush(self.as_ptr()) };
    }
}
