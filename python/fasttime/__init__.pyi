"""
Type stubs for fasttime Python bindings.

This file provides type hints for the fasttime module, which is
implemented in Rust via PyO3.
"""

from typing import ClassVar

class Weekday:
    """Calendar weekday (ISO order, Monday = 1)."""
    MONDAY: ClassVar[Weekday]
    TUESDAY: ClassVar[Weekday]
    WEDNESDAY: ClassVar[Weekday]
    THURSDAY: ClassVar[Weekday]
    FRIDAY: ClassVar[Weekday]
    SATURDAY: ClassVar[Weekday]
    SUNDAY: ClassVar[Weekday]
    
    def number_from_monday(self) -> int:
        """Get the weekday number from Monday (1-7)."""
        ...

class Date:
    """Gregorian calendar date (proleptic)."""
    
    def __init__(self, year: int, month: int, day: int) -> None:
        """Create a new Date from year, month, and day."""
        ...
    
    @property
    def year(self) -> int:
        """Year component."""
        ...
    
    @property
    def month(self) -> int:
        """Month component (1-12)."""
        ...
    
    @property
    def day(self) -> int:
        """Day component (1-31)."""
        ...
    
    @classmethod
    def from_days_since_unix_epoch(cls, days: int) -> Date:
        """Create a Date from days since Unix epoch."""
        ...
    
    def days_since_unix_epoch(self) -> int:
        """Convert to days since Unix epoch."""
        ...
    
    def weekday(self) -> Weekday:
        """Get the weekday."""
        ...
    
    def ordinal(self) -> int:
        """Get the day of the year (1-366)."""
        ...
    
    def add_days(self, days: int) -> Date:
        """Add days to the date."""
        ...
    
    @classmethod
    def parse(cls, s: str) -> Date:
        """Parse a date from ISO format (YYYY-MM-DD)."""
        ...

class Time:
    """Time of day in nanoseconds since midnight."""
    
    def __init__(self, hour: int, minute: int, second: int, nanosecond: int = 0) -> None:
        """Create a new Time."""
        ...
    
    @property
    def hour(self) -> int:
        """Hour component (0-23)."""
        ...
    
    @property
    def minute(self) -> int:
        """Minute component (0-59)."""
        ...
    
    @property
    def second(self) -> int:
        """Second component (0-59)."""
        ...
    
    @property
    def nanosecond(self) -> int:
        """Nanosecond component (0-999,999,999)."""
        ...
    
    def seconds_since_midnight(self) -> int:
        """Get total seconds since midnight."""
        ...
    
    def nanos_since_midnight(self) -> int:
        """Get total nanoseconds since midnight."""
        ...
    
    @classmethod
    def parse(cls, s: str) -> Time:
        """Parse a time from ISO format."""
        ...

class Duration:
    """Signed duration with nanosecond precision."""
    
    @classmethod
    def seconds(cls, secs: int) -> Duration:
        """Create a duration from seconds."""
        ...
    
    @classmethod
    def milliseconds(cls, ms: int) -> Duration:
        """Create a duration from milliseconds."""
        ...
    
    @classmethod
    def microseconds(cls, us: int) -> Duration:
        """Create a duration from microseconds."""
        ...
    
    @classmethod
    def nanoseconds(cls, ns: int) -> Duration:
        """Create a duration from nanoseconds."""
        ...
    
    def total_seconds(self) -> float:
        """Get total seconds as a float."""
        ...
    
    def total_nanos(self) -> int:
        """Get total nanoseconds as an integer."""
        ...
    
    def __add__(self, other: Duration) -> Duration: ...
    def __sub__(self, other: Duration) -> Duration: ...
    def __neg__(self) -> Duration: ...

class DateTime:
    """Combined UTC date and time."""
    
    def __init__(self, date: Date, time: Time) -> None:
        """Create a DateTime from a Date and Time."""
        ...
    
    @property
    def date(self) -> Date:
        """Date component."""
        ...
    
    @property
    def time(self) -> Time:
        """Time component."""
        ...
    
    @classmethod
    def from_unix_timestamp(cls, secs: int, nanos: int = 0) -> DateTime:
        """Create a DateTime from Unix timestamp."""
        ...
    
    def unix_timestamp(self) -> int:
        """Get Unix timestamp (seconds since Unix epoch)."""
        ...
    
    def unix_timestamp_nanos(self) -> int:
        """Get Unix timestamp in nanoseconds."""
        ...
    
    def add_duration(self, dur: Duration) -> DateTime:
        """Add a duration to this DateTime."""
        ...
    
    def difference(self, other: DateTime) -> Duration:
        """Calculate the difference between two DateTimes."""
        ...
    
    @classmethod
    def now_utc(cls) -> DateTime:
        """Get the current UTC DateTime."""
        ...
    
    @classmethod
    def parse(cls, s: str) -> DateTime:
        """Parse a DateTime from ISO 8601 / RFC 3339 UTC format."""
        ...

class UtcOffset:
    """Fixed offset from UTC."""
    
    @classmethod
    def from_seconds(cls, seconds: int) -> UtcOffset:
        """Create a UtcOffset from total seconds."""
        ...
    
    @classmethod
    def from_hours_minutes(cls, sign_positive: bool, hours: int, minutes: int) -> UtcOffset:
        """Create a UtcOffset from hours and minutes."""
        ...
    
    def as_seconds(self) -> int:
        """Get the offset as seconds."""
        ...
    
    def is_utc(self) -> bool:
        """Check if this is UTC (offset = 0)."""
        ...

class OffsetDateTime:
    """Date-time with a fixed offset from UTC."""
    
    @classmethod
    def from_utc(cls, utc: DateTime, offset: UtcOffset) -> OffsetDateTime:
        """Create an OffsetDateTime from a UTC DateTime and offset."""
        ...
    
    @classmethod
    def from_local(cls, date: Date, time: Time, offset: UtcOffset) -> OffsetDateTime:
        """Create an OffsetDateTime from local date, time, and offset."""
        ...
    
    @property
    def utc(self) -> DateTime:
        """UTC datetime."""
        ...
    
    @property
    def offset(self) -> UtcOffset:
        """Offset from UTC."""
        ...
    
    def to_local(self) -> DateTime:
        """Convert to local DateTime."""
        ...
    
    def unix_timestamp(self) -> int:
        """Get Unix timestamp (seconds since Unix epoch)."""
        ...
    
    def unix_timestamp_nanos(self) -> int:
        """Get Unix timestamp in nanoseconds."""
        ...
    
    def add_duration(self, dur: Duration) -> OffsetDateTime:
        """Add a duration."""
        ...
    
    def difference(self, other: OffsetDateTime) -> Duration:
        """Calculate the difference between two OffsetDateTimes."""
        ...
    
    @classmethod
    def parse(cls, s: str) -> OffsetDateTime:
        """Parse an OffsetDateTime from RFC 3339 format."""
        ...

__all__ = [
    "Weekday",
    "Date",
    "Time",
    "Duration",
    "DateTime",
    "UtcOffset",
    "OffsetDateTime",
]
