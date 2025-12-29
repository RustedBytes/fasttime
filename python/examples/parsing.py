"""
Parsing examples for the fasttime library.

This example demonstrates parsing dates, times, and datetimes
from various string formats.
"""

import fasttime


def main():
    print("=== Parsing Examples ===\n")

    # 1. Parsing Dates
    print("1. Parsing Dates (YYYY-MM-DD)")
    print("-" * 40)
    
    date_strings = [
        "2024-01-01",
        "2024-12-31",
        "2000-02-29",  # Leap year
        "1970-01-01",  # Unix epoch
    ]
    
    for s in date_strings:
        date = fasttime.Date.parse(s)
        print(f"{s:12} -> {date} (weekday: {date.weekday()})")
    print()

    # 2. Parsing Times
    print("2. Parsing Times (HH:MM:SS[.fffffffff])")
    print("-" * 40)
    
    time_strings = [
        "00:00:00",
        "12:00:00",
        "23:59:59",
        "14:30:45.5",
        "14:30:45.123",
        "14:30:45.123456",
        "14:30:45.123456789",
    ]
    
    for s in time_strings:
        time = fasttime.Time.parse(s)
        print(f"{s:25} -> {time}")
    print()

    # 3. Parsing UTC DateTimes
    print("3. Parsing UTC DateTimes (ISO 8601 / RFC 3339)")
    print("-" * 40)
    
    datetime_strings = [
        "2024-01-01T00:00:00Z",
        "2024-12-31T23:59:59Z",
        "2024-06-15T12:30:45Z",
        "2024-06-15T12:30:45.5Z",
        "2024-06-15T12:30:45.123456789Z",
    ]
    
    for s in datetime_strings:
        dt = fasttime.DateTime.parse(s)
        print(f"{s:40} -> {dt}")
        print(f"  Unix timestamp: {dt.unix_timestamp()}")
    print()

    # 4. Parsing OffsetDateTimes (with timezone offsets)
    print("4. Parsing OffsetDateTimes (RFC 3339 with offsets)")
    print("-" * 40)
    
    offset_datetime_strings = [
        "2024-06-15T12:30:45Z",
        "2024-06-15T12:30:45+00:00",
        "2024-06-15T12:30:45+05:30",
        "2024-06-15T12:30:45-08:00",
        "2024-06-15T12:30:45.123456789+02:00",
    ]
    
    for s in offset_datetime_strings:
        odt = fasttime.OffsetDateTime.parse(s)
        print(f"{s:45} -> {odt}")
        print(f"  UTC: {odt.utc}")
        print(f"  Offset: {odt.offset}")
        print(f"  Local: {odt.to_local()}")
    print()

    # 5. Round-trip: String -> Parse -> String
    print("5. Round-trip Parsing")
    print("-" * 40)
    
    original = "2024-06-15T14:30:45.123456789+05:30"
    odt = fasttime.OffsetDateTime.parse(original)
    serialized = str(odt)
    
    print(f"Original:   {original}")
    print(f"Parsed:     {odt}")
    print(f"Serialized: {serialized}")
    print(f"Match:      {original == serialized}")
    print()

    # 6. Error handling
    print("6. Error Handling")
    print("-" * 40)
    
    invalid_strings = [
        ("2024-13-01", "Invalid month"),
        ("2024-02-30", "Invalid day for February"),
        ("25:00:00", "Invalid hour"),
        ("2024-01-01", "Missing time/timezone for DateTime"),
    ]
    
    for s, reason in invalid_strings:
        try:
            if ":" in s and "-" not in s:
                # Looks like a time
                fasttime.Time.parse(s)
            elif "T" in s or "Z" in s:
                # Looks like a datetime
                fasttime.DateTime.parse(s)
            else:
                # Looks like a date
                fasttime.Date.parse(s)
            print(f"✗ {s:20} - Should have failed ({reason})")
        except ValueError as e:
            print(f"✓ {s:20} - Correctly rejected: {reason}")
    print()


if __name__ == "__main__":
    main()
