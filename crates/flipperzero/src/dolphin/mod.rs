//! Interact with your Dolphin!

use core::ffi::c_char;

use flipperzero_sys::{self as sys, furi::UnsafeRecord};

pub use sys::DolphinStats as Stats;

mod deed;
pub use deed::{App, Deed};

const RECORD_DOLPHIN: *const c_char = sys::c_string!("dolphin");

/// The dolphin in your FlipperZero!
pub struct Dolphin {
    data: UnsafeRecord<sys::Dolphin>,
}

impl Dolphin {
    /// Obtains a handle to the dolphin.
    pub fn open() -> Self {
        Self {
            data: unsafe { UnsafeRecord::open(RECORD_DOLPHIN) },
        }
    }

    /// Notifies the dolphin of deed completion.
    ///
    /// In future it will become part of assets. Thread safe, async.
    pub fn deed(&mut self, deed: Deed) {
        unsafe { sys::dolphin_deed(deed.to_raw()) };
    }

    /// Retrieves the dolphin's current stats.
    pub fn stats(&mut self) -> Stats {
        unsafe { sys::dolphin_stats(self.data.as_raw()) }
    }

    /// Upgrades the level of the dolphin, if it is ready.
    ///
    /// Returns `true` if the dolphin's level was upgraded, or `false` if it was not ready.
    pub fn upgrade_level(&mut self) -> bool {
        let ready = self.stats().level_up_is_pending;
        if ready {
            unsafe { sys::dolphin_upgrade_level(self.data.as_raw()) };
        }
        ready
    }

    /// Flushes dolphin queue and saves state.
    ///
    /// Thread safe, blocking.
    pub fn flush(&mut self) {
        unsafe { sys::dolphin_flush(self.data.as_raw()) };
    }
}
