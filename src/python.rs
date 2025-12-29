//! Python bindings for fasttime using PyO3.

#![cfg(feature = "python")]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::{Date, DateTime, Duration, OffsetDateTime, Time, UtcOffset, Weekday as RustWeekday};

// ===== Weekday =====

#[pyclass(name = "Weekday", module = "fasttime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyWeekday(RustWeekday);

#[pymethods]
impl PyWeekday {
    #[classattr]
    const MONDAY: PyWeekday = PyWeekday(RustWeekday::Monday);
    #[classattr]
    const TUESDAY: PyWeekday = PyWeekday(RustWeekday::Tuesday);
    #[classattr]
    const WEDNESDAY: PyWeekday = PyWeekday(RustWeekday::Wednesday);
    #[classattr]
    const THURSDAY: PyWeekday = PyWeekday(RustWeekday::Thursday);
    #[classattr]
    const FRIDAY: PyWeekday = PyWeekday(RustWeekday::Friday);
    #[classattr]
    const SATURDAY: PyWeekday = PyWeekday(RustWeekday::Saturday);
    #[classattr]
    const SUNDAY: PyWeekday = PyWeekday(RustWeekday::Sunday);

    /// Get the weekday number from Monday (1-7).
    #[pyo3(name = "number_from_monday")]
    fn number_from_monday(&self) -> u8 {
        self.0.number_from_monday()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }
}

// ===== PyDate =====

#[pyclass(name = "Date", module = "fasttime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyDate(Date);

#[pymethods]
impl PyDate {
    /// Create a new Date from year, month, and day.
    ///
    /// Args:
    ///     year: The year (i32).
    ///     month: The month (1-12).
    ///     day: The day (1-31).
    ///
    /// Returns:
    ///     Date: A new Date instance.
    ///
    /// Raises:
    ///     ValueError: If the date is invalid.
    #[new]
    fn new(year: i32, month: u8, day: u8) -> PyResult<Self> {
        Date::from_ymd(year, month, day)
            .map(PyDate)
            .map_err(|e| PyValueError::new_err(format!("Invalid date: {:?}", e)))
    }

    #[getter]
    fn year(&self) -> i32 {
        self.0.year
    }

    #[getter]
    fn month(&self) -> u8 {
        self.0.month
    }

    #[getter]
    fn day(&self) -> u8 {
        self.0.day
    }

    /// Create a Date from days since Unix epoch (1970-01-01).
    ///
    /// Args:
    ///     days: Days since Unix epoch.
    ///
    /// Returns:
    ///     Date: A new Date instance.
    ///
    /// Raises:
    ///     ValueError: If the date is out of range.
    #[classmethod]
    #[pyo3(name = "from_days_since_unix_epoch")]
    fn from_days_since_unix_epoch(_cls: &Bound<'_, PyType>, days: i64) -> PyResult<Self> {
        Date::from_days_since_unix_epoch(days)
            .map(PyDate)
            .map_err(|e| PyValueError::new_err(format!("Invalid date: {:?}", e)))
    }

    /// Convert to days since Unix epoch (1970-01-01).
    #[pyo3(name = "days_since_unix_epoch")]
    fn days_since_unix_epoch(&self) -> i64 {
        self.0.days_since_unix_epoch()
    }

    /// Get the weekday.
    #[pyo3(name = "weekday")]
    fn weekday(&self) -> PyWeekday {
        PyWeekday(self.0.weekday())
    }

    /// Get the day of the year (1-366).
    #[pyo3(name = "ordinal")]
    fn ordinal(&self) -> u16 {
        self.0.ordinal()
    }

    /// Add days to the date.
    ///
    /// Args:
    ///     days: Number of days to add (can be negative).
    ///
    /// Returns:
    ///     Date: A new Date instance.
    ///
    /// Raises:
    ///     ValueError: If the resulting date is out of range.
    #[pyo3(name = "add_days")]
    fn add_days(&self, days: i64) -> PyResult<Self> {
        self.0
            .add_days(days)
            .map(PyDate)
            .map_err(|e| PyValueError::new_err(format!("Date out of range: {:?}", e)))
    }

    /// Parse a date from ISO format (YYYY-MM-DD).
    ///
    /// Args:
    ///     s: The string to parse.
    ///
    /// Returns:
    ///     Date: A new Date instance.
    ///
    /// Raises:
    ///     ValueError: If the string is not a valid date.
    #[classmethod]
    #[pyo3(name = "parse")]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        s.parse::<Date>()
            .map(PyDate)
            .map_err(|e| PyValueError::new_err(format!("Invalid date string: {:?}", e)))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "Date(year={}, month={}, day={})",
            self.0.year, self.0.month, self.0.day
        )
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        use pyo3::basic::CompareOp;
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        // Using DefaultHasher is appropriate here. Like Python's hash(),
        // it provides DOS resistance and only needs to be consistent
        // within a single process, not across processes or restarts.
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

// ===== PyTime =====

#[pyclass(name = "Time", module = "fasttime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyTime(Time);

#[pymethods]
impl PyTime {
    /// Create a new Time.
    ///
    /// Args:
    ///     hour: Hour (0-23).
    ///     minute: Minute (0-59).
    ///     second: Second (0-59).
    ///     nanosecond: Nanosecond (0-999,999,999, default=0).
    ///
    /// Returns:
    ///     Time: A new Time instance.
    ///
    /// Raises:
    ///     ValueError: If the time is invalid.
    #[new]
    #[pyo3(signature = (hour, minute, second, nanosecond=0))]
    fn new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> PyResult<Self> {
        Time::from_hms_nano(hour, minute, second, nanosecond)
            .map(PyTime)
            .map_err(|e| PyValueError::new_err(format!("Invalid time: {:?}", e)))
    }

    #[getter]
    fn hour(&self) -> u8 {
        self.0.hour
    }

    #[getter]
    fn minute(&self) -> u8 {
        self.0.minute
    }

    #[getter]
    fn second(&self) -> u8 {
        self.0.second
    }

    #[getter]
    fn nanosecond(&self) -> u32 {
        self.0.nanosecond
    }

    /// Get total seconds since midnight (ignoring nanoseconds).
    #[pyo3(name = "seconds_since_midnight")]
    fn seconds_since_midnight(&self) -> u32 {
        self.0.seconds_since_midnight()
    }

    /// Get total nanoseconds since midnight.
    #[pyo3(name = "nanos_since_midnight")]
    fn nanos_since_midnight(&self) -> u64 {
        self.0.nanos_since_midnight()
    }

    /// Parse a time from ISO format (HH:MM:SS[.fffffffff]).
    ///
    /// Args:
    ///     s: The string to parse.
    ///
    /// Returns:
    ///     Time: A new Time instance.
    ///
    /// Raises:
    ///     ValueError: If the string is not a valid time.
    #[classmethod]
    #[pyo3(name = "parse")]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        s.parse::<Time>()
            .map(PyTime)
            .map_err(|e| PyValueError::new_err(format!("Invalid time string: {:?}", e)))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "Time(hour={}, minute={}, second={}, nanosecond={})",
            self.0.hour, self.0.minute, self.0.second, self.0.nanosecond
        )
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        use pyo3::basic::CompareOp;
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

// ===== PyDuration =====

#[pyclass(name = "Duration", module = "fasttime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyDuration(Duration);

#[pymethods]
impl PyDuration {
    /// Create a duration from seconds.
    #[classmethod]
    #[pyo3(name = "seconds")]
    fn seconds(_cls: &Bound<'_, PyType>, secs: i64) -> Self {
        PyDuration(Duration::seconds(secs))
    }

    /// Create a duration from milliseconds.
    #[classmethod]
    #[pyo3(name = "milliseconds")]
    fn milliseconds(_cls: &Bound<'_, PyType>, ms: i64) -> Self {
        PyDuration(Duration::milliseconds(ms))
    }

    /// Create a duration from microseconds.
    #[classmethod]
    #[pyo3(name = "microseconds")]
    fn microseconds(_cls: &Bound<'_, PyType>, us: i64) -> Self {
        PyDuration(Duration::microseconds(us))
    }

    /// Create a duration from nanoseconds.
    #[classmethod]
    #[pyo3(name = "nanoseconds")]
    fn nanoseconds(_cls: &Bound<'_, PyType>, ns: i128) -> Self {
        PyDuration(Duration::nanoseconds(ns))
    }

    /// Get total seconds as a float.
    #[pyo3(name = "total_seconds")]
    fn total_seconds(&self) -> f64 {
        self.0.total_seconds()
    }

    /// Get total nanoseconds as an integer.
    #[pyo3(name = "total_nanos")]
    fn total_nanos(&self) -> i128 {
        self.0.total_nanos()
    }

    fn __add__(&self, other: &Self) -> Self {
        PyDuration(self.0 + other.0)
    }

    fn __sub__(&self, other: &Self) -> Self {
        PyDuration(self.0 - other.0)
    }

    fn __neg__(&self) -> Self {
        PyDuration(-self.0)
    }

    fn __str__(&self) -> String {
        format!("Duration({} ns)", self.0.total_nanos())
    }

    fn __repr__(&self) -> String {
        format!("Duration.nanoseconds({})", self.0.total_nanos())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        use pyo3::basic::CompareOp;
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

// ===== PyDateTime =====

#[pyclass(name = "DateTime", module = "fasttime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyDateTime(DateTime);

#[pymethods]
impl PyDateTime {
    /// Create a DateTime from a Date and Time.
    ///
    /// Args:
    ///     date: A Date instance.
    ///     time: A Time instance.
    ///
    /// Returns:
    ///     DateTime: A new DateTime instance.
    #[new]
    fn new(date: &PyDate, time: &PyTime) -> Self {
        PyDateTime(DateTime::new(date.0, time.0))
    }

    #[getter]
    fn date(&self) -> PyDate {
        PyDate(self.0.date)
    }

    #[getter]
    fn time(&self) -> PyTime {
        PyTime(self.0.time)
    }

    /// Create a DateTime from Unix timestamp (seconds and nanoseconds).
    ///
    /// Args:
    ///     secs: Seconds since Unix epoch.
    ///     nanos: Additional nanoseconds (default=0).
    ///
    /// Returns:
    ///     DateTime: A new DateTime instance.
    ///
    /// Raises:
    ///     ValueError: If the timestamp is invalid.
    #[classmethod]
    #[pyo3(name = "from_unix_timestamp", signature = (secs, nanos=0))]
    fn from_unix_timestamp(_cls: &Bound<'_, PyType>, secs: i64, nanos: i32) -> PyResult<Self> {
        DateTime::from_unix_timestamp(secs, nanos)
            .map(PyDateTime)
            .map_err(|e| PyValueError::new_err(format!("Invalid timestamp: {:?}", e)))
    }

    /// Get Unix timestamp (seconds since Unix epoch).
    #[pyo3(name = "unix_timestamp")]
    fn unix_timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }

    /// Get Unix timestamp in nanoseconds.
    #[pyo3(name = "unix_timestamp_nanos")]
    fn unix_timestamp_nanos(&self) -> i128 {
        self.0.unix_timestamp_nanos()
    }

    /// Add a duration to this DateTime.
    ///
    /// Args:
    ///     dur: A Duration instance.
    ///
    /// Returns:
    ///     DateTime: A new DateTime instance.
    ///
    /// Raises:
    ///     ValueError: If the resulting datetime is out of range.
    #[pyo3(name = "add_duration")]
    fn add_duration(&self, dur: &PyDuration) -> PyResult<Self> {
        self.0
            .add_duration(dur.0)
            .map(PyDateTime)
            .map_err(|e| PyValueError::new_err(format!("DateTime out of range: {:?}", e)))
    }

    /// Calculate the difference between two DateTimes.
    ///
    /// Args:
    ///     other: Another DateTime instance.
    ///
    /// Returns:
    ///     Duration: The duration between the two datetimes (self - other).
    #[pyo3(name = "difference")]
    fn difference(&self, other: &PyDateTime) -> PyDuration {
        PyDuration(self.0.difference(other.0))
    }

    /// Get the current UTC DateTime (requires std feature).
    #[classmethod]
    #[pyo3(name = "now_utc")]
    fn now_utc(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        #[cfg(feature = "std")]
        {
            DateTime::now_utc()
                .map(PyDateTime)
                .map_err(|e| PyValueError::new_err(format!("Failed to get current time: {:?}", e)))
        }
        #[cfg(not(feature = "std"))]
        {
            Err(PyValueError::new_err(
                "now_utc() requires the 'std' feature",
            ))
        }
    }

    /// Parse a DateTime from ISO 8601 / RFC 3339 UTC format.
    ///
    /// Args:
    ///     s: The string to parse (e.g., "2024-01-01T12:00:00Z").
    ///
    /// Returns:
    ///     DateTime: A new DateTime instance.
    ///
    /// Raises:
    ///     ValueError: If the string is not a valid datetime.
    #[classmethod]
    #[pyo3(name = "parse")]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        s.parse::<DateTime>().map(PyDateTime).map_err(|_| {
            PyValueError::new_err(format!(
                "Invalid datetime string '{}'. Expected format: YYYY-MM-DDTHH:MM:SS[.fffffffff]Z",
                s
            ))
        })
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DateTime.parse('{}')", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        use pyo3::basic::CompareOp;
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

// ===== PyUtcOffset =====

#[pyclass(name = "UtcOffset", module = "fasttime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyUtcOffset(UtcOffset);

#[pymethods]
impl PyUtcOffset {
    /// Create a UtcOffset from total seconds.
    ///
    /// Args:
    ///     seconds: Total seconds offset from UTC (roughly -86400 to +86400).
    ///
    /// Returns:
    ///     UtcOffset: A new UtcOffset instance.
    ///
    /// Raises:
    ///     ValueError: If the offset is out of range.
    #[classmethod]
    #[pyo3(name = "from_seconds")]
    fn from_seconds(_cls: &Bound<'_, PyType>, seconds: i32) -> PyResult<Self> {
        UtcOffset::from_seconds(seconds)
            .map(PyUtcOffset)
            .map_err(|e| PyValueError::new_err(format!("Invalid offset: {:?}", e)))
    }

    /// Create a UtcOffset from hours and minutes.
    ///
    /// Args:
    ///     sign_positive: True for positive offset, False for negative.
    ///     hours: Hours component (0-23).
    ///     minutes: Minutes component (0-59).
    ///
    /// Returns:
    ///     UtcOffset: A new UtcOffset instance.
    ///
    /// Raises:
    ///     ValueError: If the offset is out of range.
    #[classmethod]
    #[pyo3(name = "from_hours_minutes")]
    fn from_hours_minutes(
        _cls: &Bound<'_, PyType>,
        sign_positive: bool,
        hours: u8,
        minutes: u8,
    ) -> PyResult<Self> {
        UtcOffset::from_hours_minutes(sign_positive, hours, minutes)
            .map(PyUtcOffset)
            .map_err(|e| PyValueError::new_err(format!("Invalid offset: {:?}", e)))
    }

    /// Get the offset as seconds.
    #[pyo3(name = "as_seconds")]
    fn as_seconds(&self) -> i32 {
        self.0.as_seconds()
    }

    /// Check if this is UTC (offset = 0).
    #[pyo3(name = "is_utc")]
    fn is_utc(&self) -> bool {
        self.0.is_utc()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("UtcOffset.from_seconds({})", self.0.as_seconds())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        use pyo3::basic::CompareOp;
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

// ===== PyOffsetDateTime =====

#[pyclass(name = "OffsetDateTime", module = "fasttime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyOffsetDateTime(OffsetDateTime);

#[pymethods]
impl PyOffsetDateTime {
    /// Create an OffsetDateTime from a UTC DateTime and offset.
    ///
    /// Args:
    ///     utc: A DateTime instance (in UTC).
    ///     offset: A UtcOffset instance.
    ///
    /// Returns:
    ///     OffsetDateTime: A new OffsetDateTime instance.
    #[classmethod]
    #[pyo3(name = "from_utc")]
    fn from_utc(_cls: &Bound<'_, PyType>, utc: &PyDateTime, offset: &PyUtcOffset) -> Self {
        PyOffsetDateTime(OffsetDateTime::from_utc(utc.0, offset.0))
    }

    /// Create an OffsetDateTime from local date, time, and offset.
    ///
    /// Args:
    ///     date: A Date instance (in local time).
    ///     time: A Time instance (in local time).
    ///     offset: A UtcOffset instance.
    ///
    /// Returns:
    ///     OffsetDateTime: A new OffsetDateTime instance.
    ///
    /// Raises:
    ///     ValueError: If the conversion fails.
    #[classmethod]
    #[pyo3(name = "from_local")]
    fn from_local(
        _cls: &Bound<'_, PyType>,
        date: &PyDate,
        time: &PyTime,
        offset: &PyUtcOffset,
    ) -> PyResult<Self> {
        OffsetDateTime::from_local(date.0, time.0, offset.0)
            .map(PyOffsetDateTime)
            .map_err(|e| PyValueError::new_err(format!("Invalid local datetime: {:?}", e)))
    }

    #[getter]
    fn utc(&self) -> PyDateTime {
        PyDateTime(self.0.utc)
    }

    #[getter]
    fn offset(&self) -> PyUtcOffset {
        PyUtcOffset(self.0.offset)
    }

    /// Convert to local DateTime.
    ///
    /// Returns:
    ///     DateTime: The local DateTime.
    ///
    /// Raises:
    ///     ValueError: If the conversion fails.
    #[pyo3(name = "to_local")]
    fn to_local(&self) -> PyResult<PyDateTime> {
        self.0
            .to_local()
            .map(PyDateTime)
            .map_err(|e| PyValueError::new_err(format!("Local datetime out of range: {:?}", e)))
    }

    /// Get Unix timestamp (seconds since Unix epoch).
    #[pyo3(name = "unix_timestamp")]
    fn unix_timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }

    /// Get Unix timestamp in nanoseconds.
    #[pyo3(name = "unix_timestamp_nanos")]
    fn unix_timestamp_nanos(&self) -> i128 {
        self.0.unix_timestamp_nanos()
    }

    /// Add a duration.
    ///
    /// Args:
    ///     dur: A Duration instance.
    ///
    /// Returns:
    ///     OffsetDateTime: A new OffsetDateTime instance.
    ///
    /// Raises:
    ///     ValueError: If the resulting datetime is out of range.
    #[pyo3(name = "add_duration")]
    fn add_duration(&self, dur: &PyDuration) -> PyResult<Self> {
        self.0
            .add_duration(dur.0)
            .map(PyOffsetDateTime)
            .map_err(|e| PyValueError::new_err(format!("DateTime out of range: {:?}", e)))
    }

    /// Calculate the difference between two OffsetDateTimes.
    ///
    /// Args:
    ///     other: Another OffsetDateTime instance.
    ///
    /// Returns:
    ///     Duration: The duration between the two datetimes (self - other).
    #[pyo3(name = "difference")]
    fn difference(&self, other: &PyOffsetDateTime) -> PyDuration {
        PyDuration(self.0.difference(other.0))
    }

    /// Parse an OffsetDateTime from RFC 3339 format.
    ///
    /// Args:
    ///     s: The string to parse.
    ///
    /// Returns:
    ///     OffsetDateTime: A new OffsetDateTime instance.
    ///
    /// Raises:
    ///     ValueError: If the string is not valid.
    #[classmethod]
    #[pyo3(name = "parse")]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        s.parse::<OffsetDateTime>()
            .map(PyOffsetDateTime)
            .map_err(|_| {
                PyValueError::new_err(format!(
                    "Invalid offset datetime string '{}'. Expected RFC 3339 format: YYYY-MM-DDTHH:MM:SS[.fffffffff][Z|±HH:MM]",
                    s
                ))
            })
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("OffsetDateTime.parse('{}')", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        use pyo3::basic::CompareOp;
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

// ===== Module definition =====

#[pymodule]
fn fasttime(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWeekday>()?;
    m.add_class::<PyDate>()?;
    m.add_class::<PyTime>()?;
    m.add_class::<PyDuration>()?;
    m.add_class::<PyDateTime>()?;
    m.add_class::<PyUtcOffset>()?;
    m.add_class::<PyOffsetDateTime>()?;
    Ok(())
}
