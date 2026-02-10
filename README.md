# fasttime

[![Test](https://github.com/RustedBytes/fasttime/actions/workflows/test.yml/badge.svg)](https://github.com/RustedBytes/fasttime/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/fasttime)](https://crates.io/crates/fasttime)

`fasttime` is a small UTC-focused date/time library for Rust. It is built
around [Ben Joffe's constant-time 64-bit days→date algorithm](https://www.benjoffe.com/fast-date-64) and offers a simple,
`no_std`-friendly API for calendar math, parsing, and formatting. The crate
only depends on `core` by default and enables a handful of conveniences (such as
`DateTime::now_utc()`) when the optional `std` feature is on.

**🐍 Python bindings are now available!** See [python/README.md](python/README.md) for details.

## Features

- Works in `no_std` environments; opt into `std` when you need wall-clock time.
- `Date`, `Time`, `DateTime`, `Duration`, `UtcOffset`, and `OffsetDateTime`
  types with ISO/RFC 3339 style `Display` implementations.
- Parsing helpers for the common textual formats used in logs and APIs.
- Fixed-offset RFC 3339 timestamps with nanosecond precision.
- Simple arithmetic helpers: add days, add durations, compute differences, and
  fetch ordinals or weekdays without extra allocations.
- **Python 3.10+ bindings** via PyO3 with full type hints and ergonomic API.

## Installation

### Rust

```toml
[dependencies]
fasttime = "0.2"
```

### Python

```bash
pip install maturin
maturin develop --release
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

`cargo bench --bench unix_timestamp -- --quick` runs a quick benchmark comparing `fasttime`'s Unix timestamp conversions to the widely used `time` crate. 

The benchmark covers three operations:
- `from_unix_timestamp`: constructor taking `(secs, nanos)`.
- `to_unix_timestamp`: extracting integral seconds.
- `to_unix_timestamp_nanos`: extracting nanoseconds as `i128`.

### Example output

```
from_unix_timestamp/fasttime::default-n=1024
                        time:   [6.3651 µs 6.3674 µs 6.3765 µs]
                        change: [+0.0149% +0.2131% +0.4118%] (p = 0.14 > 0.05)
                        No change in performance detected.
from_unix_timestamp/time::default-n=1024
                        time:   [8.6900 µs 8.6924 µs 8.7022 µs]
                        change: [+3.0617% +3.2376% +3.4139%] (p = 0.10 > 0.05)
                        No change in performance detected.

to_unix_timestamp/fasttime::default-n=1024
                        time:   [1.6843 µs 1.6853 µs 1.6894 µs]
                        change: [−1.1021% −0.5719% −0.0376%] (p = 0.10 > 0.05)
                        No change in performance detected.
to_unix_timestamp/time::default-n=1024
                        time:   [1.8566 µs 1.8589 µs 1.8682 µs]
                        change: [−0.0414% +0.3413% +0.7245%] (p = 0.33 > 0.05)
                        No change in performance detected.

to_unix_timestamp_nanos/fasttime::default-n=1024
                        time:   [2.0008 µs 2.0063 µs 2.0077 µs]
                        change: [−0.0852% +0.2399% +0.5661%] (p = 0.33 > 0.05)
                        No change in performance detected.
to_unix_timestamp_nanos/time::default-n=1024
                        time:   [2.0863 µs 2.0880 µs 2.0946 µs]
                        change: [+0.4222% +0.7240% +1.0265%] (p = 0.08 > 0.05)
                        No change in performance detected.
```

### Conclusion

In this sample run, `fasttime` is consistently faster than `time` for all three
timestamp operations (about 27% faster for `from_unix_timestamp`, about 9%
faster for `to_unix_timestamp`, and about 4% faster for
`to_unix_timestamp_nanos`), while all comparisons report no statistically
significant regression versus baseline.

## Development

- Format and lint the crate with your preferred Rust tooling (`cargo fmt`,
  `cargo clippy`).
- Run the tests with `cargo test`.

## License

Licensed under the Apache-2.0
