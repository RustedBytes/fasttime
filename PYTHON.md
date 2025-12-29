# Python Bindings for fasttime

This repository now includes Python 3.10+ bindings for the `fasttime` Rust crate!

## Quick Start

### Installation

```bash
pip install maturin
maturin develop --release
```

### Usage

```python
import fasttime

# Create dates and times
date = fasttime.Date(2024, 6, 15)
time = fasttime.Time(14, 30, 45, nanosecond=123_456_789)
dt = fasttime.DateTime(date, time)

# Get current UTC time
now = fasttime.DateTime.now_utc()

# Parse from strings
dt = fasttime.DateTime.parse("2024-06-15T12:30:45Z")

# Work with durations
later = dt.add_duration(fasttime.Duration.seconds(3600))
diff = later.difference(dt)
print(f"Difference: {diff.total_seconds()} seconds")

# Work with timezones (fixed offsets)
offset = fasttime.UtcOffset.from_hours_minutes(True, 5, 30)  # +05:30
odt = fasttime.OffsetDateTime.from_local(date, time, offset)
```

## Documentation

See [`python/README.md`](python/README.md) for comprehensive documentation.

## Examples

Check out the example files in [`python/examples/`](python/examples/):
- `basic_usage.py` - Basic date/time operations
- `parsing.py` - Parsing strings
- `durations.py` - Duration calculations
- `timezones.py` - Working with UTC offsets
- `unix_timestamps.py` - Unix timestamp conversions

## Running Tests

```bash
# Python tests
pytest python/tests/

# Rust tests
cargo test
```

## Features

✅ Fast (Rust-powered)  
✅ Nanosecond precision  
✅ UTC-focused  
✅ Type hints included  
✅ Simple, ergonomic API  
✅ Python 3.10+ support  
