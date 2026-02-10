// ===== Tests (use std, so fine even with no_std library) =====

#[cfg(test)]
mod tests {
    use fasttime::{
        parse_rfc3339_offset, Date, DateError, DateTime, Duration, OffsetDateTime, Time, TimeError,
        UtcOffset, Weekday,
    };

    #[test]
    fn date_epoch_and_neighbors() {
        let d0 = Date::from_days_since_unix_epoch(0).unwrap();
        assert_eq!(d0.year, 1970);
        assert_eq!(d0.month, 1);
        assert_eq!(d0.day, 1);

        let d1 = Date::from_days_since_unix_epoch(1).unwrap();
        assert_eq!(d1.to_string(), "1970-01-02");

        let dm1 = Date::from_days_since_unix_epoch(-1).unwrap();
        assert_eq!(dm1.to_string(), "1969-12-31");
    }

    #[test]
    fn days_round_trip() {
        let cases = [
            (1970, 1, 1),
            (1970, 1, 2),
            (1969, 12, 31),
            (2000, 2, 29),
            (2000, 3, 1),
            (1900, 3, 1),
            (2400, 2, 29),
        ];
        for &(y, m, d) in &cases {
            let date = Date::from_ymd(y, m, d).unwrap();
            let days = date.days_since_unix_epoch();
            let round = Date::from_days_since_unix_epoch(days).unwrap();
            assert_eq!(date, round);
        }
    }

    #[test]
    fn datetime_unix_round_trip() {
        let date = Date::from_ymd(2024, 5, 17).unwrap();
        let time = Time::from_hms_nano(12, 34, 56, 123_456_789).unwrap();
        let dt = DateTime::new(date, time);
        let secs = dt.unix_timestamp();
        let nanos = dt.time.nanosecond as i32;
        let rt = DateTime::from_unix_timestamp(secs, nanos).unwrap();
        assert_eq!(dt, rt);
    }

    #[test]
    fn duration_add_sub() {
        let date = Date::from_ymd(2020, 1, 1).unwrap();
        let time = Time::from_hms_nano(0, 0, 0, 0).unwrap();
        let dt = DateTime::new(date, time);
        let dur = Duration::seconds(86_400); // 1 day
        let dt2 = dt.add_duration(dur).unwrap();
        assert_eq!(dt2.date.to_string(), "2020-01-02");
        assert_eq!(dt2.time.to_string(), "00:00:00");

        let diff = dt2.difference(dt);
        assert_eq!(diff, dur);
    }

    #[test]
    fn parse_and_display_basic() {
        let d: Date = "2023-11-05".parse().unwrap();
        assert_eq!(d.to_string(), "2023-11-05");

        let t: Time = "23:59:59.001".parse().unwrap();
        assert_eq!(t.to_string(), "23:59:59.001");

        let dt: DateTime = "2023-11-05T23:59:59.001Z".parse().unwrap();
        assert_eq!(dt.to_string(), "2023-11-05T23:59:59.001Z");
    }

    #[test]
    fn offset_datetime_rfc3339() {
        let s = "2023-11-05T23:59:59.5+02:00";
        let odt: OffsetDateTime = s.parse().unwrap();
        assert_eq!(odt.to_string(), s);

        let odt_z: OffsetDateTime = "2023-11-05T23:59:59Z".parse().unwrap();
        assert_eq!(odt_z.to_string(), "2023-11-05T23:59:59Z");
    }

    #[test]
    fn date_weekday_and_ordinal() {
        let monday = Date::from_ymd(2023, 11, 6).unwrap();
        assert_eq!(monday.weekday(), Weekday::Monday);
        assert_eq!(monday.ordinal(), 310);

        let leap = Date::from_ymd(2020, 3, 1).unwrap();
        assert_eq!(leap.weekday(), Weekday::Sunday);
        assert_eq!(leap.ordinal(), 61);
    }

    #[test]
    fn time_fractional_and_nanos() {
        let t: Time = "12:34:56.123450700".parse().unwrap();
        assert_eq!(t.nanosecond, 123_450_700);
        assert_eq!(t.to_string(), "12:34:56.1234507");
        assert_eq!(t.seconds_since_midnight(), 45_296);
        assert_eq!(t.nanos_since_midnight(), 45_296_123_450_700);
    }

    #[test]
    fn time_parse_rejects_invalid_fraction() {
        assert!(matches!(
            "12:00:00.".parse::<Time>(),
            Err(TimeError::InvalidTime)
        ));
        assert!(matches!(
            "12:00:00.1234567890".parse::<Time>(),
            Err(TimeError::InvalidTime)
        ));
    }

    #[test]
    fn offset_datetime_local_conversion_and_duration() {
        let date = Date::from_ymd(2021, 1, 2).unwrap();
        let time = Time::from_hms_nano(3, 4, 5, 999_999_999).unwrap();
        let offset = UtcOffset::from_hours_minutes(false, 5, 45).unwrap(); // UTC-05:45

        let odt = OffsetDateTime::from_local(date, time, offset).unwrap();
        assert_eq!(odt.offset, offset);

        let local = odt.to_local().unwrap();
        assert_eq!(local.date, date);
        assert_eq!(local.time, time);

        let later = odt.add_duration(Duration::seconds(30)).unwrap();
        assert_eq!(later.difference(odt), Duration::seconds(30));
    }

    /// Test 1: The "Civil" Limits (i32 boundaries)
    /// The algorithm theoretically supports ~1.3 trillion years, but the struct
    /// enforces i32. We verify the code handles these explicit boundaries correctly.
    #[test]
    fn test_i32_year_limits() {
        // Max i32 year: 2,147,483,647
        // This is NOT a leap year (odd number).
        let max_date = Date::from_ymd(i32::MAX, 12, 31).unwrap();
        let max_days = max_date.days_since_unix_epoch();
        let round_trip_max = Date::from_days_since_unix_epoch(max_days).unwrap();
        assert_eq!(
            max_date, round_trip_max,
            "Failed round-trip at i32::MAX year"
        );

        // Min i32 year: -2,147,483,648
        // This IS a leap year (divisible by 4, and 400 if we treat it as proleptic).
        // -2147483648 % 4 == 0.
        // -2147483648 % 100 != 0.
        // It should have a Feb 29.
        let min_date = Date::from_ymd(i32::MIN, 2, 29).unwrap();
        let min_days = min_date.days_since_unix_epoch();
        let round_trip_min = Date::from_days_since_unix_epoch(min_days).unwrap();
        assert_eq!(
            min_date, round_trip_min,
            "Failed round-trip at i32::MIN year"
        );
    }

    /// Test 2: The "Out of Range" Safety Check
    /// Ensure the implementation correctly rejects years outside i32 range,
    /// even if the math would theoretically work.
    #[test]
    fn test_overflow_protection() {
        // Construct a day count that would result in year i32::MAX + 1
        // i32::MAX is 2,147,483,647.
        // Let's take the days for i32::MAX-12-31 and add 1 day.
        let max_date = Date::from_ymd(i32::MAX, 12, 31).unwrap();
        let days_overflow = max_date.days_since_unix_epoch().checked_add(1).unwrap();

        let result = Date::from_days_since_unix_epoch(days_overflow);
        assert_eq!(
            result,
            Err(DateError::OutOfRange),
            "Did not catch i32 year overflow"
        );
    }

    /// Test 3: Gregorian Leap Year Rules (The "Julian Map" Verification)
    /// The PDF mentions a specific "Julian map" technique to handle the
    /// 100/400 year rule without expensive modulo operators.
    /// We verify this logic holds for tricky century boundaries.
    #[test]
    fn test_gregorian_leap_rules() {
        let cases = [
            (1900, 2, 28, false), // Divisible by 100, NOT 400 -> Not Leap
            (1900, 3, 1, false),  // Day after Feb 28
            (2000, 2, 29, true),  // Divisible by 400 -> Leap
            (2100, 2, 28, false), // Divisible by 100, NOT 400 -> Not Leap
            (2024, 2, 29, true),  // Standard Leap
            (2023, 2, 28, false), // Standard Common
        ];

        for (y, m, d, _) in cases {
            let date = Date::from_ymd(y, m, d).expect("Invalid test date setup");
            let days = date.days_since_unix_epoch();

            // Round trip via fast algorithm
            let recovered = Date::from_days_since_unix_epoch(days).unwrap();
            assert_eq!(date, recovered, "Failed date: {}-{}-{}", y, m, d);

            // Verify sequential continuity
            // If it's Feb 28 of a non-leap year, next day is Mar 1
            if m == 2 && d == 28 && !is_leap_year(y) {
                let next_day = Date::from_days_since_unix_epoch(days + 1).unwrap();
                assert_eq!(next_day.month, 3);
                assert_eq!(next_day.day, 1);
            }
            // If it's Feb 28 of a leap year, next day is Feb 29
            if m == 2 && d == 28 && is_leap_year(y) {
                let next_day = Date::from_days_since_unix_epoch(days + 1).unwrap();
                assert_eq!(next_day.month, 2);
                assert_eq!(next_day.day, 29);
            }
        }
    }

    /// Test 4: The 400-Year Cycle Exhaustive Test
    /// The Gregorian calendar repeats exactly every 146,097 days (400 years).
    /// If the algorithm works for one full cycle, it is mathematically proven
    /// for all proleptic Gregorian time (ignoring integer overflow).
    /// This verifies the "Year-modulo-bitshift" logic.
    #[test]
    fn test_full_400_year_cycle() {
        // Start from an arbitrary date, e.g., 2000-01-01
        let start_year = 2000i32;
        let mut days_accum = Date::from_ymd(start_year, 1, 1)
            .unwrap()
            .days_since_unix_epoch();

        // Track an expected Gregorian date with independent step logic.
        let mut exp_year = start_year;
        let mut exp_month = 1u8;
        let mut exp_day = 1u8;

        // One full Gregorian cycle = 146,097 days.
        for _ in 0..146_097 {
            let date = Date::from_days_since_unix_epoch(days_accum).unwrap();
            assert_eq!(
                (date.year, date.month, date.day),
                (exp_year, exp_month, exp_day),
                "Date mismatch at days {}",
                days_accum
            );

            // Increment expected date by one day.
            let dim = reference_days_in_month(exp_year, exp_month);
            if exp_day < dim {
                exp_day += 1;
            } else {
                exp_day = 1;
                if exp_month < 12 {
                    exp_month += 1;
                } else {
                    exp_month = 1;
                    exp_year += 1;
                }
            }
            days_accum += 1;
        }

        // After exactly one 400-year cycle, we should land on 2400-01-01.
        let after_cycle = Date::from_days_since_unix_epoch(days_accum).unwrap();
        assert_eq!(
            (after_cycle.year, after_cycle.month, after_cycle.day),
            (2400, 1, 1)
        );
    }

    /// Test 5: Negative Unix Timestamps (Pre-1970)
    /// The algorithm counts backwards from a shifted epoch.
    /// We verify this doesn't break near the Unix epoch boundary.
    #[test]
    fn test_negative_epoch_crossing() {
        use time::OffsetDateTime as StdOffsetDateTime;

        // Exact expectations around the epoch boundary.
        let around_epoch = [
            (-2, (1969, 12, 30)),
            (-1, (1969, 12, 31)),
            (0, (1970, 1, 1)),
            (1, (1970, 1, 2)),
            (2, (1970, 1, 3)),
        ];
        for (days, (y, m, d)) in around_epoch {
            let got = Date::from_days_since_unix_epoch(days).unwrap();
            assert_eq!(
                (got.year, got.month, got.day),
                (y, m, d),
                "Epoch-neighbor mismatch at day {}",
                days
            );
        }

        // Deeper negative samples cross-checked against the `time` crate.
        let samples = [-10_000_i64, -3_653, -366, -365, -30, -7];
        for days in samples {
            let got = Date::from_days_since_unix_epoch(days).unwrap();
            let std_dt = StdOffsetDateTime::from_unix_timestamp(days * 86_400).unwrap();
            assert_eq!(got.year, std_dt.year(), "Year mismatch at day {}", days);
            assert_eq!(
                got.month,
                u8::from(std_dt.month()),
                "Month mismatch at day {}",
                days
            );
            assert_eq!(got.day, std_dt.day(), "Day mismatch at day {}", days);
        }
    }

    #[test]
    fn matches_std_time_reference() {
        use time::OffsetDateTime as StdOffsetDateTime;

        // Sample a mix of positive/negative timestamps plus some with
        // non-normalized nanosecond components to stress normalization logic.
        let samples: &[(i64, i32)] = &[
            (0, 0),
            (1, 0),
            (-1, 0),
            (86_399, 999_999_999),
            (-86_400, 0),
            (1_234_567_890, 987_654_321),
            (-1_234_567_890, 123_456_789),
            (50 * 365 * 24 * 60 * 60, 1),
            (-50 * 365 * 24 * 60 * 60, -250_000_000),
            (5_000, 1_500_000_000),
            (-5_000, -1_500_000_000),
        ];

        for &(secs, nanos) in samples {
            let fast = DateTime::from_unix_timestamp(secs, nanos).unwrap();
            let total = (secs as i128) * 1_000_000_000 + nanos as i128;
            let std_dt = StdOffsetDateTime::from_unix_timestamp_nanos(total).unwrap();

            assert_eq!(fast.date.year, std_dt.year());
            assert_eq!(fast.date.month, u8::from(std_dt.month()));
            assert_eq!(fast.date.day, std_dt.day());
            assert_eq!(fast.time.hour, std_dt.hour());
            assert_eq!(fast.time.minute, std_dt.minute());
            assert_eq!(fast.time.second, std_dt.second());
            assert_eq!(fast.time.nanosecond, std_dt.nanosecond());
        }
    }

    #[test]
    fn rfc3339_offset_variants() {
        let with_colon = parse_rfc3339_offset("+02:30").unwrap();
        assert_eq!(with_colon.as_seconds(), 2 * 3600 + 30 * 60);

        let compact = parse_rfc3339_offset("+0230").unwrap();
        assert_eq!(compact.as_seconds(), 2 * 3600 + 30 * 60);

        let hour_only = parse_rfc3339_offset("-07").unwrap();
        assert_eq!(hour_only.as_seconds(), -7 * 3600);

        assert!(parse_rfc3339_offset("invalid").is_err());
    }

    #[test]
    fn leap_year_matches_reference_formula() {
        for year in -20_000..=20_000 {
            let reference = is_leap_year(year);
            let fasttime_accepts_feb_29 = Date::from_ymd(year, 2, 29).is_ok();
            assert_eq!(
                fasttime_accepts_feb_29, reference,
                "Mismatch for year {year}"
            );
        }
    }

    #[test]
    fn month_lengths_match_reference() {
        let years = [
            i32::MIN,
            -4_000,
            -2_100,
            -2_000,
            -1,
            0,
            1,
            1_900,
            2_000,
            2_024,
            2_100,
            4_000,
            i32::MAX,
        ];

        for &year in &years {
            for month in 1..=12 {
                let expected = match month {
                    2 => {
                        if is_leap_year(year) {
                            29
                        } else {
                            28
                        }
                    }
                    4 | 6 | 9 | 11 => 30,
                    _ => 31,
                };

                assert!(
                    Date::from_ymd(year, month, expected).is_ok(),
                    "Expected valid end-of-month for {year:04}-{month:02}"
                );
                assert!(
                    Date::from_ymd(year, month, expected + 1).is_err(),
                    "Expected invalid overflow day for {year:04}-{month:02}"
                );
            }
        }
    }

    // Helper needed for the test logic (copy of internal helper)
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn reference_days_in_month(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => panic!("invalid month in test helper: {}", month),
        }
    }
}
