//! Furi API.

pub mod io;
pub mod sync;
pub mod message_queue;
pub mod thread;

use flipperzero_sys as sys;

/// Furi Result type.
pub type Result<T> = core::result::Result<T, Error>;
/// Furi Error type.
pub type Error = sys::furi::base::Status;
