//! Metadata describing log data.
//
// The structs and enums in this file are extracted from the `tracing-core` crate with
// adaptions to Furi. The original code is copyright (c) 2019 Tokio Contributors

use core::{cmp, fmt, str::FromStr};

use flipperzero_sys as sys;

/// Describes the level of verbosity of a span or event.
///
/// # Comparing Levels
///
/// `Level` implements the [`PartialOrd`] and [`Ord`] traits, allowing two
/// `Level`s to be compared to determine which is considered more or less
/// verbose. Levels which are more verbose are considered "greater than" levels
/// which are less verbose, with [`Level::ERROR`] considered the lowest, and
/// [`Level::TRACE`] considered the highest.
///
/// For example:
/// ```
/// use tracing_core::Level;
///
/// assert!(Level::TRACE > Level::DEBUG);
/// assert!(Level::ERROR < Level::WARN);
/// assert!(Level::INFO <= Level::DEBUG);
/// assert_eq!(Level::TRACE, Level::TRACE);
/// ```
///
/// # Filtering
///
/// `Level`s are typically used to implement filtering that determines which
/// spans and events are enabled. Depending on the use case, more or less
/// verbose diagnostics may be desired. For example, when running in
/// development, [`DEBUG`]-level traces may be enabled by default. When running in
/// production, only [`INFO`]-level and lower traces might be enabled. Libraries
/// may include very verbose diagnostics at the [`DEBUG`] and/or [`TRACE`] levels.
/// Applications using those libraries typically chose to ignore those traces. However, when
/// debugging an issue involving said libraries, it may be useful to temporarily
/// enable the more verbose traces.
///
/// The [`LevelFilter`] type is provided to enable filtering traces by
/// verbosity. `Level`s can be compared against [`LevelFilter`]s, and
/// [`LevelFilter`] has a variant for each `Level`, which compares analogously
/// to that level. In addition, [`LevelFilter`] adds a [`LevelFilter::OFF`]
/// variant, which is considered "less verbose" than every other `Level`. This is
/// intended to allow filters to completely disable tracing in a particular context.
///
/// For example:
/// ```
/// use tracing_core::{Level, LevelFilter};
///
/// assert!(LevelFilter::OFF < Level::TRACE);
/// assert!(LevelFilter::TRACE > Level::DEBUG);
/// assert!(LevelFilter::ERROR < Level::WARN);
/// assert!(LevelFilter::INFO <= Level::DEBUG);
/// assert!(LevelFilter::INFO >= Level::INFO);
/// ```
///
/// ## Examples
///
/// Below is a simple example of how a [collector] could implement filtering through
/// a [`LevelFilter`]. When a span or event is recorded, the [`Collect::enabled`] method
/// compares the span or event's `Level` against the configured [`LevelFilter`].
/// The optional [`Collect::max_level_hint`] method can also be implemented to  allow spans
/// and events above a maximum verbosity level to be skipped more efficiently,
/// often improving performance in short-lived programs.
///
/// ```
/// use tracing_core::{span, Event, Level, LevelFilter, Collect, Metadata};
/// # use tracing_core::span::{Id, Record, Current};
///
/// #[derive(Debug)]
/// pub struct MyCollector {
///     /// The most verbose level that this collector will enable.
///     max_level: LevelFilter,
///
///     // ...
/// }
///
/// impl MyCollector {
///     /// Returns a new `MyCollector` which will record spans and events up to
///     /// `max_level`.
///     pub fn with_max_level(max_level: LevelFilter) -> Self {
///         Self {
///             max_level,
///             // ...
///         }
///     }
/// }
/// impl Collect for MyCollector {
///     fn enabled(&self, meta: &Metadata<'_>) -> bool {
///         // A span or event is enabled if it is at or below the configured
///         // maximum level.
///         meta.level() <= &self.max_level
///     }
///
///     // This optional method returns the most verbose level that this
///     // collector will enable. Although implementing this method is not
///     // *required*, it permits additional optimizations when it is provided,
///     // allowing spans and events above the max level to be skipped
///     // more efficiently.
///     fn max_level_hint(&self) -> Option<LevelFilter> {
///         Some(self.max_level)
///     }
///
///     // Implement the rest of the collector...
///     fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
///         // ...
///         # drop(span); Id::from_u64(1)
///     }

///     fn event(&self, event: &Event<'_>) {
///         // ...
///         # drop(event);
///     }
///
///     // ...
///     # fn enter(&self, _: &Id) {}
///     # fn exit(&self, _: &Id) {}
///     # fn record(&self, _: &Id, _: &Record<'_>) {}
///     # fn record_follows_from(&self, _: &Id, _: &Id) {}
///     # fn current_span(&self) -> Current { Current::unknown() }
/// }
/// ```
///
/// It is worth noting that the `tracing-subscriber` crate provides [additional
/// APIs][envfilter] for performing more sophisticated filtering, such as
/// enabling different levels based on which module or crate a span or event is
/// recorded in.
///
/// [`DEBUG`]: Level::DEBUG
/// [`INFO`]: Level::INFO
/// [`TRACE`]: Level::TRACE
/// [`Collect::enabled`]: crate::collect::Collect::enabled
/// [`Collect::max_level_hint`]: crate::collect::Collect::max_level_hint
/// [collector]: crate::collect::Collect
/// [envfilter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Level(LevelInner);

/// A filter comparable to a verbosity [`Level`].
///
/// If a [`Level`] is considered less than a `LevelFilter`, it should be
/// considered enabled; if greater than or equal to the `LevelFilter`,
/// that level is disabled. See [`LevelFilter::current`] for more
/// details.
///
/// Note that this is essentially identical to the `Level` type, but with the
/// addition of an [`OFF`] level that completely disables all trace
/// instrumentation.
///
/// See the documentation for the [`Level`] type to see how `Level`s
/// and `LevelFilter`s interact.
///
/// [`OFF`]: LevelFilter::OFF
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct LevelFilter(LevelFilterInner);

/// Indicates that a string could not be parsed to a valid level.
#[derive(Clone, Debug)]
pub struct ParseLevelFilterError(());

// ===== impl Level =====

impl Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    pub const ERROR: Level = Level(LevelInner::Error);
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    pub const WARN: Level = Level(LevelInner::Warn);
    /// The "info" level.
    ///
    /// Designates useful information.
    pub const INFO: Level = Level(LevelInner::Info);
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    pub const DEBUG: Level = Level(LevelInner::Debug);
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    pub const TRACE: Level = Level(LevelInner::Trace);

    /// Returns the string representation of the `Level`.
    ///
    /// This returns the same string as the `fmt::Display` implementation.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Level::TRACE => "TRACE",
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARN",
            Level::ERROR => "ERROR",
        }
    }

    pub(crate) fn to_furi(self) -> sys::FuriLogLevel {
        self.0 as u8
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Level::TRACE => f.pad("TRACE"),
            Level::DEBUG => f.pad("DEBUG"),
            Level::INFO => f.pad("INFO"),
            Level::WARN => f.pad("WARN"),
            Level::ERROR => f.pad("ERROR"),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for ParseLevelError {}

impl FromStr for Level {
    type Err = ParseLevelError;
    fn from_str(s: &str) -> Result<Self, ParseLevelError> {
        match s {
            s if s.eq_ignore_ascii_case("error") => Ok(Level::ERROR),
            s if s.eq_ignore_ascii_case("warn") => Ok(Level::WARN),
            s if s.eq_ignore_ascii_case("info") => Ok(Level::INFO),
            s if s.eq_ignore_ascii_case("debug") => Ok(Level::DEBUG),
            s if s.eq_ignore_ascii_case("trace") => Ok(Level::TRACE),
            _ => Err(ParseLevelError { _p: () }),
        }
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum LevelInner {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = sys::FuriLogLevel_FuriLogLevelTrace as usize,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = sys::FuriLogLevel_FuriLogLevelDebug as usize,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = sys::FuriLogLevel_FuriLogLevelInfo as usize,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn = sys::FuriLogLevel_FuriLogLevelWarn as usize,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = sys::FuriLogLevel_FuriLogLevelError as usize,
}

// === impl LevelFilter ===

impl From<Level> for LevelFilter {
    #[inline]
    fn from(level: Level) -> Self {
        Self::from_level(level)
    }
}

impl From<Option<Level>> for LevelFilter {
    #[inline]
    fn from(level: Option<Level>) -> Self {
        level.map(Self::from_level).unwrap_or(Self::OFF)
    }
}

impl From<LevelFilter> for Option<Level> {
    #[inline]
    fn from(filter: LevelFilter) -> Self {
        filter.into_level()
    }
}

impl LevelFilter {
    /// The "off" level.
    ///
    /// Designates that trace instrumentation should be completely disabled.
    pub const OFF: LevelFilter = LevelFilter(LevelFilterInner::Off);
    /// The "error" level.
    ///
    /// Designates very serious errors.
    pub const ERROR: LevelFilter = LevelFilter::from_level(Level::ERROR);
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    pub const WARN: LevelFilter = LevelFilter::from_level(Level::WARN);
    /// The "info" level.
    ///
    /// Designates useful information.
    pub const INFO: LevelFilter = LevelFilter::from_level(Level::INFO);
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    pub const DEBUG: LevelFilter = LevelFilter::from_level(Level::DEBUG);
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    pub const TRACE: LevelFilter = LevelFilter::from_level(Level::TRACE);

    /// Returns a `LevelFilter` that enables spans and events with verbosity up
    /// to and including `level`.
    pub const fn from_level(level: Level) -> Self {
        Self(match level.0 {
            LevelInner::Trace => LevelFilterInner::Trace,
            LevelInner::Debug => LevelFilterInner::Debug,
            LevelInner::Info => LevelFilterInner::Info,
            LevelInner::Warn => LevelFilterInner::Warn,
            LevelInner::Error => LevelFilterInner::Error,
        })
    }

    /// Returns the most verbose [`Level`] that this filter accepts, or `None`
    /// if it is [`OFF`].
    ///
    /// [`Level`]: super::Level
    /// [`OFF`]: LevelFilter::OFF
    pub const fn into_level(self) -> Option<Level> {
        match self.0 {
            LevelFilterInner::Trace => Some(Level::TRACE),
            LevelFilterInner::Debug => Some(Level::DEBUG),
            LevelFilterInner::Info => Some(Level::INFO),
            LevelFilterInner::Warn => Some(Level::WARN),
            LevelFilterInner::Error => Some(Level::ERROR),
            LevelFilterInner::Off => None,
        }
    }

    /// Returns a `LevelFilter` that matches the most verbose [`Level`] that any
    /// currently active [collector] will enable.
    ///
    /// User code should treat this as a *hint*. If a given span or event has a
    /// level *higher* than the returned `LevelFilter`, it will not be enabled.
    /// However, if the level is less than or equal to this value, the span or
    /// event is *not* guaranteed to be enabled; the collector will still
    /// filter each callsite individually.
    ///
    /// Therefore, comparing a given span or event's level to the returned
    /// `LevelFilter` **can** be used for determining if something is
    /// *disabled*, but **should not** be used for determining if something is
    /// *enabled*.
    ///
    /// [`Level`]: super::Level
    /// [collector]: super::Collect
    #[inline(always)]
    pub fn current() -> Self {
        match unsafe { sys::furi_log_get_level() } {
            // Default log level is defined in `furi/core/log.c` in the FlipperZero firmware.
            sys::FuriLogLevel_FuriLogLevelDefault => Self::INFO,
            sys::FuriLogLevel_FuriLogLevelNone => Self::OFF,
            sys::FuriLogLevel_FuriLogLevelError => Self::ERROR,
            sys::FuriLogLevel_FuriLogLevelWarn => Self::WARN,
            sys::FuriLogLevel_FuriLogLevelInfo => Self::INFO,
            sys::FuriLogLevel_FuriLogLevelDebug => Self::DEBUG,
            sys::FuriLogLevel_FuriLogLevelTrace => Self::TRACE,
            #[cfg(debug_assertions)]
            unknown => unreachable!(
                "/!\\ `LevelFilter` representation seems to have changed! /!\\ \n\
                This is a bug (and it's pretty bad). Please contact the `flipperzero` \
                maintainers. Thank you and I'm sorry.\n \
                The offending repr was: {:?}",
                unknown,
            ),
            #[cfg(not(debug_assertions))]
            _ => unsafe {
                // Using `unreachable_unchecked` here (rather than
                // `unreachable!()`) is necessary to ensure that rustc generates
                // an identity conversion from integer -> discriminant, rather
                // than generating a lookup table. We want to ensure this
                // function is a single `bl` instruction (sometimes followed by
                // a `subs` instruction to handle `FuriLogLevelDefault`) if at
                // all possible, because it is called *every* time a logging
                // callsite is hit; and it is (potentially) the only code in the
                // hottest path for skipping a majority of callsites when level
                // filtering is in use.
                //
                // safety: This branch is only truly unreachable if we guarantee
                // that no values other than the possible enum discriminants
                // will *ever* be present. The log filter is initialized by the
                // Flipper Zero SDK to `FuriLogLevelDefault`, which is not a
                // valid `LevelFilter` discriminant but is specifically handled
                // above. It is set either internally by the Flipper Zero, or
                // through the Flipper Zero SDK. The latter we expose via the
                // `set_max` function, which takes a `LevelFilter` parameter;
                // this restricts the inputs to `set_max` to the set of valid
                // discriminants. Therefore, **as long as `furi_log_set_level`
                // is only ever called by `set_max`**, this is safe.
                core::hint::unreachable_unchecked()
            },
        }
    }

    pub(crate) fn set_max(LevelFilter(level): LevelFilter) {
        let val = level as u8;

        unsafe { sys::furi_log_set_level(val) };
    }
}

impl fmt::Display for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LevelFilter::OFF => f.pad("off"),
            LevelFilter::ERROR => f.pad("error"),
            LevelFilter::WARN => f.pad("warn"),
            LevelFilter::INFO => f.pad("info"),
            LevelFilter::DEBUG => f.pad("debug"),
            LevelFilter::TRACE => f.pad("trace"),
        }
    }
}

impl fmt::Debug for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LevelFilter::OFF => f.pad("LevelFilter::OFF"),
            LevelFilter::ERROR => f.pad("LevelFilter::ERROR"),
            LevelFilter::WARN => f.pad("LevelFilter::WARN"),
            LevelFilter::INFO => f.pad("LevelFilter::INFO"),
            LevelFilter::DEBUG => f.pad("LevelFilter::DEBUG"),
            LevelFilter::TRACE => f.pad("LevelFilter::TRACE"),
        }
    }
}

impl FromStr for LevelFilter {
    type Err = ParseLevelFilterError;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from {
            "" => Some(LevelFilter::ERROR),
            s if s.eq_ignore_ascii_case("error") => Some(LevelFilter::ERROR),
            s if s.eq_ignore_ascii_case("warn") => Some(LevelFilter::WARN),
            s if s.eq_ignore_ascii_case("info") => Some(LevelFilter::INFO),
            s if s.eq_ignore_ascii_case("debug") => Some(LevelFilter::DEBUG),
            s if s.eq_ignore_ascii_case("trace") => Some(LevelFilter::TRACE),
            s if s.eq_ignore_ascii_case("off") => Some(LevelFilter::OFF),
            _ => None,
        }
        .ok_or(ParseLevelFilterError(()))
    }
}

/// Returned if parsing a `Level` fails.
#[derive(Debug)]
pub struct ParseLevelError {
    _p: (),
}

impl fmt::Display for ParseLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(
            "error parsing level: expected one of \"error\", \"warn\", \
             \"info\", \"debug\", \"trace\"",
        )
    }
}

impl fmt::Display for ParseLevelFilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(
            "error parsing level filter: expected one of \"off\", \"error\", \
            \"warn\", \"info\", \"debug\", \"trace\"",
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseLevelFilterError {}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum LevelFilterInner {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = sys::FuriLogLevel_FuriLogLevelTrace as usize,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = sys::FuriLogLevel_FuriLogLevelDebug as usize,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = sys::FuriLogLevel_FuriLogLevelInfo as usize,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn = sys::FuriLogLevel_FuriLogLevelWarn as usize,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = sys::FuriLogLevel_FuriLogLevelError as usize,
    /// The "off" level.
    ///
    /// Designates that trace instrumentation should be completely disabled.
    Off = sys::FuriLogLevel_FuriLogLevelNone as usize,
}

// ==== Level and LevelFilter comparisons ====

impl PartialEq<LevelFilter> for Level {
    #[inline(always)]
    fn eq(&self, other: &LevelFilter) -> bool {
        self.0 as usize == (other.0 as usize)
    }
}

impl PartialOrd<LevelFilter> for Level {
    #[inline(always)]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some((self.0 as usize).cmp(&(other.0 as usize)))
    }
}

impl PartialEq<Level> for LevelFilter {
    #[inline(always)]
    fn eq(&self, other: &Level) -> bool {
        (self.0 as usize) == other.0 as usize
    }
}

impl PartialOrd<Level> for LevelFilter {
    #[inline(always)]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some((self.0 as usize).cmp(&(other.0 as usize)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn level_from_str() {
        assert_eq!("error".parse::<Level>().unwrap(), Level::ERROR);
    }

    #[test]
    fn filter_level_conversion() {
        let mapping = [
            (LevelFilter::OFF, None),
            (LevelFilter::ERROR, Some(Level::ERROR)),
            (LevelFilter::WARN, Some(Level::WARN)),
            (LevelFilter::INFO, Some(Level::INFO)),
            (LevelFilter::DEBUG, Some(Level::DEBUG)),
            (LevelFilter::TRACE, Some(Level::TRACE)),
        ];
        for (filter, level) in mapping.iter() {
            assert_eq!(filter.into_level(), *level);
            match level {
                Some(level) => {
                    let actual: LevelFilter = (*level).into();
                    assert_eq!(actual, *filter);
                }
                None => {
                    let actual: LevelFilter = None.into();
                    assert_eq!(actual, *filter);
                }
            }
        }
    }

    #[test]
    fn level_filter_is_usize_sized() {
        assert_eq!(
            mem::size_of::<LevelFilter>(),
            mem::size_of::<usize>(),
            "`LevelFilter` is no longer `usize`-sized! global MAX_LEVEL may now be invalid!"
        )
    }

    #[test]
    fn level_filter_reprs() {
        let mapping = [
            (LevelFilter::OFF, LevelFilterInner::Off as usize),
            (LevelFilter::ERROR, LevelFilterInner::Error as usize),
            (LevelFilter::WARN, LevelFilterInner::Warn as usize),
            (LevelFilter::INFO, LevelFilterInner::Info as usize),
            (LevelFilter::DEBUG, LevelFilterInner::Debug as usize),
            (LevelFilter::TRACE, LevelFilterInner::Trace as usize),
        ];
        for &(filter, expected) in &mapping {
            let repr = unsafe {
                // safety: The entire purpose of this test is to assert that the
                // actual repr matches what we expect it to be --- we're testing
                // that *other* unsafe code is sound using the transmuted value.
                // We're not going to do anything with it that might be unsound.
                mem::transmute::<LevelFilter, usize>(filter)
            };
            assert_eq!(expected, repr, "repr changed for {:?}", filter)
        }
    }
}
