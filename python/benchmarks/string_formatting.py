"""Compare installed release wheels in separate virtual environments.

Run with each environment's Python. Results are median nanoseconds per call,
including Python dispatch, over seven repetitions of 200,000 calls.
"""
import fasttime, timeit, statistics, json
values = {'date': fasttime.Date(2024,6,15), 'time': fasttime.Time(12,30,45,nanosecond=123456789), 'datetime': fasttime.DateTime.parse('2024-06-15T12:30:45.123456789Z'), 'offset': fasttime.OffsetDateTime.parse('2024-06-15T12:30:45.123456789+05:30'), 'duration': fasttime.Duration.nanoseconds(123456789), 'weekday':fasttime.Weekday.MONDAY}
print(json.dumps({f'{name}/{op.__name__}': round(statistics.median(timeit.repeat(lambda:op(value), number=200000, repeat=7))*1e9/200000,1) for name,value in values.items() for op in [str,repr]}))
