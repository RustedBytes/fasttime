# fasttime

`fasttime` is a small UTC-focused date/time library for Rust. It is built
around [Ben Joffe's constant-time 64-bit days→date algorithm](https://www.benjoffe.com/fast-date-64) and offers a simple,
`no_std`-friendly API for calendar math, parsing, and formatting. The crate
only depends on `core` by default and enables a handful of conveniences (such as
`DateTime::now_utc()`) when the optional `std` feature is on.

## Features

- Works in `no_std` environments; opt into `std` when you need wall-clock time.
- `Date`, `Time`, `DateTime`, `Duration`, `UtcOffset`, and `OffsetDateTime`
  types with ISO/RFC 3339 style `Display` implementations.
- Parsing helpers for the common textual formats used in logs and APIs.
- Fixed-offset RFC 3339 timestamps with nanosecond precision.
- Simple arithmetic helpers: add days, add durations, compute differences, and
  fetch ordinals or weekdays without extra allocations.

## Installation

```toml
[dependencies]
fasttime = "0.1"
```

Disable the default `std` feature when targeting bare-metal or embedded
environments:

```toml
[dependencies]
fasttime = { version = "0.1", default-features = false }
```

## Example

```rust
use fasttime::{
    Date, DateError, DateTime, Duration, OffsetDateTime, Time, UtcOffset,
};

fn main() -> Result<(), DateError> {
    // Fetch the current UTC date and time, works only with `std` feature.
    // let now = DateTime::now_utc()?;
    // println!("Current UTC time: {now}");

    // Build calendar dates either from year-month-day or days since the Unix epoch.
    let jan_first = Date::from_ymd(2024, 1, 1)?;
    let epoch = Date::from_days_since_unix_epoch(0)?;
    println!("{jan_first} weekday {}", jan_first.weekday().number_from_monday());
    println!("{epoch} ordinal {}", epoch.ordinal());

    // Combine dates and times to work with UTC instants.
    let dt = DateTime::from_unix_timestamp(1_700_000_000, 123_456_789)?;
    let later = dt.add_duration(Duration::seconds(90))?;
    println!("Delta: {} ns", later.difference(dt).total_nanos());

    // Parse ISO/RFC 3339 style strings and work with fixed offsets.
    let parsed_date: Date = "2024-03-15".parse()?;
    let parsed_time: Time = "11:22:33.123".parse().unwrap();
    let offset = UtcOffset::from_hours_minutes(true, 2, 0).unwrap();
    let local = OffsetDateTime::from_local(parsed_date, parsed_time, offset)?;
    println!("{local} (offset {})", offset);

    Ok(())
}
```

## Benchmarks

`cargo bench --bench unix_timestamp` runs a Criterion benchmark that compares
`fasttime::DateTime::from_unix_timestamp` with `time::OffsetDateTime::from_unix_timestamp_nanos`.

### Example output

```
from_unix_timestamp/fasttime::DateTime
                        time:   [9.0027 µs 9.0169 µs 9.0311 µs]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
from_unix_timestamp/time::OffsetDateTime
                        time:   [11.872 µs 11.963 µs 12.077 µs]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
```

## Development

- Format and lint the crate with your preferred Rust tooling (`cargo fmt`,
  `cargo clippy`).
- Run the tests with `cargo test`.

## License

Licensed under the Apache License, Version 2.0 (see `Cargo.toml` metadata).
