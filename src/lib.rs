#![cfg_attr(not(any(feature = "std", test)), no_std)]

//! fasttime — small UTC date/time library built around Ben Joffe's
//! fast 64-bit days→date algorithm.
//!
//! Features:
//! - `no_std` compatible (only `core`; `std` is optional).
//! - `Date` / `Time` / `DateTime` (UTC).
//! - `Duration` with nanosecond precision.
//! - `UtcOffset` and `OffsetDateTime` (fixed offset, RFC 3339-style).
//! - ISO-like formatting via `Display`.
//! - Parsing of:
//!   - `Date`: "YYYY-MM-DD"
//!   - `Time`: "HH:MM:SS[.fffffffff]"
//!   - `DateTime` (UTC): "YYYY-MM-DDTHH:MM:SS[.fffffffff]Z"
//!   - `OffsetDateTime`: `YYYY-MM-DDTHH:MM:SS[.fffffffff][Z|±HH:MM]` (RFC 3339 subset).
//! - `DateTime::now_utc()` when the `std` feature is enabled.
//!
//! ## Python Bindings
//!
//! When built with the `python` feature, this crate provides Python bindings via `PyO3`.
//! See the `python/` directory for examples and documentation.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

#[cfg(feature = "python")]
mod python;

/// Calendar weekday (ISO order, Monday = 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    #[must_use]
    pub fn number_from_monday(self) -> u8 {
        match self {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        }
    }
}

/// Errors constructing or parsing a `Date`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateError {
    /// Year/month/day combination is not a valid Gregorian date.
    InvalidDate,
    /// The date is outside the supported range.
    OutOfRange,
}

/// Gregorian calendar date (proleptic).
///
/// This is independent of any time zone; think "calendar day in UTC".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: i32,
    pub month: u8, // 1..=12
    pub day: u8,   // 1..=31
}

impl Date {
    /// Construct a date, validating year/month/day.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::InvalidDate`] if the components do not form a valid
    /// Gregorian date.
    #[inline]
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Result<Self, DateError> {
        if !(1..=12).contains(&month) {
            return Err(DateError::InvalidDate);
        }
        let dim = days_in_month(year, month);
        if day == 0 || day > dim {
            return Err(DateError::InvalidDate);
        }
        Ok(Date { year, month, day })
    }

    /// Construct a date with minimal checking; debug-only asserts.
    ///
    /// Panics in debug builds if the date is invalid.
    #[must_use]
    pub const fn from_ymd_unchecked(year: i32, month: u8, day: u8) -> Self {
        // These are simple invariants, checked in debug builds only.
        debug_assert!(month >= 1 && month <= 12);
        debug_assert!(day >= 1 && day <= 31);
        Date { year, month, day }
    }

    /// Ben Joffe's fast 64-bit days→date algorithm, adapted to Rust.
    ///
    /// `days` is days since Unix epoch:
    ///
    /// - 1970-01-01 => 0
    /// - 1969-12-31 => -1
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if `days` cannot be represented by a
    /// [`Date`], or [`DateError::InvalidDate`] if conversion produces invalid
    /// date components.
    #[inline]
    pub fn from_days_since_unix_epoch(days: i64) -> Result<Self, DateError> {
        // Constants from the article (x64 version).
        const ERAS: i64 = 4_726_498_270;
        const D_SHIFT: i64 = 146_097 * ERAS - 719_469;
        const Y_SHIFT: i64 = 400 * ERAS - 1;
        const C1: u64 = 505_054_698_555_331;
        const C2: u64 = 50_504_432_782_230_121;
        const C3: u64 = 8_619_973_866_219_416;

        let rev = D_SHIFT.checked_sub(days).ok_or(DateError::OutOfRange)?;
        let rev = u64::try_from(rev).map_err(|_| DateError::OutOfRange)?;

        // 64x64 → high 64 bit multiplies via u128.
        let cen = i64::try_from((u128::from(rev) * u128::from(C1)) >> 64)
            .map_err(|_| DateError::OutOfRange)?;
        let rev = i64::try_from(rev).map_err(|_| DateError::OutOfRange)?;
        let jul = rev + cen - cen / 4;

        let jul = u64::try_from(jul).map_err(|_| DateError::OutOfRange)?;
        let num = u128::from(jul) * u128::from(C2);
        let high = i64::try_from(num >> 64).map_err(|_| DateError::OutOfRange)?;
        let yrs = Y_SHIFT - high;
        let low = u64::try_from(num & u128::from(u64::MAX)).map_err(|_| DateError::OutOfRange)?;
        let ypt = i64::try_from((782_432u128 * u128::from(low)) >> 64)
            .map_err(|_| DateError::OutOfRange)?;

        let bump = ypt < 126_464;
        let shift: i64 = if bump { 191_360 } else { 977_792 };

        let n: i64 = (yrs.rem_euclid(4)) * 512 + shift - ypt;

        let n_low = u64::try_from(n).map_err(|_| DateError::OutOfRange)? & 0xFFFF;
        let d = i64::try_from((u128::from(n_low) * u128::from(C3)) >> 64)
            .map_err(|_| DateError::OutOfRange)?;

        let day_i: i64 = d + 1;
        let month_i: i64 = n / 65_536;
        let year_i = yrs + i64::from(bump);

        if !(i64::from(i32::MIN)..=i64::from(i32::MAX)).contains(&year_i) {
            return Err(DateError::OutOfRange);
        }
        let year = i32::try_from(year_i).map_err(|_| DateError::OutOfRange)?;
        let month = u8::try_from(month_i).map_err(|_| DateError::InvalidDate)?;
        let day = u8::try_from(day_i).map_err(|_| DateError::InvalidDate)?;

        // Extra safety: validate
        if Date::from_ymd(year, month, day).is_err() {
            return Err(DateError::InvalidDate);
        }

        Ok(Date { year, month, day })
    }

    /// Convert to days since Unix epoch (1970-01-01 = 0).
    ///
    /// This uses a modified Neri-Schneider inverse civil→days formula
    /// (as described by Ben Joffe), exact for the proleptic Gregorian calendar.
    #[inline]
    #[must_use]
    pub fn days_since_unix_epoch(self) -> i64 {
        days_from_civil(self.year, self.month, self.day)
    }

    /// Day of week (Monday = 1).
    ///
    /// Unix epoch 1970-01-01 was a Thursday, so we just offset.
    #[must_use]
    pub fn weekday(self) -> Weekday {
        // 1970-01-01 was Thursday (4).
        let days = self.days_since_unix_epoch();
        let w = days.rem_euclid(7);
        match w {
            0 => Weekday::Thursday,
            1 => Weekday::Friday,
            2 => Weekday::Saturday,
            3 => Weekday::Sunday,
            4 => Weekday::Monday,
            5 => Weekday::Tuesday,
            6 => Weekday::Wednesday,
            _ => unreachable!(),
        }
    }

    /// Day of year, 1..=365 (or 366 for leap years).
    #[must_use]
    pub fn ordinal(self) -> u16 {
        const CUM_DAYS: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

        let month = self.month;
        let day = u16::from(self.day);
        let mut ord = CUM_DAYS[usize::from(month - 1)] + day;
        if month > 2 && is_leap_year(self.year) {
            ord += 1;
        }
        ord
    }

    /// Add a number of days, returning a new `Date` or `OutOfRange`.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if the resulting date cannot be
    /// represented.
    pub fn add_days(self, days: i64) -> Result<Date, DateError> {
        let base = self.days_since_unix_epoch();
        let result = base.checked_add(days).ok_or(DateError::OutOfRange)?;
        Date::from_days_since_unix_epoch(result)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        let days_self = self.days_since_unix_epoch();
        let days_other = other.days_since_unix_epoch();
        days_self.cmp(&days_other)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ISO-like: YYYY-MM-DD with at least 4 digits of year.
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl FromStr for Date {
    type Err = DateError;

    /// Parse "YYYY-MM-DD" (no timezone).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return Err(DateError::InvalidDate);
        }

        let mut start = 0;
        if bytes[0] == b'+' || bytes[0] == b'-' {
            start = 1;
            if start == bytes.len() {
                return Err(DateError::InvalidDate);
            }
        }

        let mut first = None;
        let mut second = None;
        for (i, &b) in bytes.iter().enumerate().skip(start) {
            if b == b'-' {
                if first.is_none() {
                    first = Some(i);
                } else if second.is_none() {
                    second = Some(i);
                } else {
                    return Err(DateError::InvalidDate);
                }
            }
        }

        let (Some(first), Some(second)) = (first, second) else {
            return Err(DateError::InvalidDate);
        };

        let y = parse_i32_bytes(&bytes[..first]).ok_or(DateError::InvalidDate)?;
        let m = parse_u32_bytes(&bytes[first + 1..second], 12)
            .and_then(|value| u8::try_from(value).ok())
            .ok_or(DateError::InvalidDate)?;
        let d = parse_u32_bytes(&bytes[second + 1..], 31)
            .and_then(|value| u8::try_from(value).ok())
            .ok_or(DateError::InvalidDate)?;
        Date::from_ymd(y, m, d)
    }
}

/// Errors constructing or parsing a `Time`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeError {
    InvalidTime,
}

/// Time of day in nanoseconds since midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    pub hour: u8,        // 0..=23
    pub minute: u8,      // 0..=59
    pub second: u8,      // 0..=59 (no leap seconds)
    pub nanosecond: u32, // 0..1_000_000_000
}

impl Time {
    /// Construct a time, validating each component.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::InvalidTime`] if a component is outside its valid
    /// range.
    #[inline]
    pub fn from_hms_nano(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<Self, TimeError> {
        if hour > 23 || minute > 59 || second > 59 || nanosecond >= 1_000_000_000 {
            return Err(TimeError::InvalidTime);
        }
        Ok(Time {
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    /// Total seconds since midnight (ignores nanoseconds).
    #[inline]
    #[must_use]
    pub fn seconds_since_midnight(self) -> u32 {
        u32::from(self.hour) * 3600 + u32::from(self.minute) * 60 + u32::from(self.second)
    }

    /// Total nanoseconds since midnight.
    #[inline]
    #[must_use]
    pub fn nanos_since_midnight(self) -> u64 {
        u64::from(self.seconds_since_midnight()) * 1_000_000_000 + u64::from(self.nanosecond)
    }

    /// Build from seconds and nanoseconds since midnight.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::InvalidTime`] if `secs` is not within one day or
    /// `nanos` is not within one second.
    #[inline]
    pub fn from_seconds_nanos(secs: u32, nanos: u32) -> Result<Self, TimeError> {
        if secs >= 86_400 || nanos >= 1_000_000_000 {
            return Err(TimeError::InvalidTime);
        }
        let hour = u8::try_from(secs / 3600).map_err(|_| TimeError::InvalidTime)?;
        let rem = secs % 3600;
        let minute = u8::try_from(rem / 60).map_err(|_| TimeError::InvalidTime)?;
        let second = u8::try_from(rem % 60).map_err(|_| TimeError::InvalidTime)?;
        Time::from_hms_nano(hour, minute, second, nanos)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        let nanos_self = self.nanos_since_midnight();
        let nanos_other = other.nanos_since_midnight();
        nanos_self.cmp(&nanos_other)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nanosecond == 0 {
            write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
        } else {
            // Print fractional seconds, trimming trailing zeros.
            let mut frac = [b'0'; 9];
            let mut ns = self.nanosecond;
            for i in (0..9).rev() {
                let digit = u8::try_from(ns % 10).expect("a decimal digit fits in u8");
                frac[i] = b'0' + digit;
                ns /= 10;
            }
            // find last non-zero
            let mut end = 9;
            while end > 0 && frac[end - 1] == b'0' {
                end -= 1;
            }
            let frac_str = core::str::from_utf8(&frac[..end]).unwrap_or("0");
            write!(
                f,
                "{:02}:{:02}:{:02}.{}",
                self.hour, self.minute, self.second, frac_str
            )
        }
    }
}

impl FromStr for Time {
    type Err = TimeError;

    /// Parse "HH:MM:SS[.fffffffff]".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let (hms_bytes, frac_bytes) = match bytes.iter().position(|&b| b == b'.') {
            Some(idx) => (&bytes[..idx], Some(&bytes[idx + 1..])),
            None => (bytes, None),
        };

        let mut first = None;
        let mut second = None;
        for (i, &b) in hms_bytes.iter().enumerate() {
            if b == b':' {
                if first.is_none() {
                    first = Some(i);
                } else if second.is_none() {
                    second = Some(i);
                } else {
                    return Err(TimeError::InvalidTime);
                }
            }
        }

        let (Some(first), Some(second)) = (first, second) else {
            return Err(TimeError::InvalidTime);
        };

        let h = parse_u32_bytes(&hms_bytes[..first], 23)
            .and_then(|value| u8::try_from(value).ok())
            .ok_or(TimeError::InvalidTime)?;
        let m = parse_u32_bytes(&hms_bytes[first + 1..second], 59)
            .and_then(|value| u8::try_from(value).ok())
            .ok_or(TimeError::InvalidTime)?;
        let sec = parse_u32_bytes(&hms_bytes[second + 1..], 59)
            .and_then(|value| u8::try_from(value).ok())
            .ok_or(TimeError::InvalidTime)?;

        let nanos = if let Some(fr) = frac_bytes {
            parse_fraction_nanos(fr).ok_or(TimeError::InvalidTime)?
        } else {
            0
        };

        Time::from_hms_nano(h, m, sec, nanos)
    }
}

/// Signed duration with nanosecond precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Duration {
    nanos: i128,
}

impl Duration {
    pub const ZERO: Duration = Duration { nanos: 0 };

    #[inline]
    #[must_use]
    pub fn seconds(secs: i64) -> Duration {
        Duration {
            nanos: i128::from(secs) * 1_000_000_000,
        }
    }

    #[must_use]
    pub fn milliseconds(ms: i64) -> Duration {
        Duration {
            nanos: i128::from(ms) * 1_000_000,
        }
    }

    #[must_use]
    pub fn microseconds(us: i64) -> Duration {
        Duration {
            nanos: i128::from(us) * 1_000,
        }
    }

    #[must_use]
    pub fn nanoseconds(ns: i128) -> Duration {
        Duration { nanos: ns }
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn total_seconds(self) -> f64 {
        self.nanos as f64 / 1_000_000_000.0
    }

    #[inline]
    #[must_use]
    pub fn total_nanos(self) -> i128 {
        self.nanos
    }
}

impl core::ops::Add for Duration {
    type Output = Duration;
    fn add(self, rhs: Duration) -> Duration {
        Duration {
            nanos: self.nanos + rhs.nanos,
        }
    }
}

impl core::ops::Sub for Duration {
    type Output = Duration;
    fn sub(self, rhs: Duration) -> Duration {
        Duration {
            nanos: self.nanos - rhs.nanos,
        }
    }
}

impl core::ops::Neg for Duration {
    type Output = Duration;
    fn neg(self) -> Duration {
        Duration { nanos: -self.nanos }
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nanos.cmp(&other.nanos)
    }
}

/// Combined UTC date and time (no time zone).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    #[inline]
    #[must_use]
    pub fn new(date: Date, time: Time) -> DateTime {
        DateTime { date, time }
    }

    /// Build from Unix timestamp (seconds since 1970-01-01T00:00:00Z)
    /// plus an additional nanoseconds offset (can be negative or >1e9).
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if the normalized timestamp cannot be
    /// represented, or [`DateError::InvalidDate`] if its time components are
    /// invalid.
    #[inline]
    pub fn from_unix_timestamp(secs: i64, nanos: i32) -> Result<DateTime, DateError> {
        // Normalize (secs, nanos) pair.
        let mut s = i128::from(secs);
        let mut n = i128::from(nanos);
        s += n.div_euclid(1_000_000_000);
        n = n.rem_euclid(1_000_000_000);
        let s_i64 = i64::try_from(s).map_err(|_| DateError::OutOfRange)?;

        let days = s_i64.div_euclid(86_400);
        let secs_of_day = s_i64.rem_euclid(86_400);
        let date = Date::from_days_since_unix_epoch(days)?;
        let secs_of_day = u32::try_from(secs_of_day).map_err(|_| DateError::InvalidDate)?;
        let nanos = u32::try_from(n).map_err(|_| DateError::InvalidDate)?;
        let time =
            Time::from_seconds_nanos(secs_of_day, nanos).map_err(|_| DateError::InvalidDate)?;
        Ok(DateTime { date, time })
    }

    /// Seconds since Unix epoch (1970-01-01T00:00:00Z).
    #[inline]
    #[must_use]
    pub fn unix_timestamp(self) -> i64 {
        let days = self.date.days_since_unix_epoch();
        let day_secs = i64::from(self.time.seconds_since_midnight());
        days * 86_400 + day_secs
    }

    /// Nanoseconds since Unix epoch, as i128.
    #[inline]
    #[must_use]
    pub fn unix_timestamp_nanos(self) -> i128 {
        i128::from(self.unix_timestamp()) * 1_000_000_000 + i128::from(self.time.nanosecond)
    }

    /// Add a duration, returning a new `DateTime` (or `OutOfRange` on overflow).
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if the resulting timestamp cannot be
    /// represented.
    pub fn add_duration(self, dur: Duration) -> Result<DateTime, DateError> {
        let t = self
            .unix_timestamp_nanos()
            .checked_add(dur.total_nanos())
            .ok_or(DateError::OutOfRange)?;
        let secs = t.div_euclid(1_000_000_000);
        let nanos = t.rem_euclid(1_000_000_000);
        let secs = i64::try_from(secs).map_err(|_| DateError::OutOfRange)?;
        let nanos = i32::try_from(nanos).map_err(|_| DateError::InvalidDate)?;
        DateTime::from_unix_timestamp(secs, nanos)
    }

    /// Difference between two instants (self - other).
    #[inline]
    #[must_use]
    pub fn difference(self, other: DateTime) -> Duration {
        Duration::nanoseconds(self.unix_timestamp_nanos() - other.unix_timestamp_nanos())
    }

    /// Get the current UTC `DateTime` (requires `std` feature).
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if the system clock is outside the
    /// range supported by [`DateTime`].
    #[cfg(feature = "std")]
    pub fn now_utc() -> Result<Self, DateError> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(dur) => {
                let secs = i64::try_from(dur.as_secs()).map_err(|_| DateError::OutOfRange)?;
                let nanos =
                    i32::try_from(dur.subsec_nanos()).map_err(|_| DateError::InvalidDate)?;
                DateTime::from_unix_timestamp(secs, nanos)
            }
            Err(e) => {
                let dur = e.duration();
                let secs = i64::try_from(dur.as_secs()).map_err(|_| DateError::OutOfRange)?;
                let nanos =
                    i32::try_from(dur.subsec_nanos()).map_err(|_| DateError::InvalidDate)?;
                DateTime::from_unix_timestamp(-secs, -nanos)
            }
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ISO 8601 / RFC 3339 UTC: YYYY-MM-DDTHH:MM:SS[.frac]Z
        write!(f, "{}T{}Z", self.date, self.time)
    }
}

impl FromStr for DateTime {
    type Err = ();

    /// Parse "YYYY-MM-DDTHH:MM:SS[.fffffffff]Z" (UTC only).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_suffix('Z')
            .or_else(|| s.strip_suffix('z'))
            .ok_or(())?;
        let (date_str, time_str) = s.split_once('T').or_else(|| s.split_once(' ')).ok_or(())?;
        let date = date_str.parse::<Date>().map_err(|_| ())?;
        let time = time_str.parse::<Time>().map_err(|_| ())?;
        Ok(DateTime { date, time })
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unix_timestamp_nanos()
            .cmp(&other.unix_timestamp_nanos())
    }
}

/// Error constructing a UTC offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtcOffsetError {
    OutOfRange,
}

/// Fixed offset from UTC, in seconds (e.g. +02:00).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtcOffset {
    seconds: i32,
}

impl UtcOffset {
    /// Construct from a total number of seconds, roughly in [-24h, +24h].
    ///
    /// # Errors
    ///
    /// Returns [`UtcOffsetError::OutOfRange`] if `seconds` is outside the
    /// inclusive range `-86_400..=86_400`.
    pub fn from_seconds(seconds: i32) -> Result<Self, UtcOffsetError> {
        // Rough sanity bounds: [-24h, +24h].
        if !(-86_400..=86_400).contains(&seconds) {
            return Err(UtcOffsetError::OutOfRange);
        }
        Ok(UtcOffset { seconds })
    }

    /// Construct from hours and minutes, with `sign_positive` sign.
    ///
    /// For example:
    /// - `from_hours_minutes(true, 2, 0)` => +02:00
    /// - `from_hours_minutes(false, 5, 30)` => -05:30
    ///
    /// # Errors
    ///
    /// Returns [`UtcOffsetError::OutOfRange`] if `hours` exceeds 23 or
    /// `minutes` exceeds 59.
    pub fn from_hours_minutes(
        sign_positive: bool,
        hours: u8,
        minutes: u8,
    ) -> Result<Self, UtcOffsetError> {
        if hours > 23 || minutes > 59 {
            return Err(UtcOffsetError::OutOfRange);
        }
        let total = i32::from(hours) * 3600 + i32::from(minutes) * 60;
        let total = if sign_positive { total } else { -total };
        Self::from_seconds(total)
    }

    #[inline]
    #[must_use]
    pub fn as_seconds(self) -> i32 {
        self.seconds
    }

    #[inline]
    #[must_use]
    pub fn is_utc(self) -> bool {
        self.seconds == 0
    }
}

impl PartialOrd for UtcOffset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UtcOffset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seconds.cmp(&other.seconds)
    }
}

impl fmt::Display for UtcOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut secs = self.seconds;
        let sign = if secs >= 0 { '+' } else { '-' };
        if secs < 0 {
            secs = -secs;
        }
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        write!(f, "{sign}{hours:02}:{minutes:02}")
    }
}

/// Date-time with a fixed offset from UTC (RFC 3339-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetDateTime {
    /// Instant in UTC.
    pub utc: DateTime,
    /// Fixed offset.
    pub offset: UtcOffset,
}

impl OffsetDateTime {
    /// Construct from a UTC instant and an offset.
    #[must_use]
    pub fn from_utc(utc: DateTime, offset: UtcOffset) -> Self {
        OffsetDateTime { utc, offset }
    }

    /// Construct from a local date+time with offset, converting to UTC.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if conversion to UTC exceeds the
    /// supported date range.
    pub fn from_local(date: Date, time: Time, offset: UtcOffset) -> Result<Self, DateError> {
        let local = DateTime::new(date, time);
        let utc = local.add_duration(Duration::seconds(-i64::from(offset.as_seconds())))?;
        Ok(OffsetDateTime { utc, offset })
    }

    /// Local date/time as seen in this offset.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if applying the offset exceeds the
    /// supported date range.
    pub fn to_local(&self) -> Result<DateTime, DateError> {
        self.utc
            .add_duration(Duration::seconds(i64::from(self.offset.as_seconds())))
    }

    /// Seconds since Unix epoch (1970-01-01T00:00:00Z).
    #[inline]
    #[must_use]
    pub fn unix_timestamp(&self) -> i64 {
        self.utc.unix_timestamp()
    }

    /// Nanoseconds since Unix epoch.
    #[inline]
    #[must_use]
    pub fn unix_timestamp_nanos(&self) -> i128 {
        self.utc.unix_timestamp_nanos()
    }

    /// Add a duration, keeping the same offset.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::OutOfRange`] if the resulting instant exceeds the
    /// supported date range.
    pub fn add_duration(&self, dur: Duration) -> Result<Self, DateError> {
        let utc = self.utc.add_duration(dur)?;
        Ok(OffsetDateTime {
            utc,
            offset: self.offset,
        })
    }

    /// Difference between two instants (self - other).
    #[inline]
    #[must_use]
    pub fn difference(&self, other: OffsetDateTime) -> Duration {
        self.utc.difference(other.utc)
    }
}

impl fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // RFC 3339: local "YYYY-MM-DDTHH:MM:SS[.frac]" + offset.
        let local = self
            .to_local()
            .expect("OffsetDateTime local representation out of range");
        write!(f, "{}T{}", local.date, local.time)?;
        if self.offset.is_utc() {
            write!(f, "Z")
        } else {
            write!(f, "{}", self.offset)
        }
    }
}

impl FromStr for OffsetDateTime {
    type Err = ();

    /// Parse RFC 3339-style:
    /// `YYYY-MM-DDTHH:MM:SS[.fffffffff][Z|±HH:MM]`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (date_part, rest) = s.split_once('T').or_else(|| s.split_once(' ')).ok_or(())?;
        let date: Date = date_part.parse().map_err(|_| ())?;

        // Parse time + offset.
        let (time_part, offset_part) = if rest.ends_with('Z') || rest.ends_with('z') {
            (&rest[..rest.len() - 1], "Z")
        } else {
            let idx = rest.rfind(['+', '-']).ok_or(())?;
            (&rest[..idx], &rest[idx..])
        };

        let time: Time = time_part.parse().map_err(|_| ())?;
        let offset = parse_rfc3339_offset(offset_part).map_err(|_| ())?;
        OffsetDateTime::from_local(date, time, offset).map_err(|_| ())
    }
}

impl PartialOrd for OffsetDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OffsetDateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.utc.cmp(&other.utc)
    }
}

// ===== Internal helpers =====

const POW10_U32: [u32; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

fn parse_i32_bytes(bytes: &[u8]) -> Option<i32> {
    if bytes.is_empty() {
        return None;
    }
    let mut idx = 0;
    let mut neg = false;
    match bytes[0] {
        b'+' => idx = 1,
        b'-' => {
            idx = 1;
            neg = true;
        }
        _ => {}
    }
    if idx == bytes.len() {
        return None;
    }

    let limit: i64 = if neg {
        i64::from(i32::MAX) + 1
    } else {
        i64::from(i32::MAX)
    };
    let mut val: i64 = 0;
    for &b in &bytes[idx..] {
        if !b.is_ascii_digit() {
            return None;
        }
        let digit = i64::from(b - b'0');
        if val > limit / 10 || (val == limit / 10 && digit > limit % 10) {
            return None;
        }
        val = val * 10 + digit;
    }

    if neg {
        val = -val;
    }
    i32::try_from(val).ok()
}

fn parse_u32_bytes(bytes: &[u8], max: u32) -> Option<u32> {
    if bytes.is_empty() {
        return None;
    }
    let mut val: u32 = 0;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        let digit = u32::from(b - b'0');
        if val > max / 10 || (val == max / 10 && digit > max % 10) {
            return None;
        }
        val = val * 10 + digit;
    }
    Some(val)
}

fn parse_fraction_nanos(bytes: &[u8]) -> Option<u32> {
    let len = bytes.len();
    if len == 0 || len > 9 {
        return None;
    }
    let mut val: u32 = 0;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        val = val * 10 + u32::from(b - b'0');
    }
    let scale = 9 - len;
    Some(val * POW10_U32[scale])
}

/// Errors parsing an RFC 3339 UTC offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rfc3339OffsetError {
    InvalidFormat,
    OutOfRange,
}

/// Parse an RFC 3339 UTC offset such as `Z`, `+02:30`, or `-0700`.
///
/// # Errors
///
/// Returns [`Rfc3339OffsetError::InvalidFormat`] if the string is malformed,
/// or [`Rfc3339OffsetError::OutOfRange`] if the parsed offset exceeds the
/// supported range.
pub fn parse_rfc3339_offset(s: &str) -> Result<UtcOffset, Rfc3339OffsetError> {
    if s == "Z" || s == "z" {
        return UtcOffset::from_seconds(0).map_err(|_| Rfc3339OffsetError::OutOfRange);
    }
    let bytes = s.as_bytes();
    if bytes.len() < 3 {
        return Err(Rfc3339OffsetError::InvalidFormat);
    }
    let sign_positive = match bytes[0] {
        b'+' => true,
        b'-' => false,
        _ => return Err(Rfc3339OffsetError::InvalidFormat),
    };
    let body = &bytes[1..];

    let mut colon = None;
    for (idx, &b) in body.iter().enumerate() {
        if b == b':' {
            colon = Some(idx);
            break;
        }
    }

    let (h_bytes, m_bytes) = if let Some(colon_idx) = colon {
        let h = &body[..colon_idx];
        let m = &body[colon_idx + 1..];
        if h.is_empty() || h.len() > 2 || m.len() > 2 {
            return Err(Rfc3339OffsetError::InvalidFormat);
        }
        (h, m)
    } else if body.len() == 2 {
        (&body[..2], &[][..])
    } else if body.len() == 4 {
        (&body[..2], &body[2..])
    } else {
        return Err(Rfc3339OffsetError::InvalidFormat);
    };

    if h_bytes.len() > 2 || m_bytes.len() > 2 {
        return Err(Rfc3339OffsetError::InvalidFormat);
    }

    let hours = parse_u32_bytes(h_bytes, 99)
        .and_then(|value| u8::try_from(value).ok())
        .ok_or(Rfc3339OffsetError::InvalidFormat)?;
    let minutes = if m_bytes.is_empty() {
        0
    } else {
        parse_u32_bytes(m_bytes, 99)
            .and_then(|value| u8::try_from(value).ok())
            .ok_or(Rfc3339OffsetError::InvalidFormat)?
    };
    UtcOffset::from_hours_minutes(sign_positive, hours, minutes)
        .map_err(|_| Rfc3339OffsetError::OutOfRange)
}

fn is_leap_year(year: i32) -> bool {
    let century_candidate = year % 25 == 0;
    (year & if century_candidate { 15 } else { 3 }) == 0
}

fn days_in_month(year: i32, month: u8) -> u8 {
    if month == 2 {
        return if is_leap_year(year) { 29 } else { 28 };
    }
    if !(1..=12).contains(&month) {
        return 0;
    }
    // Branch-free month length for all non-February months.
    (month ^ (month >> 3)) | 0b1_1110
}

// Modified Neri-Schneider inverse (civil → days), as documented by Ben Joffe.
// Returns days since Unix epoch for a given Gregorian date.
#[inline]
fn days_from_civil(y: i32, m: u8, d: u8) -> i64 {
    // Large enough so shifted years are non-negative for the full i32 range.
    const S: i64 = 5_368_710;
    const YEAR_SHIFT: i64 = 400 * S;
    const RATA_SHIFT: i64 = 719_468 + 146_097 * S + 1;

    let bump = m <= 2;
    let year = i64::from(y) + YEAR_SHIFT - i64::from(bump);
    let cent = year / 100;
    let phase = if bump { 8_829 } else { -2_919 };

    let y_days = year * 365 + year / 4 - cent + cent / 4;
    let m_days = (979 * i64::from(m) + phase) / 32;
    y_days + m_days + i64::from(d) - RATA_SHIFT
}
