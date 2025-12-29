"""
Duration examples for the fasttime library.

This example demonstrates working with durations for time
calculations and interval measurements.
"""

import fasttime


def main():
    print("=== Duration Examples ===\n")

    # 1. Creating Durations
    print("1. Creating Durations")
    print("-" * 40)
    
    seconds = fasttime.Duration.seconds(60)
    millis = fasttime.Duration.milliseconds(500)
    micros = fasttime.Duration.microseconds(1000)
    nanos = fasttime.Duration.nanoseconds(123_456_789)
    
    print(f"60 seconds: {seconds.total_seconds()} s")
    print(f"500 milliseconds: {millis.total_seconds()} s")
    print(f"1000 microseconds: {micros.total_seconds()} s")
    print(f"123,456,789 nanoseconds: {nanos.total_seconds()} s")
    print()

    # 2. Duration Arithmetic
    print("2. Duration Arithmetic")
    print("-" * 40)
    
    one_hour = fasttime.Duration.seconds(3600)
    thirty_minutes = fasttime.Duration.seconds(1800)
    
    total = one_hour + thirty_minutes
    difference = one_hour - thirty_minutes
    negated = -thirty_minutes
    
    print(f"1 hour + 30 minutes: {total.total_seconds()} seconds")
    print(f"1 hour - 30 minutes: {difference.total_seconds()} seconds")
    print(f"-(30 minutes): {negated.total_seconds()} seconds")
    print()

    # 3. Adding Durations to DateTimes
    print("3. Adding Durations to DateTimes")
    print("-" * 40)
    
    start = fasttime.DateTime.parse("2024-01-01T00:00:00Z")
    print(f"Start: {start}")
    
    # Add various durations
    one_day = fasttime.Duration.seconds(86400)
    one_week = fasttime.Duration.seconds(86400 * 7)
    one_year = fasttime.Duration.seconds(86400 * 365)
    
    print(f"+1 day:  {start.add_duration(one_day)}")
    print(f"+1 week: {start.add_duration(one_week)}")
    print(f"+1 year: {start.add_duration(one_year)}")
    print()

    # 4. Calculating Time Differences
    print("4. Calculating Time Differences")
    print("-" * 40)
    
    dt1 = fasttime.DateTime.parse("2024-01-01T12:00:00Z")
    dt2 = fasttime.DateTime.parse("2024-01-01T18:30:45Z")
    
    diff = dt2.difference(dt1)
    print(f"Time 1: {dt1}")
    print(f"Time 2: {dt2}")
    print(f"Difference: {diff.total_seconds()} seconds")
    print(f"            {diff.total_seconds() / 3600:.2f} hours")
    print()

    # 5. Measuring Intervals
    print("5. Measuring Intervals")
    print("-" * 40)
    
    meeting_start = fasttime.DateTime.parse("2024-06-15T09:00:00Z")
    meeting_end = fasttime.DateTime.parse("2024-06-15T10:30:00Z")
    
    duration = meeting_end.difference(meeting_start)
    print(f"Meeting start: {meeting_start}")
    print(f"Meeting end:   {meeting_end}")
    print(f"Duration:      {duration.total_seconds() / 60} minutes")
    print()

    # 6. Comparing Durations
    print("6. Comparing Durations")
    print("-" * 40)
    
    short = fasttime.Duration.seconds(10)
    long = fasttime.Duration.seconds(100)
    
    print(f"Short duration: {short.total_seconds()} s")
    print(f"Long duration:  {long.total_seconds()} s")
    print(f"short < long:   {short < long}")
    print(f"short == long:  {short == long}")
    print(f"short > long:   {short > long}")
    print()

    # 7. Working with Nanoseconds
    print("7. Working with Nanosecond Precision")
    print("-" * 40)
    
    # High-precision timestamps
    dt_start = fasttime.DateTime.parse("2024-01-01T00:00:00.000000000Z")
    dt_end = fasttime.DateTime.parse("2024-01-01T00:00:00.123456789Z")
    
    nano_diff = dt_end.difference(dt_start)
    print(f"Start:      {dt_start}")
    print(f"End:        {dt_end}")
    print(f"Difference: {nano_diff.total_nanos()} nanoseconds")
    print(f"            {nano_diff.total_seconds()} seconds")
    print()

    # 8. Negative Durations
    print("8. Negative Durations")
    print("-" * 40)
    
    future = fasttime.DateTime.parse("2024-12-31T23:59:59Z")
    past = fasttime.DateTime.parse("2024-01-01T00:00:00Z")
    
    # Future - past = positive
    forward = future.difference(past)
    # Past - future = negative
    backward = past.difference(future)
    
    print(f"Future: {future}")
    print(f"Past:   {past}")
    print(f"Future - Past: {forward.total_seconds()} seconds (positive)")
    print(f"Past - Future: {backward.total_seconds()} seconds (negative)")
    print()

    # 9. Common Duration Constants
    print("9. Common Duration Values")
    print("-" * 40)
    
    constants = [
        ("1 millisecond", fasttime.Duration.milliseconds(1)),
        ("1 second", fasttime.Duration.seconds(1)),
        ("1 minute", fasttime.Duration.seconds(60)),
        ("1 hour", fasttime.Duration.seconds(3600)),
        ("1 day", fasttime.Duration.seconds(86400)),
        ("1 week", fasttime.Duration.seconds(86400 * 7)),
    ]
    
    for name, dur in constants:
        print(f"{name:15} = {dur.total_nanos():>20} nanoseconds")
    print()


if __name__ == "__main__":
    main()
