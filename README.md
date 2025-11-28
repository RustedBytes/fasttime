# fasttime

[![Test](https://github.com/RustedBytes/fasttime/actions/workflows/test.yml/badge.svg)](https://github.com/RustedBytes/fasttime/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/fasttime)](https://crates.io/crates/fasttime)

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

`cargo bench --bench unix_timestamp` runs Criterion benchmarks that compare `fasttime` with the `time` crate when converting to and from Unix timestamps:
- `from_unix_timestamp`: constructor taking `(secs, nanos)`.
- `to_unix_timestamp`: extracting integral seconds.
- `to_unix_timestamp_nanos`: extracting nanoseconds as `i128`.

Each group runs the conversions over 64, 1,024, and 16,384 timestamp samples to
show how performance scales with dataset size.

### Example output

```
from_unix_timestamp/fasttime::default-n=1024
                        time:   [7.5703 µs 7.5711 µs 7.5721 µs]
                        change: [-4.0811% -4.0481% -4.0177%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe
from_unix_timestamp/time::default-n=1024
                        time:   [8.0240 µs 8.0287 µs 8.0347 µs]
                        change: [-0.1042% -0.0677% -0.0232%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

to_unix_timestamp/fasttime::default-n=1024
                        time:   [3.2994 µs 3.2996 µs 3.2999 µs]
                        change: [+0.0822% +0.1046% +0.1245%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
to_unix_timestamp/time::default-n=1024
                        time:   [1.8323 µs 1.8326 µs 1.8330 µs]
                        change: [-0.0536% -0.0195% +0.0093%] (p = 0.24 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

to_unix_timestamp_nanos/fasttime::default-n=1024
                        time:   [3.7286 µs 3.7313 µs 3.7342 µs]
                        change: [+1.7513% +1.9180% +2.0849%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
to_unix_timestamp_nanos/time::default-n=1024
                        time:   [2.0604 µs 2.0607 µs 2.0610 µs]
                        change: [+0.0420% +0.0554% +0.0703%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
```

## Development

- Format and lint the crate with your preferred Rust tooling (`cargo fmt`,
  `cargo clippy`).
- Run the tests with `cargo test`.

## License

Licensed under the Apache-2.0
