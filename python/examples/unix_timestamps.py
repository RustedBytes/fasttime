"""
Unix timestamp examples for the fasttime library.

This example demonstrates working with Unix timestamps
(seconds and nanoseconds since the Unix epoch).
"""

import fasttime


def main():
    print("=== Unix Timestamp Examples ===\n")

    # 1. The Unix Epoch
    print("1. The Unix Epoch")
    print("-" * 40)
    
    epoch = fasttime.DateTime.from_unix_timestamp(0)
    print(f"Unix epoch (timestamp 0): {epoch}")
    print(f"Date: {epoch.date}")
    print(f"Time: {epoch.time}")
    
    epoch_date = fasttime.Date.from_days_since_unix_epoch(0)
    print(f"Epoch date: {epoch_date}")
    print()

    # 2. Converting to Unix Timestamps
    print("2. Converting to Unix Timestamps")
    print("-" * 40)
    
    dt = fasttime.DateTime.parse("2024-06-15T12:30:45Z")
    timestamp = dt.unix_timestamp()
    timestamp_nanos = dt.unix_timestamp_nanos()
    
    print(f"DateTime: {dt}")
    print(f"Unix timestamp (seconds): {timestamp}")
    print(f"Unix timestamp (nanos):   {timestamp_nanos}")
    print()

    # 3. Creating DateTimes from Timestamps
    print("3. Creating DateTimes from Timestamps")
    print("-" * 40)
    
    # Various timestamps
    timestamps = [
        (0, "Unix epoch"),
        (1_000_000_000, "2001-09-09 (Billennium)"),
        (1_700_000_000, "2023-11-14"),
        (2_000_000_000, "2033-05-18"),
    ]
    
    for ts, description in timestamps:
        dt = fasttime.DateTime.from_unix_timestamp(ts)
        print(f"{ts:15} -> {dt} ({description})")
    print()

    # 4. Nanosecond Precision
    print("4. Nanosecond Precision")
    print("-" * 40)
    
    # Create with nanoseconds
    dt1 = fasttime.DateTime.from_unix_timestamp(1_700_000_000, 0)
    dt2 = fasttime.DateTime.from_unix_timestamp(1_700_000_000, 123_456_789)
    dt3 = fasttime.DateTime.from_unix_timestamp(1_700_000_000, 999_999_999)
    
    print(f"Same second, different nanos:")
    print(f"  {dt1} ({dt1.unix_timestamp_nanos()})")
    print(f"  {dt2} ({dt2.unix_timestamp_nanos()})")
    print(f"  {dt3} ({dt3.unix_timestamp_nanos()})")
    print()

    # 5. Negative Timestamps (Before 1970)
    print("5. Negative Timestamps (Before 1970)")
    print("-" * 40)
    
    negative_timestamps = [
        -86400,      # 1 day before epoch
        -31_536_000,  # ~1 year before epoch
    ]
    
    for ts in negative_timestamps:
        dt = fasttime.DateTime.from_unix_timestamp(ts)
        print(f"{ts:15} -> {dt}")
    print()

    # 6. Round-trip Conversion
    print("6. Round-trip Conversion")
    print("-" * 40)
    
    original = fasttime.DateTime.parse("2024-06-15T14:30:45.123456789Z")
    timestamp = original.unix_timestamp()
    nanos = original.time.nanosecond
    
    reconstructed = fasttime.DateTime.from_unix_timestamp(timestamp, nanos)
    
    print(f"Original:      {original}")
    print(f"Timestamp:     {timestamp}")
    print(f"Nanoseconds:   {nanos}")
    print(f"Reconstructed: {reconstructed}")
    print(f"Match:         {original == reconstructed}")
    print()

    # 7. Timestamp Arithmetic
    print("7. Timestamp Arithmetic")
    print("-" * 40)
    
    dt = fasttime.DateTime.from_unix_timestamp(1_700_000_000)
    print(f"Base datetime: {dt}")
    
    # Add various durations
    one_hour_later = dt.add_duration(fasttime.Duration.seconds(3600))
    one_day_later = dt.add_duration(fasttime.Duration.seconds(86400))
    
    print(f"+1 hour: {one_hour_later} (ts: {one_hour_later.unix_timestamp()})")
    print(f"+1 day:  {one_day_later} (ts: {one_day_later.unix_timestamp()})")
    print()

    # 8. Timestamp Differences
    print("8. Timestamp Differences")
    print("-" * 40)
    
    dt1 = fasttime.DateTime.from_unix_timestamp(1_700_000_000)
    dt2 = fasttime.DateTime.from_unix_timestamp(1_700_086_400)
    
    diff = dt2.difference(dt1)
    ts_diff = dt2.unix_timestamp() - dt1.unix_timestamp()
    
    print(f"DateTime 1:     {dt1} (ts: {dt1.unix_timestamp()})")
    print(f"DateTime 2:     {dt2} (ts: {dt2.unix_timestamp()})")
    print(f"Duration diff:  {diff.total_seconds()} seconds")
    print(f"Timestamp diff: {ts_diff} seconds")
    print(f"Match:          {diff.total_seconds() == ts_diff}")
    print()

    # 9. OffsetDateTime Timestamps
    print("9. OffsetDateTime Timestamps")
    print("-" * 40)
    
    # Different offsets, same instant
    utc_odt = fasttime.OffsetDateTime.parse("2024-06-15T12:00:00Z")
    ny_odt = fasttime.OffsetDateTime.parse("2024-06-15T07:00:00-05:00")
    tokyo_odt = fasttime.OffsetDateTime.parse("2024-06-15T21:00:00+09:00")
    
    print(f"UTC:   {utc_odt} (ts: {utc_odt.unix_timestamp()})")
    print(f"NY:    {ny_odt} (ts: {ny_odt.unix_timestamp()})")
    print(f"Tokyo: {tokyo_odt} (ts: {tokyo_odt.unix_timestamp()})")
    print(f"\nAll same instant: {utc_odt.unix_timestamp() == ny_odt.unix_timestamp() == tokyo_odt.unix_timestamp()}")
    print()

    # 10. High-resolution Timing
    print("10. High-resolution Timing")
    print("-" * 40)
    
    # Simulate high-precision measurements
    measurements = [
        fasttime.DateTime.from_unix_timestamp(1_700_000_000, 0),
        fasttime.DateTime.from_unix_timestamp(1_700_000_000, 100_000),
        fasttime.DateTime.from_unix_timestamp(1_700_000_000, 200_000),
        fasttime.DateTime.from_unix_timestamp(1_700_000_000, 300_000),
    ]
    
    print("High-resolution measurements:")
    for i, dt in enumerate(measurements):
        nanos = dt.unix_timestamp_nanos()
        print(f"  {i}: {dt} ({nanos} ns)")
    
    # Calculate intervals
    print("\nIntervals:")
    for i in range(len(measurements) - 1):
        diff = measurements[i + 1].difference(measurements[i])
        print(f"  {i} -> {i+1}: {diff.total_nanos()} ns")
    print()

    # 11. Common Timestamps
    print("11. Common Historical Timestamps")
    print("-" * 40)
    
    historical = [
        (0, "Unix epoch"),
        (315_532_800, "Y2K preparation era (1980-01-01)"),
        (946_684_800, "Y2K (2000-01-01)"),
        (1_000_000_000, "Billennium"),
        (1_234_567_890, "Unix billionth second + change"),
    ]
    
    for ts, desc in historical:
        dt = fasttime.DateTime.from_unix_timestamp(ts)
        print(f"{ts:15} -> {dt:30} {desc}")
    print()


if __name__ == "__main__":
    main()
