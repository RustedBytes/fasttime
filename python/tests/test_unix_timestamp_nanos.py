"""Lossless timestamp reconstruction, including the complete i32 year range."""

import fasttime
import pytest


@pytest.mark.parametrize(
    "timestamp, text",
    [
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
    ],
)
def test_known_instants(timestamp: int, text: str):
    dt = fasttime.DateTime.from_unix_timestamp_nanos(timestamp)
    assert dt == fasttime.DateTime.parse(text)
    assert dt.unix_timestamp_nanos() == timestamp


@pytest.mark.parametrize("year", [-(2**31), -400, -1, 0, 1, 1969, 2000, 2**31 - 1])
@pytest.mark.parametrize("month, day", [(1, 1), (2, 28), (3, 1), (12, 31)])
@pytest.mark.parametrize("nanos", [0, 1, 123_456_789, 999_999_999])
def test_datetime_round_trip(year: int, month: int, day: int, nanos: int):
    original = fasttime.DateTime(
        fasttime.Date(year, month, day), fasttime.Time(23, 59, 59, nanos)
    )
    timestamp = original.unix_timestamp_nanos()
    assert fasttime.DateTime.from_unix_timestamp_nanos(timestamp) == original


def test_supported_range_boundaries():
    earliest = fasttime.DateTime(fasttime.Date(-(2**31), 1, 1), fasttime.Time(0, 0, 0))
    latest = fasttime.DateTime(
        fasttime.Date(2**31 - 1, 12, 31), fasttime.Time(23, 59, 59, 999_999_999)
    )
    for dt in [earliest, latest]:
        assert (
            fasttime.DateTime.from_unix_timestamp_nanos(dt.unix_timestamp_nanos()) == dt
        )
    for timestamp in [
        earliest.unix_timestamp_nanos() - 1,
        latest.unix_timestamp_nanos() + 1,
        -(2**127),
        2**127 - 1,
        -(2**63) * 1_000_000_000,
        (2**63 - 1) * 1_000_000_000,
    ]:
        with pytest.raises(ValueError, match="OutOfRange"):
            fasttime.DateTime.from_unix_timestamp_nanos(timestamp)


@pytest.mark.parametrize("timestamp", [-(2**127) - 1, 2**127])
def test_python_integer_outside_i128(timestamp: int):
    with pytest.raises(OverflowError):
        fasttime.DateTime.from_unix_timestamp_nanos(timestamp)


@pytest.mark.parametrize("timestamp", [1.0, "1", None])
def test_requires_integer(timestamp):
    with pytest.raises(TypeError):
        fasttime.DateTime.from_unix_timestamp_nanos(timestamp)


def test_keyword_argument():
    assert str(fasttime.DateTime.from_unix_timestamp_nanos(nanos=-1)) == (
        "1969-12-31T23:59:59.999999999Z"
    )
