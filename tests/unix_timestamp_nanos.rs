use fasttime::{Date, DateError, DateTime, Time};

#[test]
fn nanosecond_timestamps_match_known_instants() {
    let cases = [
        (0, "1970-01-01T00:00:00Z"),
        (1, "1970-01-01T00:00:00.000000001Z"),
        (999_999_999, "1970-01-01T00:00:00.999999999Z"),
        (1_000_000_000, "1970-01-01T00:00:01Z"),
        (1_000_000_001, "1970-01-01T00:00:01.000000001Z"),
        (-1, "1969-12-31T23:59:59.999999999Z"),
        (-999_999_999, "1969-12-31T23:59:59.000000001Z"),
        (-1_000_000_000, "1969-12-31T23:59:59Z"),
        (-1_000_000_001, "1969-12-31T23:59:58.999999999Z"),
        (-86_400_000_000_001, "1969-12-30T23:59:59.999999999Z"),
        (1_700_000_000_123_456_789, "2023-11-14T22:13:20.123456789Z"),
        (-2_208_988_799_123_456_789, "1900-01-01T00:00:00.876543211Z"),
    ];
    for (timestamp, text) in cases {
        let expected: DateTime = text.parse().unwrap();
        let actual = DateTime::from_unix_timestamp_nanos(timestamp).unwrap();
        assert_eq!(actual, expected, "timestamp {timestamp}");
        assert_eq!(actual.unix_timestamp_nanos(), timestamp);
    }
}

fn date_limits() -> (DateTime, DateTime) {
    (
        DateTime::new(
            Date::from_ymd(i32::MIN, 1, 1).unwrap(),
            Time::from_hms_nano(0, 0, 0, 0).unwrap(),
        ),
        DateTime::new(
            Date::from_ymd(i32::MAX, 12, 31).unwrap(),
            Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
        ),
    )
}

#[test]
fn nanosecond_timestamp_date_limits_and_overflow() {
    let (min, max) = date_limits();
    for dt in [min, max] {
        assert_eq!(
            DateTime::from_unix_timestamp_nanos(dt.unix_timestamp_nanos()),
            Ok(dt)
        );
    }
    for timestamp in [
        min.unix_timestamp_nanos() - 1,
        max.unix_timestamp_nanos() + 1,
        i128::MIN,
        i128::MAX,
        i128::from(i64::MIN) * 1_000_000_000,
        i128::from(i64::MAX) * 1_000_000_000,
    ] {
        assert_eq!(
            DateTime::from_unix_timestamp_nanos(timestamp),
            Err(DateError::OutOfRange),
            "timestamp {timestamp}"
        );
    }
}

#[test]
fn nanosecond_timestamp_round_trips_across_entire_date_range() {
    // Calendar-first samples cover both i32 year limits, BCE, leap days,
    // centuries, and subsecond values that floating-point timestamps lose.
    for year in [i32::MIN, -400, -1, 0, 1, 1900, 1969, 1970, 2000, i32::MAX] {
        for month in 1..=12 {
            for day in [1, 28, 29, 30, 31] {
                let Ok(date) = Date::from_ymd(year, month, day) else {
                    continue;
                };
                for (h, m, s, ns) in [
                    (0, 0, 0, 0),
                    (0, 0, 0, 1),
                    (12, 34, 56, 123_456_789),
                    (23, 59, 59, 999_999_999),
                ] {
                    let dt = DateTime::new(date, Time::from_hms_nano(h, m, s, ns).unwrap());
                    assert_eq!(
                        DateTime::from_unix_timestamp_nanos(dt.unix_timestamp_nanos()),
                        Ok(dt)
                    );
                }
            }
        }
    }

    // Timestamp-first samples span the full supported interval (well beyond
    // i64 nanoseconds), including its endpoints, without a random dependency.
    let (min, max) = date_limits();
    let start = min.unix_timestamp_nanos();
    let span = max.unix_timestamp_nanos() - start;
    for i in 0..=16_384 {
        let timestamp = start + span * i / 16_384;
        let dt = DateTime::from_unix_timestamp_nanos(timestamp).unwrap();
        assert_eq!(dt.unix_timestamp_nanos(), timestamp);
    }
}

#[test]
fn nanosecond_timestamps_match_time_crate() {
    let start = time::Date::MIN
        .midnight()
        .assume_utc()
        .unix_timestamp_nanos();
    let end = time::Date::MAX
        .with_hms_nano(23, 59, 59, 999_999_999)
        .unwrap()
        .assume_utc()
        .unix_timestamp_nanos();
    let span = end - start;
    for i in 0..=16_384 {
        let timestamp = start + span * i / 16_384;
        let reference = time::OffsetDateTime::from_unix_timestamp_nanos(timestamp).unwrap();
        let actual = DateTime::from_unix_timestamp_nanos(timestamp).unwrap();
        assert_eq!(
            actual.date,
            Date::from_ymd(reference.year(), reference.month().into(), reference.day()).unwrap()
        );
        assert_eq!(
            actual.time,
            Time::from_hms_nano(
                reference.hour(),
                reference.minute(),
                reference.second(),
                reference.nanosecond()
            )
            .unwrap()
        );
    }
}
