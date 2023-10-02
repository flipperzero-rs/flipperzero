use core::cmp::Ordering;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use flipperzero_sys as sys;
use ufmt::derive::uDebug;

/// The maximum number of ticks that a [`Duration`] can contain for it to be usable with
/// [`Instant`].
const MAX_DURATION_TICKS: u32 = u32::MAX / 2;

const NANOS_PER_SEC_F: f64 = 1_000_000_000_f64;
const NANOS_PER_SEC: u64 = 1_000_000_000;
const NANOS_PER_MILLI: u64 = 1_000_000;
const NANOS_PER_MICRO: u64 = 1_000;
const MILLIS_PER_SEC: u32 = 1_000;

/// Converts the given number of nanoseconds to ticks.
fn ns_to_ticks(nanos: u64) -> u64 {
    let rate = unsafe { sys::furi_kernel_get_tick_frequency() };
    if rate == MILLIS_PER_SEC {
        // This can be up to around 2^45 ticks.
        nanos / NANOS_PER_MILLI
    } else {
        // If `rate` is higher than `NANOS_PER_SEC_F` then this will overflow and be
        // truncated. We assume that no Flipper Zero is clocked this fast.
        ((f64::from(rate) / NANOS_PER_SEC_F) * nanos as f64) as u64
    }
}

/// Converts the given number of ticks to nanoseconds.
///
/// The upper 2 bits of the return value will always be zero.
fn ticks_to_ns(ticks: u32) -> u64 {
    let rate = unsafe { sys::furi_kernel_get_tick_frequency() };
    if rate == MILLIS_PER_SEC {
        // This can be up to around 2^52 nanoseconds.
        (ticks as u64) * NANOS_PER_MILLI
    } else {
        // This can be up to around 2^62 nanoseconds when `rate` is 1.
        ((NANOS_PER_SEC_F / f64::from(rate)) * ticks as f64) as u64
    }
}

/// A measurement of a wrapping clock. Opaque and useful only with [`Duration`].
#[derive(Copy, Clone, Debug, uDebug, PartialEq, Eq, Hash)]
pub struct Instant(pub(super) u32);

impl Instant {
    /// Returns an instant corresponding to "now".
    #[must_use]
    pub fn now() -> Instant {
        Instant(unsafe { sys::furi_get_tick() })
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// # Panics
    ///
    /// Panics if `earlier` is later than `self`.
    #[must_use]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier)
            .expect("earlier is later than self")
    }

    /// Returns the amount of time elapsed from another instant to this one, or `None` if
    /// that instant is later than this one.
    #[must_use]
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        if self >= &earlier {
            Some(Duration(self.0.wrapping_sub(earlier.0)))
        } else {
            None
        }
    }

    /// Returns the amount of time elapsed from another instant to this one, or zero
    /// duration if that instant is later than this one.
    #[must_use]
    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    /// Returns the amount of time elapsed since this instant.
    ///
    /// Due to the wrapping nature of the clock, there are several caveats on the value
    /// returned by this method:
    /// - The longest duration this can return is [`Duration::MAX`]; durations above this
    ///   length will saturate to it. Use [`Instant::checked_duration_since`] to detect
    ///   this occurring.
    /// - The elapsed time is periodic, and jumps back to zero approximately every
    ///   `2 * Duration::MAX` time.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Instant::now()
            .checked_duration_since(*self)
            .unwrap_or(Duration::MAX)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the underlying
    /// data structure), `None` otherwise.
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        if duration.0 <= MAX_DURATION_TICKS {
            Some(Instant(self.0.wrapping_add(duration.0)))
        } else {
            None
        }
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the underlying
    /// data structure), `None` otherwise.
    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        if duration.0 <= MAX_DURATION_TICKS {
            Some(Instant(self.0.wrapping_sub(duration.0)))
        } else {
            None
        }
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            // We cannot distinguish between equality and exact wraparound.
            Ordering::Equal
        } else {
            // We use modular arithmetic to define ordering.
            // This requires a maximum `Duration` value of `MAX_DURATION_TICKS`.
            self.0
                .wrapping_sub(other.0)
                .cmp(&MAX_DURATION_TICKS)
                .reverse()
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by
    /// the underlying data structure. See [`Instant::checked_add`] for a version without
    /// panic.
    fn add(self, other: Duration) -> Instant {
        self.checked_add(other)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        self.checked_sub(other)
            .expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// # Panics
    ///
    /// Panics if `other` is later than `self`.
    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}

/// A `Duration` type to represent a span of time, typically used for system timeouts.
///
/// Each `Duration` is composed of a whole number of "ticks", the length of which depends
/// on the firmware's tick frequency. While a `Duration` can contain any value that
/// is at most [`u32::MAX`] ticks, only the range `[Duration::ZERO..=DURATION::MAX]` can
/// be used with [`Instant`].
#[derive(Clone, Copy, Debug, uDebug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub(super) u32);

impl Duration {
    /// A duration of zero time.
    pub const ZERO: Duration = Duration(0);

    /// The maximum duration.
    ///
    /// May vary by platform as necessary. Must be able to contain the difference between
    /// two instances of [`Instant`]. This constraint gives it a value of about 24 days in
    /// practice on stock firmware.
    pub const MAX: Duration = Duration(MAX_DURATION_TICKS);

    /// Creates a new `Duration` from the specified number of whole seconds.
    ///
    /// # Panics
    ///
    /// Panics if the duration would exceed [`u32::MAX`] ticks.
    #[inline]
    #[must_use]
    pub fn from_secs(secs: u64) -> Duration {
        let ticks = ns_to_ticks(secs * NANOS_PER_SEC);
        let ticks = u32::try_from(ticks).expect("Duration is too long");
        Duration(ticks)
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    ///
    /// # Panics
    ///
    /// Panics if the duration would exceed [`u32::MAX`] ticks.
    #[inline]
    #[must_use]
    pub fn from_millis(millis: u64) -> Duration {
        let ticks = ns_to_ticks(millis * NANOS_PER_MILLI);
        let ticks = u32::try_from(ticks).expect("Duration is too long");
        Duration(ticks)
    }

    /// Creates a new `Duration` from the specified number of microseconds.
    ///
    /// # Panics
    ///
    /// Panics if the duration would exceed [`u32::MAX`] ticks.
    #[inline]
    #[must_use]
    pub fn from_micros(micros: u64) -> Duration {
        let ticks = ns_to_ticks(micros * NANOS_PER_MICRO);
        let ticks = u32::try_from(ticks).expect("Duration is too long");
        Duration(ticks)
    }

    /// Creates a new `Duration` from the specified number of nanoseconds.
    ///
    /// # Panics
    ///
    /// Panics if the duration would exceed [`u32::MAX`] ticks.
    #[inline]
    #[must_use]
    pub fn from_nanos(nanos: u64) -> Duration {
        let ticks = ns_to_ticks(nanos);
        let ticks = u32::try_from(ticks).expect("Duration is too long");
        Duration(ticks)
    }

    /// Returns true if this `Duration` spans no time.
    #[inline]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Returns the total number of whole seconds contained by this `Duration`.
    #[inline]
    #[must_use]
    pub fn as_secs(&self) -> u64 {
        // This can be up to around 2^33 seconds with a tick frequency of 1.
        self.as_nanos() / NANOS_PER_SEC
    }

    /// Returns the total number of whole milliseconds contained by this `Duration`.
    #[inline]
    #[must_use]
    pub fn as_millis(&self) -> u64 {
        // This can be up to around 2^43 seconds with a tick frequency of 1.
        self.as_nanos() / NANOS_PER_MILLI
    }

    /// Returns the total number of whole microseconds contained by this `Duration`.
    #[inline]
    #[must_use]
    pub fn as_micros(&self) -> u64 {
        // This can be up to around 2^53 seconds with a tick frequency of 1.
        self.as_nanos() / NANOS_PER_MICRO
    }

    /// Returns the total number of nanoseconds contained by this `Duration`.
    #[inline]
    #[must_use]
    pub fn as_nanos(&self) -> u64 {
        // This can be up to around 2^62 seconds with a tick frequency of 1.
        ticks_to_ns(self.0)
    }

    /// Checked `Duration` addition. Computes `self + other`, returning [`None`] if
    /// overflow occurred.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn checked_add(self, rhs: Duration) -> Option<Duration> {
        if let Some(ticks) = self.0.checked_add(rhs.0) {
            Some(Duration(ticks))
        } else {
            None
        }
    }

    /// Saturating `Duration` addition. Computes `self + other`, returning
    /// [`Duration::MAX`] if overflow occurred.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn saturating_add(self, rhs: Duration) -> Duration {
        match self.checked_add(rhs) {
            Some(res) => res,
            None => Duration::MAX,
        }
    }

    /// Checked `Duration` subtraction. Computes `self - other`, returning [`None`] if the
    /// result would be negative or if overflow occurred.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        if let Some(ticks) = self.0.checked_sub(rhs.0) {
            Some(Duration(ticks))
        } else {
            None
        }
    }

    /// Saturating `Duration` subtraction. Computes `self - other`, returning
    /// [`Duration::ZERO`] if the result would be negative or if overflow occurred.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn saturating_sub(self, rhs: Duration) -> Duration {
        match self.checked_sub(rhs) {
            Some(res) => res,
            None => Duration::ZERO,
        }
    }

    /// Checked `Duration` multiplication. Computes `self * other`, returning [`None`] if
    /// overflow occurred.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn checked_mul(self, rhs: u32) -> Option<Duration> {
        if let Some(ticks) = self.0.checked_mul(rhs) {
            Some(Duration(ticks))
        } else {
            None
        }
    }

    /// Saturating `Duration` multiplication. Computes `self * other`, returning
    /// [`Duration::MAX`] if overflow occurred.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn saturating_mul(self, rhs: u32) -> Duration {
        match self.checked_mul(rhs) {
            Some(res) => res,
            None => Duration::MAX,
        }
    }

    /// Checked `Duration` division. Computes `self / other`, returning [`None`] if
    /// `other == 0`.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn checked_div(self, rhs: u32) -> Option<Duration> {
        if rhs != 0 {
            let ticks = self.0 / rhs;
            Some(Duration(ticks))
        } else {
            None
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(rhs)
            .expect("overflow when adding durations")
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        self.checked_sub(rhs)
            .expect("overflow when subtracting durations")
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: u32) -> Duration {
        self.checked_mul(rhs)
            .expect("overflow when multiplying duration by scalar")
    }
}

impl Mul<Duration> for u32 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Duration {
        rhs * self
    }
}

impl MulAssign<u32> for Duration {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Duration {
    type Output = Duration;

    fn div(self, rhs: u32) -> Duration {
        self.checked_div(rhs)
            .expect("divide by zero error when dividing duration by scalar")
    }
}

impl DivAssign<u32> for Duration {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Sum for Duration {
    fn sum<I: Iterator<Item = Duration>>(iter: I) -> Duration {
        Duration(iter.map(|d| d.0).sum())
    }
}

impl<'a> Sum<&'a Duration> for Duration {
    fn sum<I: Iterator<Item = &'a Duration>>(iter: I) -> Duration {
        Duration(iter.map(|d| d.0).sum())
    }
}

#[flipperzero_test::tests]
mod tests {
    use super::{ticks_to_ns, Duration, Instant, MAX_DURATION_TICKS};
    use crate::println;

    #[cfg(feature = "alloc")]
    use {crate::furi::thread, alloc::vec::Vec};

    macro_rules! assert_almost_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = ($a, $b);
            if a != b {
                let (a, b) = if a > b { (a, b) } else { (b, a) };
                assert!(
                    a - Duration::from_micros(1) <= b,
                    "{:?} is not almost equal to {:?}",
                    a,
                    b
                );
            }
        }};
    }

    #[test]
    fn instant_increases() {
        let a = Instant::now();
        loop {
            let b = Instant::now();
            assert!(b >= a);
            if b > a {
                break;
            }
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn instant_increases_concurrent() {
        let threads: Vec<_> = (0..8)
            .map(|_| {
                thread::spawn(|| {
                    let mut old = Instant::now();
                    let count = 1_000; // TODO 5_000_000 hangs; figure out why.
                    for _ in 0..count {
                        let new = Instant::now();
                        assert!(new >= old);
                        old = new;
                    }
                    0
                })
            })
            .collect();
        for t in threads {
            t.join();
        }
    }

    #[test]
    fn instant_elapsed() {
        let a = Instant::now();
        let _ = a.elapsed();
    }

    #[test]
    fn instant_math() {
        let a = Instant::now();
        let b = Instant::now();
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        let dur = b.duration_since(a);
        println!("dur: {} ns", dur.as_nanos());
        assert_almost_eq!(b - dur, a);
        assert_almost_eq!(a + dur, b);

        let second = Duration::from_secs(1);
        assert_almost_eq!(a - second + second, a);
        assert_almost_eq!(
            a.checked_sub(second).unwrap().checked_add(second).unwrap(),
            a
        );

        // checked_add_duration will not panic on overflow
        let mut maybe_t = Some(Instant::now());
        let max_duration = Duration::from_nanos(ticks_to_ns(u32::MAX));
        // in case `Instant` can store `>= now + max_duration`.
        for _ in 0..2 {
            maybe_t = maybe_t.and_then(|t| t.checked_add(max_duration));
        }
        assert_eq!(maybe_t, None);

        // checked_add_duration calculates the right time and will work for another week
        let week = Duration::from_secs(60 * 60 * 24 * 7);
        assert_eq!(a + week, a.checked_add(week).unwrap());
    }

    #[test]
    fn instant_math_is_associative() {
        let now = Instant::now();
        let offset = Duration::from_millis(5);
        // Changing the order of instant math shouldn't change the results,
        // especially when the expression reduces to X + identity.
        assert_eq!((now + offset) - now, (now - now) + offset);

        // On any platform, `Instant` should have the same resolution as `Duration`
        // (i.e. 1 tick) or better. Otherwise, math will be non-associative.
        let tick_nanos = ticks_to_ns(1);
        let now = Instant::now();
        let provided_offset = Duration::from_nanos(tick_nanos);
        let later = now + provided_offset;
        let measured_offset = later - now;
        assert_eq!(measured_offset, provided_offset);
    }

    #[test]
    fn instant_checked_duration_since_nopanic() {
        let now = Instant::now();
        let earlier = now - Duration::from_secs(1);
        let later = now + Duration::from_secs(1);
        assert_eq!(earlier.checked_duration_since(now), None);
        assert_eq!(
            later.checked_duration_since(now),
            Some(Duration::from_secs(1))
        );
        assert_eq!(now.checked_duration_since(now), Some(Duration::ZERO));
    }

    #[test]
    fn instant_saturating_duration_since_nopanic() {
        let a = Instant::now();
        let ret = (a - Duration::from_secs(1)).saturating_duration_since(a);
        assert_eq!(ret, Duration::ZERO);
    }

    #[test]
    fn big_math() {
        // Check that the same result occurs when adding/subtracting each duration one at a time as when
        // adding/subtracting them all at once.
        #[track_caller]
        fn check<T: Eq + Copy + core::fmt::Debug>(
            start: Option<T>,
            op: impl Fn(&T, Duration) -> Option<T>,
        ) {
            const DURATIONS: [Duration; 2] = [Duration(MAX_DURATION_TICKS >> 1), Duration(50)];
            if let Some(start) = start {
                assert_eq!(
                    op(&start, DURATIONS.into_iter().sum()),
                    DURATIONS.into_iter().try_fold(start, |t, d| op(&t, d))
                )
            }
        }

        let instant = Instant::now();
        check(instant.checked_sub(Duration(100)), Instant::checked_add);
        check(instant.checked_sub(Duration::MAX), Instant::checked_add);
        check(instant.checked_add(Duration(100)), Instant::checked_sub);
        check(instant.checked_add(Duration::MAX), Instant::checked_sub);
    }
}
