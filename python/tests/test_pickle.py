"""Pickle regressions for nanosecond-precision DateTime and Duration values."""

import pickle
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

import fasttime
import pytest

PROTOCOLS = [None, -1, *range(pickle.HIGHEST_PROTOCOL + 1)]
DURATION_NANOS = [
    -(2**127),
    -(2**63) - 1,
    -1_000_000_001,
    -1,
    0,
    1,
    1_000_000_001,
    2**63 + 1,
    2**127 - 1,
]
DATETIME_PARTS = [
    (-(2**31), 1, 1, 0, 0, 0, 0),
    (-1, 12, 31, 23, 59, 59, 999_999_999),
    (0, 2, 29, 12, 30, 45, 123_456_789),
    (1, 1, 1, 0, 0, 0, 1),
    (1969, 12, 31, 23, 59, 59, 999_999_999),
    (1970, 1, 1, 0, 0, 0, 0),
    (1970, 1, 1, 0, 0, 0, 1),
    (2024, 2, 29, 12, 30, 45, 123_456_789),
    (9999, 12, 31, 23, 59, 59, 999_999_999),
    (2**31 - 1, 12, 31, 23, 59, 59, 999_999_999),
]


@pytest.mark.parametrize("protocol", PROTOCOLS)
@pytest.mark.parametrize("nanos", DURATION_NANOS)
def test_duration_pickle(nanos: int, protocol: int | None) -> None:
    original = fasttime.Duration.nanoseconds(nanos)
    restored = pickle.loads(pickle.dumps(original, protocol=protocol))

    assert type(restored) is fasttime.Duration
    assert restored is not original
    assert restored.total_nanos() == nanos
    assert restored == original
    assert hash(restored) == hash(original)
    assert {original: "value"}[restored] == "value"
    assert len({original, restored}) == 1


@pytest.mark.parametrize("protocol", PROTOCOLS)
@pytest.mark.parametrize("parts", DATETIME_PARTS)
def test_datetime_pickle(parts: tuple[int, ...], protocol: int | None) -> None:
    year, month, day, hour, minute, second, nanos = parts
    original = fasttime.DateTime(
        fasttime.Date(year, month, day), fasttime.Time(hour, minute, second, nanos)
    )
    restored = pickle.loads(pickle.dumps(original, protocol=protocol))

    assert type(restored) is fasttime.DateTime
    assert restored is not original
    assert (
        restored.date.year,
        restored.date.month,
        restored.date.day,
        restored.time.hour,
        restored.time.minute,
        restored.time.second,
        restored.time.nanosecond,
    ) == parts
    assert restored.unix_timestamp_nanos() == original.unix_timestamp_nanos()
    assert restored == original
    assert hash(restored) == hash(original)
    assert {original: "value"}[restored] == "value"
    assert len({original, restored}) == 1


@pytest.mark.parametrize("protocol", PROTOCOLS)
def test_datetime_pickle_negative_nanoseconds(protocol: int | None) -> None:
    original = fasttime.DateTime.from_unix_timestamp(0, -1)
    restored = pickle.loads(pickle.dumps(original, protocol=protocol))
    assert restored.unix_timestamp_nanos() == -1
    assert restored.unix_timestamp() == -1
    assert restored.time.nanosecond == 999_999_999
    assert restored == original
    assert hash(restored) == hash(original)


def test_pickle_fresh_interpreter() -> None:
    """Reconstruction callables must resolve without prior class registration."""
    values = [
        fasttime.Duration.nanoseconds(-(2**127)),
        fasttime.Duration.nanoseconds(2**127 - 1),
        fasttime.DateTime.from_unix_timestamp(0, -1),
    ]
    # Include each protocol in the payload so a clean process exercises them all.
    payload = pickle.dumps([pickle.dumps(values, protocol=p) for p in PROTOCOLS])
    script = """
import pickle
import sys
import fasttime

for payload in pickle.loads(sys.stdin.buffer.read()):
    minimum, maximum, before_epoch = pickle.loads(payload)
    assert type(minimum) is fasttime.Duration
    assert type(maximum) is fasttime.Duration
    assert type(before_epoch) is fasttime.DateTime
    assert minimum.total_nanos() == -(2**127)
    assert maximum.total_nanos() == 2**127 - 1
    assert before_epoch.unix_timestamp_nanos() == -1
"""
    subprocess.run(
        [sys.executable, "-c", script], input=payload, check=True, timeout=30
    )


def test_pickle_shared_values_across_threads() -> None:
    """Exercise shared instances, including when the interpreter has no GIL."""
    values = (
        fasttime.Duration.nanoseconds(-(2**127)),
        fasttime.DateTime.from_unix_timestamp(0, -1),
    )

    def roundtrip(_: int) -> None:
        for protocol in PROTOCOLS:
            for original in values:
                restored = pickle.loads(pickle.dumps(original, protocol=protocol))
                assert restored == original
                assert hash(restored) == hash(original)

    with ThreadPoolExecutor(max_workers=8) as executor:
        list(executor.map(roundtrip, range(32)))
