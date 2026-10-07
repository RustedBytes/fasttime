"""
fasttime - Fast UTC date/time library for Python, powered by Rust.
"""

from .fasttime import (
    Date as Date,
)
from .fasttime import (
    DateTime as DateTime,
)
from .fasttime import (
    Duration as Duration,
)
from .fasttime import (
    OffsetDateTime as OffsetDateTime,
)
from .fasttime import (
    Time as Time,
)
from .fasttime import (
    UtcOffset as UtcOffset,
)
from .fasttime import (
    Weekday as Weekday,
)

__all__ = [
    "Weekday",
    "Date",
    "Time",
    "Duration",
    "DateTime",
    "UtcOffset",
    "OffsetDateTime",
]
