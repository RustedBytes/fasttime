"""
Basic usage examples for the fasttime library.

This example demonstrates common operations with dates, times,
datetimes, and durations.
"""

import fasttime


def main():
    print("=== Basic Usage Examples ===\n")

    # 1. Working with Dates
    print("1. Working with Dates")
    print("-" * 40)
    date = fasttime.Date(2024, 1, 15)
    print(f"Date: {date}")
    print(f"Year: {date.year}, Month: {date.month}, Day: {date.day}")
    print(f"Weekday: {date.weekday()}")
    print(f"Day of year: {date.ordinal()}")
    
    # Add/subtract days
    tomorrow = date.add_days(1)
    yesterday = date.add_days(-1)
    print(f"Tomorrow: {tomorrow}")
    print(f"Yesterday: {yesterday}")
    print()

    # 2. Working with Times
    print("2. Working with Times")
    print("-" * 40)
    time = fasttime.Time(14, 30, 45, nanosecond=123_456_789)
    print(f"Time: {time}")
    print(f"Hour: {time.hour}, Minute: {time.minute}, Second: {time.second}")
    print(f"Nanosecond: {time.nanosecond}")
    print(f"Seconds since midnight: {time.seconds_since_midnight()}")
    print(f"Nanos since midnight: {time.nanos_since_midnight()}")
    print()

    # 3. Working with DateTimes
    print("3. Working with DateTimes")
    print("-" * 40)
    dt = fasttime.DateTime(date, time)
    print(f"DateTime: {dt}")
    print(f"Date component: {dt.date}")
    print(f"Time component: {dt.time}")
    
    # Get current time
    now = fasttime.DateTime.now_utc()
    print(f"Current UTC time: {now}")
    print()

    # 4. Working with Durations
    print("4. Working with Durations")
    print("-" * 40)
    
    # Create durations
    one_hour = fasttime.Duration.seconds(3600)
    one_day = fasttime.Duration.seconds(86400)
    half_second = fasttime.Duration.milliseconds(500)
    
    print(f"One hour: {one_hour.total_seconds()} seconds")
    print(f"One day: {one_day.total_seconds()} seconds")
    print(f"Half second: {half_second.total_seconds()} seconds")
    
    # Add durations
    total = one_hour + half_second
    print(f"One hour + 500ms: {total.total_seconds()} seconds")
    
    # Add duration to datetime
    later = dt.add_duration(one_hour)
    print(f"Original: {dt}")
    print(f"One hour later: {later}")
    
    # Calculate difference
    diff = later.difference(dt)
    print(f"Difference: {diff.total_seconds()} seconds")
    print()

    # 5. Comparisons
    print("5. Comparisons")
    print("-" * 40)
    date1 = fasttime.Date(2024, 1, 1)
    date2 = fasttime.Date(2024, 12, 31)
    
    print(f"{date1} < {date2}: {date1 < date2}")
    print(f"{date1} == {date2}: {date1 == date2}")
    print(f"{date1} > {date2}: {date1 > date2}")
    print()

    # 6. Hash and Sets
    print("6. Hash and Sets")
    print("-" * 40)
    dates = {
        fasttime.Date(2024, 1, 1),
        fasttime.Date(2024, 6, 15),
        fasttime.Date(2024, 12, 31),
        fasttime.Date(2024, 1, 1),  # Duplicate
    }
    print(f"Unique dates: {len(dates)}")
    for d in sorted(dates):
        print(f"  {d}")
    print()

    # 7. Converting to/from days since epoch
    print("7. Days Since Unix Epoch")
    print("-" * 40)
    epoch = fasttime.Date.from_days_since_unix_epoch(0)
    print(f"Unix epoch (day 0): {epoch}")
    
    days = date.days_since_unix_epoch()
    print(f"{date} is {days} days since epoch")
    
    reconstructed = fasttime.Date.from_days_since_unix_epoch(days)
    print(f"Reconstructed: {reconstructed}")
    print(f"Match: {date == reconstructed}")
    print()


if __name__ == "__main__":
    main()
