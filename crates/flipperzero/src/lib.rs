//! High-level bindings for the Flipper Zero.
//!
//! # Features
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

#![no_std]
#![cfg_attr(test, no_main)]
#![cfg_attr(feature = "unstable_intrinsics", feature(int_roundings))]
#![cfg_attr(feature = "unstable_lints", feature(must_not_suspend))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(any(feature = "alloc", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
extern crate alloc;

#[cfg(feature = "service-dialogs")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-dialogs")))]
pub mod dialogs;
#[cfg(feature = "service-dolphin")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-dolphin")))]
pub mod dolphin;
pub mod furi;
pub mod gpio;
#[cfg(feature = "service-gui")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-gui")))]
pub mod gui;
#[cfg(feature = "service-input")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-input")))]
pub mod input;
pub(crate) mod internals;
pub mod io;
pub mod kernel;
pub mod macros;
#[cfg(feature = "service-notification")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-notification")))]
pub mod notification;
#[cfg(feature = "service-storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-storage")))]
pub mod storage;
pub mod toolbox;
#[cfg(feature = "xbm")]
#[cfg_attr(docsrs, doc(cfg(feature = "xbm")))]
pub mod xbm;

#[doc(hidden)]
pub mod __macro_support {
    use crate::furi::log::Level;

    // Re-export for use in macros
    pub use ufmt;

    pub use crate::furi::string::FuriString;

    /// ⚠️ WARNING: This is *not* a stable API! ⚠️
    ///
    /// This module, and all code contained in the `__macro_support` module, is a
    /// *private* API of `flipperzero`. It is exposed publicly because it is used by the
    /// `flipperzero` macros, but it is not part of the stable versioned API. Breaking
    /// changes to this module may occur in small-numbered versions without warning.
    pub use flipperzero_sys as __sys;

    /// ⚠️ WARNING: This is *not* a stable API! ⚠️
    ///
    /// This function, and all code contained in the `__macro_support` module, is a
    /// *private* API of `flipperzero`. It is exposed publicly because it is used by the
    /// `flipperzero` macros, but it is not part of the stable versioned API. Breaking
    /// changes to this module may occur in small-numbered versions without warning.
    pub fn __level_to_furi(level: Level) -> __sys::FuriLogLevel {
        level.to_furi()
    }
}

flipperzero_test::tests_runner!(
    name = "flipperzero-rs Unit Tests",
    stack_size = 4096,
    [
        crate::furi::log::metadata::tests,
        crate::furi::message_queue::tests,
        crate::furi::rng::tests,
        crate::furi::string::tests,
        crate::furi::sync::tests,
        crate::furi::time::tests,
        crate::gpio::i2c::tests,
        crate::toolbox::crc32::tests,
        crate::toolbox::md5::tests,
        crate::toolbox::sha256::tests,
        crate::xbm::tests,
    ]
);
