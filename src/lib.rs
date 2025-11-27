#![cfg_attr(not(feature = "std"), no_std)]

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
//!   - `OffsetDateTime`: "YYYY-MM-DDTHH:MM:SS[.fffffffff][Z|±HH:MM]" (RFC 3339 subset).
//! - `DateTime::now_utc()` when the `std` feature is enabled.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

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
    pub fn from_days_since_unix_epoch(days: i64) -> Result<Self, DateError> {
        // Constants from the article (x64 version).
        const ERAS: i64 = 4_726_498_270;
        const D_SHIFT: i64 = 146_097 * ERAS - 719_469;
        const Y_SHIFT: i64 = 400 * ERAS - 1;
        const C1: u64 = 505_054_698_555_331;
        const C2: u64 = 50_504_432_782_230_121;
        const C3: u64 = 8_619_973_866_219_416;

        let rev: i64 = D_SHIFT - days;

        // 64x64 → high 64 bit multiplies via u128 with explicit u64 casts.
        let cen: i64 = (((rev as u64 as u128) * (C1 as u128)) >> 64) as i64;
        let jul: i64 = rev + cen - cen / 4;

        let num: u128 = (jul as u64 as u128) * (C2 as u128);
        let yrs: i64 = Y_SHIFT - ((num >> 64) as i64);
        let low: u64 = num as u64;
        let ypt: i64 = ((782_432u128 * low as u128) >> 64) as i64;

        let bump = ypt < 126_464;
        let shift: i64 = if bump { 191_360 } else { 977_792 };

        let n: i64 = (yrs % 4) * 512 + shift - ypt;

        let d: i64 = (((((n as u64) & 0xFFFF) as u128) * (C3 as u128)) >> 64) as i64;

        let day_i: i64 = d + 1;
        let month_i: i64 = n / 65_536;
        let year_i: i64 = yrs + if bump { 1 } else { 0 };

        if !(i32::MIN as i64..=i32::MAX as i64).contains(&year_i) {
            return Err(DateError::OutOfRange);
        }
        let year = year_i as i32;
        let month = month_i as u8;
        let day = day_i as u8;

        // Extra safety: validate
        if Date::from_ymd(year, month, day).is_err() {
            return Err(DateError::InvalidDate);
        }

        Ok(Date { year, month, day })
    }

    /// Convert to days since Unix epoch (1970-01-01 = 0).
    ///
    /// This uses Howard Hinnant's well-known constant-time civil→days
    /// algorithm, which is exact for the proleptic Gregorian calendar.
    pub fn days_since_unix_epoch(self) -> i64 {
        days_from_civil(self.year, self.month, self.day)
    }

    /// Day of week (Monday = 1).
    ///
    /// Unix epoch 1970-01-01 was a Thursday, so we just offset.
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
    pub fn ordinal(self) -> u16 {
        let month = self.month;
        let day = self.day as u16;
        const CUM_DAYS: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let mut ord = CUM_DAYS[(month - 1) as usize] + day;
        if month > 2 && is_leap_year(self.year) {
            ord += 1;
        }
        ord
    }

    /// Add a number of days, returning a new `Date` or `OutOfRange`.
    pub fn add_days(self, days: i64) -> Result<Date, DateError> {
        let base = self.days_since_unix_epoch();
        Date::from_days_since_unix_epoch(base + days)
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
        let mut parts = s.splitn(3, '-');
        let y = parts
            .next()
            .ok_or(DateError::InvalidDate)?
            .parse::<i32>()
            .map_err(|_| DateError::InvalidDate)?;
        let m = parts
            .next()
            .ok_or(DateError::InvalidDate)?
            .parse::<u8>()
            .map_err(|_| DateError::InvalidDate)?;
        let d = parts
            .next()
            .ok_or(DateError::InvalidDate)?
            .parse::<u8>()
            .map_err(|_| DateError::InvalidDate)?;
        if parts.next().is_some() {
            return Err(DateError::InvalidDate);
        }
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
    pub fn seconds_since_midnight(self) -> u32 {
        (self.hour as u32) * 3600 + (self.minute as u32) * 60 + (self.second as u32)
    }

    /// Total nanoseconds since midnight.
    pub fn nanos_since_midnight(self) -> u64 {
        self.seconds_since_midnight() as u64 * 1_000_000_000 + self.nanosecond as u64
    }

    /// Build from seconds and nanoseconds since midnight.
    pub fn from_seconds_nanos(secs: u32, nanos: u32) -> Result<Self, TimeError> {
        if secs >= 86_400 || nanos >= 1_000_000_000 {
            return Err(TimeError::InvalidTime);
        }
        let hour = (secs / 3600) as u8;
        let rem = secs % 3600;
        let minute = (rem / 60) as u8;
        let second = (rem % 60) as u8;
        Time::from_hms_nano(hour, minute, second, nanos)
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
                frac[i] = b'0' + (ns % 10) as u8;
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
        let (hms, frac) = match s.split_once('.') {
            Some((h, f)) => (h, Some(f)),
            None => (s, None),
        };
        let mut parts = hms.splitn(3, ':');
        let h = parts
            .next()
            .ok_or(TimeError::InvalidTime)?
            .parse::<u8>()
            .map_err(|_| TimeError::InvalidTime)?;
        let m = parts
            .next()
            .ok_or(TimeError::InvalidTime)?
            .parse::<u8>()
            .map_err(|_| TimeError::InvalidTime)?;
        let sec = parts
            .next()
            .ok_or(TimeError::InvalidTime)?
            .parse::<u8>()
            .map_err(|_| TimeError::InvalidTime)?;
        if parts.next().is_some() {
            return Err(TimeError::InvalidTime);
        }

        let nanos = if let Some(fr) = frac {
            if fr.is_empty() || fr.len() > 9 {
                return Err(TimeError::InvalidTime);
            }
            // Interpret fractional seconds with up to 9 digits, padding
            // with zeros on the right.
            let mut nanos: u32 = 0;
            let mut factor: u32 = 100_000_000; // 10^(9-1)
            for ch in fr.chars() {
                if !ch.is_ascii_digit() {
                    return Err(TimeError::InvalidTime);
                }
                let digit = (ch as u8 - b'0') as u32;
                nanos = nanos
                    .checked_add(digit * factor)
                    .ok_or(TimeError::InvalidTime)?;
                factor /= 10;
            }
            nanos
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

    pub fn seconds(secs: i64) -> Duration {
        Duration {
            nanos: (secs as i128) * 1_000_000_000,
        }
    }

    pub fn milliseconds(ms: i64) -> Duration {
        Duration {
            nanos: (ms as i128) * 1_000_000,
        }
    }

    pub fn microseconds(us: i64) -> Duration {
        Duration {
            nanos: (us as i128) * 1_000,
        }
    }

    pub fn nanoseconds(ns: i128) -> Duration {
        Duration { nanos: ns }
    }

    pub fn total_seconds(self) -> f64 {
        self.nanos as f64 / 1_000_000_000.0
    }

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
    pub fn new(date: Date, time: Time) -> DateTime {
        DateTime { date, time }
    }

    /// Build from Unix timestamp (seconds since 1970-01-01T00:00:00Z)
    /// plus an additional nanoseconds offset (can be negative or >1e9).
    pub fn from_unix_timestamp(secs: i64, nanos: i32) -> Result<DateTime, DateError> {
        // Normalize (secs, nanos) pair.
        let mut s = secs as i128;
        let mut n = nanos as i128;
        s += n.div_euclid(1_000_000_000);
        n = n.rem_euclid(1_000_000_000);
        let s_i64 = s as i64;

        let days = s_i64.div_euclid(86_400);
        let secs_of_day = s_i64.rem_euclid(86_400);
        let date = Date::from_days_since_unix_epoch(days)?;
        let time = Time::from_seconds_nanos(secs_of_day as u32, n as u32)
            .map_err(|_| DateError::InvalidDate)?;
        Ok(DateTime { date, time })
    }

    /// Seconds since Unix epoch (1970-01-01T00:00:00Z).
    pub fn unix_timestamp(self) -> i64 {
        let days = self.date.days_since_unix_epoch();
        let day_secs = self.time.seconds_since_midnight() as i64;
        days * 86_400 + day_secs
    }

    /// Nanoseconds since Unix epoch, as i128.
    pub fn unix_timestamp_nanos(self) -> i128 {
        self.unix_timestamp() as i128 * 1_000_000_000 + self.time.nanosecond as i128
    }

    /// Add a duration, returning a new `DateTime` (or `OutOfRange` on overflow).
    pub fn add_duration(self, dur: Duration) -> Result<DateTime, DateError> {
        let t = self.unix_timestamp_nanos() + dur.total_nanos();
        let secs = t.div_euclid(1_000_000_000);
        let nanos = t.rem_euclid(1_000_000_000);
        DateTime::from_unix_timestamp(secs as i64, nanos as i32)
    }

    /// Difference between two instants (self - other).
    pub fn difference(self, other: DateTime) -> Duration {
        Duration::nanoseconds(self.unix_timestamp_nanos() - other.unix_timestamp_nanos())
    }

    /// Get the current UTC `DateTime` (requires `std` feature).
    #[cfg(feature = "std")]
    pub fn now_utc() -> Result<Self, DateError> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(dur) => {
                DateTime::from_unix_timestamp(dur.as_secs() as i64, dur.subsec_nanos() as i32)
            }
            Err(e) => {
                let dur = e.duration();
                let secs = dur.as_secs() as i64;
                let nanos = dur.subsec_nanos() as i32;
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
    pub fn from_hours_minutes(
        sign_positive: bool,
        hours: u8,
        minutes: u8,
    ) -> Result<Self, UtcOffsetError> {
        if hours > 23 || minutes > 59 {
            return Err(UtcOffsetError::OutOfRange);
        }
        let total = (hours as i32) * 3600 + (minutes as i32) * 60;
        let total = if sign_positive { total } else { -total };
        Self::from_seconds(total)
    }

    pub fn as_seconds(self) -> i32 {
        self.seconds
    }

    pub fn is_utc(self) -> bool {
        self.seconds == 0
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
        write!(f, "{}{:02}:{:02}", sign, hours, minutes)
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
    pub fn from_utc(utc: DateTime, offset: UtcOffset) -> Self {
        OffsetDateTime { utc, offset }
    }

    /// Construct from a local date+time with offset, converting to UTC.
    pub fn from_local(date: Date, time: Time, offset: UtcOffset) -> Result<Self, DateError> {
        let local = DateTime::new(date, time);
        let utc = local.add_duration(Duration::seconds(-(offset.as_seconds() as i64)))?;
        Ok(OffsetDateTime { utc, offset })
    }

    /// Local date/time as seen in this offset.
    pub fn to_local(&self) -> Result<DateTime, DateError> {
        self.utc
            .add_duration(Duration::seconds(self.offset.as_seconds() as i64))
    }

    /// Seconds since Unix epoch (1970-01-01T00:00:00Z).
    pub fn unix_timestamp(&self) -> i64 {
        self.utc.unix_timestamp()
    }

    /// Nanoseconds since Unix epoch.
    pub fn unix_timestamp_nanos(&self) -> i128 {
        self.utc.unix_timestamp_nanos()
    }

    /// Add a duration, keeping the same offset.
    pub fn add_duration(&self, dur: Duration) -> Result<Self, DateError> {
        let utc = self.utc.add_duration(dur)?;
        Ok(OffsetDateTime {
            utc,
            offset: self.offset,
        })
    }

    /// Difference between two instants (self - other).
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
    /// "YYYY-MM-DDTHH:MM:SS[.fffffffff][Z|±HH:MM]"
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

fn parse_rfc3339_offset(s: &str) -> Result<UtcOffset, ()> {
    if s == "Z" || s == "z" {
        return UtcOffset::from_seconds(0).map_err(|_| ());
    }
    let bytes = s.as_bytes();
    if bytes.len() < 3 {
        return Err(());
    }
    let sign_positive = match bytes[0] {
        b'+' => true,
        b'-' => false,
        _ => return Err(()),
    };
    let body = &s[1..];
    let (h_str, m_str) = if let Some(colon) = body.find(':') {
        (&body[..colon], &body[colon + 1..])
    } else if body.len() == 2 {
        (&body[..2], "0")
    } else if body.len() == 4 {
        (&body[..2], &body[2..])
    } else {
        return Err(());
    };
    if h_str.is_empty() || h_str.len() > 2 || m_str.len() > 2 {
        return Err(());
    }
    let hours: u8 = h_str.parse().map_err(|_| ())?;
    let minutes: u8 = if m_str.is_empty() {
        0
    } else {
        m_str.parse().map_err(|_| ())?
    };
    UtcOffset::from_hours_minutes(sign_positive, hours, minutes).map_err(|_| ())
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

// Howard Hinnant's civil-from-days/inverse algorithm.
// Returns days since Unix epoch for a given Gregorian date.
fn days_from_civil(y: i32, m: u8, d: u8) -> i64 {
    let y = y as i64;
    let m = m as i64;
    let d = d as i64;
    let y0 = y - if m <= 2 { 1 } else { 0 };
    let era = if y0 >= 0 { y0 / 400 } else { (y0 - 399) / 400 };
    let yoe = y0 - era * 400; // [0, 399]
    let mp = m + if m > 2 { -3 } else { 9 }; // March=0,...,Feb=11
    let doy = (153 * mp + 2) / 5 + d - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146_097 + doe - 719_468
}

// ===== Tests (use std, so fine even with no_std library) =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_epoch_and_neighbors() {
        let d0 = Date::from_days_since_unix_epoch(0).unwrap();
        assert_eq!(d0.year, 1970);
        assert_eq!(d0.month, 1);
        assert_eq!(d0.day, 1);

        let d1 = Date::from_days_since_unix_epoch(1).unwrap();
        assert_eq!(d1.to_string(), "1970-01-02");

        let dm1 = Date::from_days_since_unix_epoch(-1).unwrap();
        assert_eq!(dm1.to_string(), "1969-12-31");
    }

    #[test]
    fn days_round_trip() {
        let cases = [
            (1970, 1, 1),
            (1970, 1, 2),
            (1969, 12, 31),
            (2000, 2, 29),
            (2000, 3, 1),
            (1900, 3, 1),
            (2400, 2, 29),
        ];
        for &(y, m, d) in &cases {
            let date = Date::from_ymd(y, m, d).unwrap();
            let days = date.days_since_unix_epoch();
            let round = Date::from_days_since_unix_epoch(days).unwrap();
            assert_eq!(date, round);
        }
    }

    #[test]
    fn datetime_unix_round_trip() {
        let date = Date::from_ymd(2024, 5, 17).unwrap();
        let time = Time::from_hms_nano(12, 34, 56, 123_456_789).unwrap();
        let dt = DateTime::new(date, time);
        let secs = dt.unix_timestamp();
        let nanos = dt.time.nanosecond as i32;
        let rt = DateTime::from_unix_timestamp(secs, nanos).unwrap();
        assert_eq!(dt, rt);
    }

    #[test]
    fn duration_add_sub() {
        let date = Date::from_ymd(2020, 1, 1).unwrap();
        let time = Time::from_hms_nano(0, 0, 0, 0).unwrap();
        let dt = DateTime::new(date, time);
        let dur = Duration::seconds(86_400); // 1 day
        let dt2 = dt.add_duration(dur).unwrap();
        assert_eq!(dt2.date.to_string(), "2020-01-02");
        assert_eq!(dt2.time.to_string(), "00:00:00");

        let diff = dt2.difference(dt);
        assert_eq!(diff, dur);
    }

    #[test]
    fn parse_and_display_basic() {
        let d: Date = "2023-11-05".parse().unwrap();
        assert_eq!(d.to_string(), "2023-11-05");

        let t: Time = "23:59:59.001".parse().unwrap();
        assert_eq!(t.to_string(), "23:59:59.001");

        let dt: DateTime = "2023-11-05T23:59:59.001Z".parse().unwrap();
        assert_eq!(dt.to_string(), "2023-11-05T23:59:59.001Z");
    }

    #[test]
    fn offset_datetime_rfc3339() {
        let s = "2023-11-05T23:59:59.5+02:00";
        let odt: OffsetDateTime = s.parse().unwrap();
        assert_eq!(odt.to_string(), s);

        let odt_z: OffsetDateTime = "2023-11-05T23:59:59Z".parse().unwrap();
        assert_eq!(odt_z.to_string(), "2023-11-05T23:59:59Z");
    }

    #[test]
    fn date_weekday_and_ordinal() {
        let monday = Date::from_ymd(2023, 11, 6).unwrap();
        assert_eq!(monday.weekday(), Weekday::Monday);
        assert_eq!(monday.ordinal(), 310);

        let leap = Date::from_ymd(2020, 3, 1).unwrap();
        assert_eq!(leap.weekday(), Weekday::Sunday);
        assert_eq!(leap.ordinal(), 61);
    }

    #[test]
    fn time_fractional_and_nanos() {
        let t: Time = "12:34:56.123450700".parse().unwrap();
        assert_eq!(t.nanosecond, 123_450_700);
        assert_eq!(t.to_string(), "12:34:56.1234507");
        assert_eq!(t.seconds_since_midnight(), 45_296);
        assert_eq!(t.nanos_since_midnight(), 45_296_123_450_700);
    }

    #[test]
    fn time_parse_rejects_invalid_fraction() {
        assert!(matches!(
            "12:00:00.".parse::<Time>(),
            Err(TimeError::InvalidTime)
        ));
        assert!(matches!(
            "12:00:00.1234567890".parse::<Time>(),
            Err(TimeError::InvalidTime)
        ));
    }

    #[test]
    fn rfc3339_offset_variants() {
        let with_colon = parse_rfc3339_offset("+02:30").unwrap();
        assert_eq!(with_colon.as_seconds(), 2 * 3600 + 30 * 60);

        let compact = parse_rfc3339_offset("+0230").unwrap();
        assert_eq!(compact.as_seconds(), 2 * 3600 + 30 * 60);

        let hour_only = parse_rfc3339_offset("-07").unwrap();
        assert_eq!(hour_only.as_seconds(), -7 * 3600);

        assert!(parse_rfc3339_offset("invalid").is_err());
    }

    #[test]
    fn offset_datetime_local_conversion_and_duration() {
        let date = Date::from_ymd(2021, 1, 2).unwrap();
        let time = Time::from_hms_nano(3, 4, 5, 999_999_999).unwrap();
        let offset = UtcOffset::from_hours_minutes(false, 5, 45).unwrap(); // UTC-05:45

        let odt = OffsetDateTime::from_local(date, time, offset).unwrap();
        assert_eq!(odt.offset, offset);

        let local = odt.to_local().unwrap();
        assert_eq!(local.date, date);
        assert_eq!(local.time, time);

        let later = odt.add_duration(Duration::seconds(30)).unwrap();
        assert_eq!(later.difference(odt), Duration::seconds(30));
    }
}
