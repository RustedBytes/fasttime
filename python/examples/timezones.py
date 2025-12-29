"""
Timezone (fixed offset) examples for the fasttime library.

This example demonstrates working with fixed UTC offsets
using OffsetDateTime.
"""

import fasttime


def main():
    print("=== Timezone (Fixed Offset) Examples ===\n")

    # 1. Creating UTC Offsets
    print("1. Creating UTC Offsets")
    print("-" * 40)
    
    utc = fasttime.UtcOffset.from_seconds(0)
    plus_five_thirty = fasttime.UtcOffset.from_hours_minutes(True, 5, 30)
    minus_eight = fasttime.UtcOffset.from_hours_minutes(False, 8, 0)
    plus_one = fasttime.UtcOffset.from_seconds(3600)
    
    print(f"UTC:         {utc} ({utc.as_seconds()} seconds)")
    print(f"+05:30:      {plus_five_thirty} ({plus_five_thirty.as_seconds()} seconds)")
    print(f"-08:00:      {minus_eight} ({minus_eight.as_seconds()} seconds)")
    print(f"+01:00:      {plus_one} ({plus_one.as_seconds()} seconds)")
    print(f"UTC check:   {utc.is_utc()}")
    print(f"+05:30 UTC?: {plus_five_thirty.is_utc()}")
    print()

    # 2. Creating OffsetDateTime from UTC
    print("2. Creating OffsetDateTime from UTC")
    print("-" * 40)
    
    utc_dt = fasttime.DateTime.parse("2024-06-15T12:00:00Z")
    offset = fasttime.UtcOffset.from_hours_minutes(True, 5, 30)
    
    odt = fasttime.OffsetDateTime.from_utc(utc_dt, offset)
    print(f"UTC DateTime:    {utc_dt}")
    print(f"With offset:     {odt}")
    print(f"Local DateTime:  {odt.to_local()}")
    print()

    # 3. Creating OffsetDateTime from Local Time
    print("3. Creating OffsetDateTime from Local Time")
    print("-" * 40)
    
    date = fasttime.Date(2024, 6, 15)
    time = fasttime.Time(18, 0, 0)  # 6:00 PM local
    offset = fasttime.UtcOffset.from_hours_minutes(True, 5, 30)  # +05:30
    
    odt = fasttime.OffsetDateTime.from_local(date, time, offset)
    print(f"Local Date:      {date}")
    print(f"Local Time:      {time}")
    print(f"Offset:          {offset}")
    print(f"OffsetDateTime:  {odt}")
    print(f"UTC equivalent:  {odt.utc}")
    print()

    # 4. Common Timezone Offsets
    print("4. Common Timezone Offsets")
    print("-" * 40)
    
    common_offsets = [
        ("UTC", 0, 0, True),
        ("EST", 5, 0, False),   # -05:00
        ("PST", 8, 0, False),   # -08:00
        ("CET", 1, 0, True),    # +01:00
        ("IST", 5, 30, True),   # +05:30 (India)
        ("JST", 9, 0, True),    # +09:00 (Japan)
    ]
    
    date = fasttime.Date(2024, 6, 15)
    time = fasttime.Time(12, 0, 0)  # Noon local
    
    for name, hours, minutes, positive in common_offsets:
        offset = fasttime.UtcOffset.from_hours_minutes(positive, hours, minutes)
        odt = fasttime.OffsetDateTime.from_local(date, time, offset)
        print(f"{name:5} ({offset}): {odt} -> UTC: {odt.utc}")
    print()

    # 5. Converting Between Offsets
    print("5. Converting Between Offsets")
    print("-" * 40)
    
    # Start with a time in New York (EST)
    ny_offset = fasttime.UtcOffset.from_hours_minutes(False, 5, 0)
    ny_date = fasttime.Date(2024, 6, 15)
    ny_time = fasttime.Time(14, 30, 0)  # 2:30 PM in NY
    
    ny_dt = fasttime.OffsetDateTime.from_local(ny_date, ny_time, ny_offset)
    print(f"New York time:    {ny_dt}")
    
    # Convert to UTC
    utc_dt = ny_dt.utc
    print(f"UTC time:         {utc_dt}")
    
    # Convert to Tokyo (JST)
    tokyo_offset = fasttime.UtcOffset.from_hours_minutes(True, 9, 0)
    tokyo_dt = fasttime.OffsetDateTime.from_utc(utc_dt, tokyo_offset)
    print(f"Tokyo time:       {tokyo_dt}")
    print(f"Tokyo local:      {tokyo_dt.to_local()}")
    print()

    # 6. Parsing RFC 3339 with Offsets
    print("6. Parsing RFC 3339 with Offsets")
    print("-" * 40)
    
    rfc3339_examples = [
        "2024-06-15T12:00:00Z",
        "2024-06-15T12:00:00+00:00",
        "2024-06-15T12:00:00-05:00",
        "2024-06-15T12:00:00+05:30",
    ]
    
    for s in rfc3339_examples:
        odt = fasttime.OffsetDateTime.parse(s)
        print(f"{s:35} -> {odt}")
        print(f"  UTC: {odt.utc}, Offset: {odt.offset}")
    print()

    # 7. Duration with Offsets
    print("7. Duration with Offsets")
    print("-" * 40)
    
    start = fasttime.OffsetDateTime.parse("2024-06-15T10:00:00+05:30")
    duration = fasttime.Duration.seconds(7200)  # 2 hours
    end = start.add_duration(duration)
    
    print(f"Start:    {start}")
    print(f"Duration: {duration.total_seconds() / 3600} hours")
    print(f"End:      {end}")
    print(f"Difference: {end.difference(start).total_seconds()} seconds")
    print()

    # 8. Comparing Across Offsets
    print("8. Comparing Across Offsets")
    print("-" * 40)
    
    # 12:00 in New York
    ny = fasttime.OffsetDateTime.parse("2024-06-15T12:00:00-05:00")
    # 17:00 in UTC (same instant)
    utc = fasttime.OffsetDateTime.parse("2024-06-15T17:00:00Z")
    # 22:30 in India (same instant)
    india = fasttime.OffsetDateTime.parse("2024-06-15T22:30:00+05:30")
    
    print(f"New York: {ny}")
    print(f"UTC:      {utc}")
    print(f"India:    {india}")
    print(f"\nAll represent the same instant:")
    print(f"NY == UTC:    {ny == utc}")
    print(f"UTC == India: {utc == india}")
    print(f"NY == India:  {ny == india}")
    print()

    # 9. Unix Timestamps with Offsets
    print("9. Unix Timestamps with Offsets")
    print("-" * 40)
    
    odt = fasttime.OffsetDateTime.parse("2024-06-15T18:00:00+05:30")
    timestamp = odt.unix_timestamp()
    timestamp_nanos = odt.unix_timestamp_nanos()
    
    print(f"OffsetDateTime:    {odt}")
    print(f"UTC equivalent:    {odt.utc}")
    print(f"Unix timestamp:    {timestamp}")
    print(f"Unix timestamp ns: {timestamp_nanos}")
    
    # Convert back
    utc_dt = fasttime.DateTime.from_unix_timestamp(timestamp)
    print(f"From timestamp:    {utc_dt}")
    print()


if __name__ == "__main__":
    main()
