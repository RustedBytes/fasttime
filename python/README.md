# fasttime for Python

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

`fasttime` is a fast, lightweight UTC-focused date/time library for Python, powered by Rust. It provides simple, ergonomic APIs for working with dates, times, and durations with nanosecond precision.

## Features

- **Fast**: Built with Rust using [Ben Joffe's constant-time 64-bit days→date algorithm](https://www.benjoffe.com/fast-date-64)
- **Simple**: Clean, Pythonic API for common date/time operations
- **Precise**: Nanosecond precision for all time values
- **UTC-focused**: All datetime operations are in UTC by default
- **RFC 3339 support**: Parse and format timestamps with fixed offsets
- **Type-safe**: Full type hints for better IDE support and type checking

## Installation

```bash
pip install fasttime
```

Requires Python 3.10 or later.

## Quick Start

```python
import fasttime

# Create dates
date = fasttime.Date(2024, 1, 15)
print(date)  # 2024-01-15
print(date.weekday())  # Weekday.Monday
print(date.ordinal())  # 15

# Work with times
time = fasttime.Time(14, 30, 45, nanosecond=123_456_789)
print(time)  # 14:30:45.123456789

# Combine into UTC datetimes
dt = fasttime.DateTime(date, time)
print(dt)  # 2024-01-15T14:30:45.123456789Z

# Get current UTC time
now = fasttime.DateTime.now_utc()
print(f"Current UTC time: {now}")

# Work with durations
later = now.add_duration(fasttime.Duration.seconds(3600))
diff = later.difference(now)
print(f"Difference: {diff.total_seconds()} seconds")

# Parse from strings
parsed_date = fasttime.Date.parse("2024-12-25")
parsed_time = fasttime.Time.parse("23:59:59.999")
parsed_dt = fasttime.DateTime.parse("2024-12-25T23:59:59.999Z")

# Work with time zones (fixed offsets)
offset = fasttime.UtcOffset.from_hours_minutes(True, 5, 30)  # +05:30
local_dt = fasttime.OffsetDateTime.from_local(date, time, offset)
print(local_dt)  # 2024-01-15T14:30:45.123456789+05:30

# Unix timestamps
timestamp = dt.unix_timestamp()  # seconds
timestamp_ns = dt.unix_timestamp_nanos()  # nanoseconds
dt_from_ts = fasttime.DateTime.from_unix_timestamp(timestamp, 123_456_789)
```

## API Reference

### Date

A Gregorian calendar date (year, month, day).

```python
# Constructor
date = fasttime.Date(year: int, month: int, day: int)

# Properties
date.year: int        # Year (i32 range)
date.month: int       # Month (1-12)
date.day: int         # Day (1-31)

# Methods
date.weekday() -> Weekday              # Get day of week
date.ordinal() -> int                  # Day of year (1-366)
date.add_days(days: int) -> Date       # Add/subtract days
date.days_since_unix_epoch() -> int    # Days since 1970-01-01

# Class methods
Date.from_days_since_unix_epoch(days: int) -> Date
Date.parse(s: str) -> Date             # Parse "YYYY-MM-DD"
```

### Time

A time of day with nanosecond precision.

```python
# Constructor
time = fasttime.Time(hour: int, minute: int, second: int, nanosecond: int = 0)

# Properties
time.hour: int         # Hour (0-23)
time.minute: int       # Minute (0-59)
time.second: int       # Second (0-59)
time.nanosecond: int   # Nanosecond (0-999,999,999)

# Methods
time.seconds_since_midnight() -> int   # Total seconds (ignoring nanos)
time.nanos_since_midnight() -> int     # Total nanoseconds

# Class methods
Time.parse(s: str) -> Time             # Parse "HH:MM:SS[.fffffffff]"
```

### DateTime

A UTC date and time.

```python
# Constructor
dt = fasttime.DateTime(date: Date, time: Time)

# Properties
dt.date: Date
dt.time: Time

# Methods
dt.unix_timestamp() -> int                      # Seconds since Unix epoch
dt.unix_timestamp_nanos() -> int                # Nanoseconds since Unix epoch
dt.add_duration(dur: Duration) -> DateTime      # Add a duration
dt.difference(other: DateTime) -> Duration      # Calculate difference

# Class methods
DateTime.from_unix_timestamp(secs: int, nanos: int = 0) -> DateTime
DateTime.now_utc() -> DateTime                  # Current UTC time
DateTime.parse(s: str) -> DateTime              # Parse "YYYY-MM-DDTHH:MM:SS[.fff]Z"
```

### Duration

A signed duration with nanosecond precision.

```python
# Class methods (constructors)
Duration.seconds(secs: int) -> Duration
Duration.milliseconds(ms: int) -> Duration
Duration.microseconds(us: int) -> Duration
Duration.nanoseconds(ns: int) -> Duration

# Methods
dur.total_seconds() -> float      # Total seconds as float
dur.total_nanos() -> int          # Total nanoseconds as int

# Operators
dur1 + dur2                       # Add durations
dur1 - dur2                       # Subtract durations
-dur                              # Negate duration
dur1 < dur2                       # Compare durations
```

### UtcOffset

A fixed offset from UTC.

```python
# Class methods
UtcOffset.from_seconds(seconds: int) -> UtcOffset
UtcOffset.from_hours_minutes(sign_positive: bool, hours: int, minutes: int) -> UtcOffset

# Methods
offset.as_seconds() -> int        # Total offset in seconds
offset.is_utc() -> bool           # True if offset is zero
```

### OffsetDateTime

A datetime with a fixed UTC offset (RFC 3339 style).

```python
# Class methods
OffsetDateTime.from_utc(utc: DateTime, offset: UtcOffset) -> OffsetDateTime
OffsetDateTime.from_local(date: Date, time: Time, offset: UtcOffset) -> OffsetDateTime
OffsetDateTime.parse(s: str) -> OffsetDateTime  # Parse RFC 3339

# Properties
odt.utc: DateTime                 # UTC datetime
odt.offset: UtcOffset             # Offset from UTC

# Methods
odt.to_local() -> DateTime                          # Convert to local datetime
odt.unix_timestamp() -> int                         # Seconds since Unix epoch
odt.unix_timestamp_nanos() -> int                   # Nanoseconds since Unix epoch
odt.add_duration(dur: Duration) -> OffsetDateTime   # Add a duration
odt.difference(other: OffsetDateTime) -> Duration   # Calculate difference
```

### Weekday

An enumeration of weekdays (ISO order, Monday = 1).

```python
# Constants
Weekday.MONDAY
Weekday.TUESDAY
Weekday.WEDNESDAY
Weekday.THURSDAY
Weekday.FRIDAY
Weekday.SATURDAY
Weekday.SUNDAY

# Methods
weekday.number_from_monday() -> int  # 1-7
```

## Examples

See the [`python/examples/`](examples/) directory for more examples:

- [`basic_usage.py`](examples/basic_usage.py) - Common operations
- [`parsing.py`](examples/parsing.py) - Parsing dates and times
- [`durations.py`](examples/durations.py) - Working with durations
- [`timezones.py`](examples/timezones.py) - Fixed offset timezones
- [`unix_timestamps.py`](examples/unix_timestamps.py) - Unix timestamp conversions

## Comparison with Other Libraries

| Feature | fasttime | datetime (stdlib) | pendulum | arrow |
|---------|----------|------------------|----------|-------|
| Speed | ⚡ Very Fast (Rust) | Fast (C) | Moderate | Moderate |
| Nanosecond precision | ✅ | ❌ (microsecond) | ❌ (microsecond) | ❌ (microsecond) |
| UTC-focused | ✅ | Partial | ❌ | ❌ |
| RFC 3339 parsing | ✅ | ✅ | ✅ | ✅ |
| Timezone database | ❌ (fixed offset only) | ❌ | ✅ | ✅ |
| Dependencies | None (after build) | None | Many | Many |
| Memory footprint | Small | Small | Large | Large |

**When to use fasttime:**
- You need UTC timestamps and simple date/time operations
- You want nanosecond precision
- You prefer a minimal, fast library without heavy dependencies
- You're working with logs, metrics, or APIs using ISO 8601/RFC 3339

**When to use other libraries:**
- You need full timezone database support (use `pendulum` or `zoneinfo`)
- You need complex calendar/business day calculations
- You're already using the standard library and don't need extra precision

## Performance

`fasttime` is built with Rust and uses highly optimized algorithms for date/time conversions:

- **Days ↔ Date conversion**: Uses Ben Joffe's constant-time algorithm (no division/modulo)
- **Unix timestamp conversion**: O(1) operations
- **Parsing**: Minimal allocations, direct byte processing

Typical operations complete in microseconds, making it suitable for high-throughput applications.

## Development

### Building from Source

```bash
# Install maturin
pip install maturin

# Build the package
maturin develop

# Run tests
pytest python/tests/
```

### Running Examples

```bash
python python/examples/basic_usage.py
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
