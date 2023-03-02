//! Furi Logging system.

mod metadata;
pub use metadata::{Level, LevelFilter};

/// The standard logging macro.
///
/// This macro will generically log with the specified `Level` and `format!` based
/// argument list.
///
/// # Examples
///
/// ```
/// use flipperzero::{error, furi::log::Level};
///
/// # fn main() {
/// let error_code = 42;
/// log!(Level::ERROR, "Failed to handle the florp: {}", error_code);
/// log!(target: "events", Level::INFO, "Finished the documentation!");
/// # }
/// ```
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $msg:expr $(, $arg:expr)*) => ({
        if $lvl <= $crate::furi::log::LevelFilter::current() {
            let mut buf = $crate::__macro_support::FuriString::new();
            $crate::__macro_support::ufmt::uwrite!(&mut buf, $msg $(, $arg)*)
                .expect("can append to FuriString");
            unsafe {
                $crate::__macro_support::__sys::furi_log_print_format(
                    $crate::__macro_support::__level_to_furi($lvl),
                    $crate::__macro_support::__sys::c_string!($target),
                    buf.as_c_str().as_ptr(),
                );
            }
        }
    });

    ($lvl:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: module_path!(), $lvl, $msg $(, $arg)*)
    );
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```
/// use flipperzero::error;
///
/// # fn main() {
/// let error_code = 42;
/// let name = "Flipper";
///
/// error!("Failed to handle the florp: {}", error_code);
/// error!(target: "events", "Missed birthday party for {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::ERROR, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::ERROR, $msg $(, $arg)*)
    );
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```
/// use flipperzero::warn;
///
/// # fn main() {
/// let name = "Flipper";
///
/// warn!("Event almost started!");
/// warn!(target: "events", "About to miss the birthday party for {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::WARN, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::WARN, $msg $(, $arg)*)
    );
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```
/// use flipperzero::info;
///
/// # fn main() {
/// let name = "Flipper";
///
/// info!("It's {}'s birthday today!", name);
/// info!(target: "events", "Birthday party today: {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::INFO, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::INFO, $msg $(, $arg)*)
    );
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```
/// use flipperzero::debug;
///
/// # fn main() {
/// let name = "Flipper";
///
/// debug!("Creating {} event", 1);
/// debug!(target: "events", "New event created: birthday party for {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::DEBUG, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::DEBUG, $msg $(, $arg)*)
    );
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```
/// use flipperzero::trace;
///
/// # fn main() {
/// let name = "Flipper";
///
/// trace!("About to show how the target field works");
/// trace!(target: "events", "Scanning for events involving {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::TRACE, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::TRACE, $msg $(, $arg)*)
    );
}
