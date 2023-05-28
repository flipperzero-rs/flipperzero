//! Furi API.

pub mod io;
pub mod log;
pub mod message_queue;
pub mod rng;
pub mod string;
pub mod sync;
pub mod thread;
pub mod time;

use flipperzero_sys as sys;

/// Furi Result type.
pub type Result<T> = core::result::Result<T, Error>;
/// Furi Error type.
pub type Error = sys::furi::Status;
