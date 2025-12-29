"""
Tests for the fasttime Python bindings.
"""

import pytest
import fasttime


def test_date_creation():
    """Test creating dates."""
    date = fasttime.Date(2024, 1, 15)
    assert date.year == 2024
    assert date.month == 1
    assert date.day == 15


def test_date_from_days():
    """Test creating dates from days since epoch."""
    epoch = fasttime.Date.from_days_since_unix_epoch(0)
    assert epoch.year == 1970
    assert epoch.month == 1
    assert epoch.day == 1


def test_date_parse():
    """Test parsing dates from strings."""
    date = fasttime.Date.parse("2024-06-15")
    assert date.year == 2024
    assert date.month == 6
    assert date.day == 15


def test_date_invalid():
    """Test that invalid dates raise errors."""
    with pytest.raises(ValueError):
        fasttime.Date(2024, 13, 1)  # Invalid month
    
    with pytest.raises(ValueError):
        fasttime.Date(2024, 2, 30)  # Invalid day for February


def test_date_weekday():
    """Test weekday calculation."""
    date = fasttime.Date(2024, 1, 1)  # Was a Monday
    weekday = date.weekday()
    assert weekday.number_from_monday() == 1


def test_date_ordinal():
    """Test ordinal (day of year) calculation."""
    date = fasttime.Date(2024, 1, 1)
    assert date.ordinal() == 1
    
    date = fasttime.Date(2024, 12, 31)
    assert date.ordinal() == 366  # 2024 is a leap year


def test_date_add_days():
    """Test adding days to a date."""
    date = fasttime.Date(2024, 1, 1)
    next_day = date.add_days(1)
    assert next_day.year == 2024
    assert next_day.month == 1
    assert next_day.day == 2


def test_date_comparison():
    """Test date comparisons."""
    date1 = fasttime.Date(2024, 1, 1)
    date2 = fasttime.Date(2024, 12, 31)
    
    assert date1 < date2
    assert date1 <= date2
    assert date2 > date1
    assert date2 >= date1
    assert date1 == date1
    assert date1 != date2


def test_date_hash():
    """Test that dates can be hashed."""
    date1 = fasttime.Date(2024, 1, 1)
    date2 = fasttime.Date(2024, 1, 1)
    date3 = fasttime.Date(2024, 12, 31)
    
    assert hash(date1) == hash(date2)
    assert hash(date1) != hash(date3)
    
    # Test in sets
    dates = {date1, date2, date3}
    assert len(dates) == 2


def test_time_creation():
    """Test creating times."""
    time = fasttime.Time(14, 30, 45, nanosecond=123_456_789)
    assert time.hour == 14
    assert time.minute == 30
    assert time.second == 45
    assert time.nanosecond == 123_456_789


def test_time_parse():
    """Test parsing times from strings."""
    time = fasttime.Time.parse("14:30:45.123456789")
    assert time.hour == 14
    assert time.minute == 30
    assert time.second == 45
    assert time.nanosecond == 123_456_789


def test_time_invalid():
    """Test that invalid times raise errors."""
    with pytest.raises(ValueError):
        fasttime.Time(25, 0, 0)  # Invalid hour
    
    with pytest.raises(ValueError):
        fasttime.Time(0, 60, 0)  # Invalid minute


def test_datetime_creation():
    """Test creating datetimes."""
    date = fasttime.Date(2024, 6, 15)
    time = fasttime.Time(12, 30, 45)
    dt = fasttime.DateTime(date, time)
    
    assert dt.date == date
    assert dt.time == time


def test_datetime_from_timestamp():
    """Test creating datetimes from Unix timestamps."""
    dt = fasttime.DateTime.from_unix_timestamp(1_700_000_000, 123_456_789)
    assert dt.unix_timestamp() == 1_700_000_000
    assert dt.time.nanosecond == 123_456_789


def test_datetime_parse():
    """Test parsing datetimes from strings."""
    dt = fasttime.DateTime.parse("2024-06-15T12:30:45Z")
    assert dt.date.year == 2024
    assert dt.date.month == 6
    assert dt.date.day == 15
    assert dt.time.hour == 12
    assert dt.time.minute == 30
    assert dt.time.second == 45


def test_datetime_now_utc():
    """Test getting current UTC time."""
    now = fasttime.DateTime.now_utc()
    assert now.date.year >= 2024  # Should be in the future


def test_datetime_add_duration():
    """Test adding durations to datetimes."""
    dt = fasttime.DateTime.parse("2024-01-01T00:00:00Z")
    one_hour = fasttime.Duration.seconds(3600)
    
    later = dt.add_duration(one_hour)
    assert later.time.hour == 1


def test_datetime_difference():
    """Test calculating differences between datetimes."""
    dt1 = fasttime.DateTime.parse("2024-01-01T00:00:00Z")
    dt2 = fasttime.DateTime.parse("2024-01-01T01:00:00Z")
    
    diff = dt2.difference(dt1)
    assert diff.total_seconds() == 3600


def test_duration_creation():
    """Test creating durations."""
    seconds = fasttime.Duration.seconds(60)
    millis = fasttime.Duration.milliseconds(1000)
    
    assert seconds.total_seconds() == 60
    assert millis.total_seconds() == 1


def test_duration_arithmetic():
    """Test duration arithmetic."""
    dur1 = fasttime.Duration.seconds(10)
    dur2 = fasttime.Duration.seconds(5)
    
    total = dur1 + dur2
    diff = dur1 - dur2
    neg = -dur1
    
    assert total.total_seconds() == 15
    assert diff.total_seconds() == 5
    assert neg.total_seconds() == -10


def test_duration_comparison():
    """Test duration comparisons."""
    short = fasttime.Duration.seconds(10)
    long = fasttime.Duration.seconds(100)
    
    assert short < long
    assert short <= long
    assert long > short
    assert long >= short
    assert short == short
    assert short != long


def test_utc_offset_creation():
    """Test creating UTC offsets."""
    offset = fasttime.UtcOffset.from_hours_minutes(True, 5, 30)
    assert offset.as_seconds() == 5 * 3600 + 30 * 60
    
    offset_neg = fasttime.UtcOffset.from_hours_minutes(False, 8, 0)
    assert offset_neg.as_seconds() == -8 * 3600


def test_utc_offset_is_utc():
    """Test checking if offset is UTC."""
    utc = fasttime.UtcOffset.from_seconds(0)
    not_utc = fasttime.UtcOffset.from_hours_minutes(True, 5, 0)
    
    assert utc.is_utc()
    assert not not_utc.is_utc()


def test_offset_datetime_from_local():
    """Test creating offset datetimes from local time."""
    date = fasttime.Date(2024, 6, 15)
    time = fasttime.Time(18, 0, 0)
    offset = fasttime.UtcOffset.from_hours_minutes(True, 5, 30)
    
    odt = fasttime.OffsetDateTime.from_local(date, time, offset)
    assert odt.offset == offset


def test_offset_datetime_parse():
    """Test parsing offset datetimes."""
    odt = fasttime.OffsetDateTime.parse("2024-06-15T12:00:00+05:30")
    assert odt.offset.as_seconds() == 5 * 3600 + 30 * 60


def test_offset_datetime_to_local():
    """Test converting offset datetime to local."""
    odt = fasttime.OffsetDateTime.parse("2024-06-15T12:00:00Z")
    local = odt.to_local()
    
    assert local.date.year == 2024
    assert local.date.month == 6
    assert local.date.day == 15


def test_round_trip_date():
    """Test round-trip conversion for dates."""
    original = fasttime.Date(2024, 6, 15)
    days = original.days_since_unix_epoch()
    reconstructed = fasttime.Date.from_days_since_unix_epoch(days)
    
    assert original == reconstructed


def test_round_trip_datetime():
    """Test round-trip conversion for datetimes."""
    original = fasttime.DateTime.parse("2024-06-15T12:30:45.123456789Z")
    timestamp = original.unix_timestamp()
    nanos = original.time.nanosecond
    reconstructed = fasttime.DateTime.from_unix_timestamp(timestamp, nanos)
    
    assert original == reconstructed


def test_round_trip_offset_datetime():
    """Test round-trip conversion for offset datetimes."""
    original = "2024-06-15T12:00:00+05:30"
    odt = fasttime.OffsetDateTime.parse(original)
    serialized = str(odt)
    
    assert original == serialized
