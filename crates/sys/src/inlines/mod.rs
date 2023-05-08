//! Inline functions.
//!
//! Parts of the Flipper Zero C API are defined using `inline`.
//! This means that there is no implementation we can link to,
//! and thus need to provide one ourselves.

pub mod furi_hal_gpio;
